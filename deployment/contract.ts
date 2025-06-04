import { PathOrFileDescriptor, readFileSync } from "node:fs";
import {
  BlockfrostProvider,
  MeshTxBuilder,
  MeshWallet,
  serializePlutusScript,
  UTxO,
  Asset,
  deserializeAddress,
  MConStr,
  MOption,
  Data,
  mConStr0,
} from "@meshsdk/core";

import { applyParamsToScript } from "@meshsdk/core-csl";

export const mVoid = () => mConStr0([]);

export class BlockchainProvider {
  blockchainProvider: BlockfrostProvider;
  
  constructor() {
    this.blockchainProvider = new BlockfrostProvider(process.env.BLOCKFROST_PROJECT_ID);
  }

  async isTxHashDeployed(txHash: string): Promise<UTxO[]> {
    return this.blockchainProvider.fetchUTxOs(txHash);
  }

  async getUtxoByTxHash(txHash: string): Promise<UTxO> {
    const utxos = await this.blockchainProvider.fetchUTxOs(txHash);
    if (utxos.length === 0) {
      throw new Error("UTxO not found");
    }
    return utxos[0];
  }

  getWallet(walletKeyPath: PathOrFileDescriptor) {
    return new MeshWallet({
      networkId: 0,
      fetcher: this.blockchainProvider,
      submitter: this.blockchainProvider,
      key: {
        type: "root",
        bech32: readFileSync(walletKeyPath).toString(),
      },
    });
  }

  newTxBuilder() {
    return new MeshTxBuilder({
      fetcher: this.blockchainProvider,
      submitter: this.blockchainProvider,
    });
  }
}

 


export class Contract {
  compiledScript: any;
  blockchainProvider: BlockchainProvider;
  wallet: MeshWallet;
  txBuilder: MeshTxBuilder;

  constructor(compiledContractPath: string) {
    this.blockchainProvider = new BlockchainProvider();
    this.compiledScript = this.loadScriptFrom(compiledContractPath);
    this.txBuilder = this.blockchainProvider.newTxBuilder();
    this.wallet = this.blockchainProvider.getWallet("me.sk");
  }

  async deploy(validatorIndex: number, datum: Data): Promise<string> {
    const txBuilder = this.blockchainProvider.newTxBuilder();
    await txBuilder
      .txOut(this.getValidatorAddress(validatorIndex), this.oneADA()) // send assets to the script address
      .txOutInlineDatumValue(datum)
      .changeAddress(await this.walletAddress()) // send change back to the wallet address
      .selectUtxosFrom(await this.wallet.getUtxos())
      .complete();

    const txHashPromise = this.deployTx(txBuilder.txHex);

    txHashPromise.then(
      (txHash) => {
        console.log(`1 tADA locked into the contract at Tx ID: ${txHash}`);
      }
    );
    
    return txHashPromise;
  }

  async spend(validatorIndex: number, txHashFromDeposit: string, redeemer: Data): Promise<string> {
    const utxos = await this.wallet.getUtxos();
    const walletAddress = (await this.wallet.getUsedAddresses())[0];
    const collateral = (await this.wallet.getCollateral())[0];

    const scriptCbor = this.getValidatorCbor(validatorIndex);

    // hash of the public key of the wallet, to be used in the datum
    const signerHash = deserializeAddress(walletAddress).pubKeyHash;

    // get the utxo from the script address of the locked funds
    const scriptUtxo = await this.blockchainProvider.getUtxoByTxHash(txHashFromDeposit);

    const budget = { mem: 7489, steps: 2303111 };
    // build transaction with MeshTxBuilder
    const txBuilder = this.blockchainProvider.newTxBuilder();
    await txBuilder
      .spendingPlutusScript("V3") // we used plutus v3
      .txIn(
        scriptUtxo.input.txHash,
        scriptUtxo.input.outputIndex,
        scriptUtxo.output.amount,
        scriptUtxo.output.address
      )
      .txInScript(scriptCbor)
      .txInRedeemerValue(redeemer)
      //.txInDatumValue(mConStr0([signerHash])) // only the owner of the wallet can unlock the funds
      .txInInlineDatumPresent()
      .requiredSignerHash(signerHash)
      .changeAddress(walletAddress)
      .txInCollateral(
        collateral.input.txHash,
        collateral.input.outputIndex,
        collateral.output.amount,
        collateral.output.address
      )
      .selectUtxosFrom(utxos)
      .complete();

    const unsignedTx = txBuilder.txHex;
    // const evaluatedTx = await blockchainProvider.evaluateTx(unsignedTx);
    // console.log(evaluatedTx)
    // maxTxExMem: '16000000',
    // maxTxExSteps: '10000000000',
    const signedTx = await this.wallet.signTx(unsignedTx);
    const txHashPromise = this.wallet.submitTx(signedTx);

    txHashPromise.then((txHash) => {
      console.log(`1 tADA unlocked from the contract at Tx ID: ${txHash}`);
    });

    return txHashPromise;
  }

  private loadScriptFrom(compiledContractPath: string) {
    const jsonString = readFileSync(compiledContractPath, 'utf-8');
    return JSON.parse(jsonString);
  }


  private oneADA(): Asset[] {
    return [
      {
        unit: "lovelace",
        quantity: "10000000",
      },
    ];
  }

  private async deployTx(unsignedTx: string): Promise<string> {
    const signedTx = await this.wallet.signTx(unsignedTx);
    return this.wallet.submitTx(signedTx);
  }

  private async walletAddress() {
    return (await this.wallet.getUsedAddresses())[0];
  }

  private getValidatorAddress(validatorIndex: number) {
    const scriptCbor = this.getValidatorCbor(validatorIndex);

    const scriptAddr = serializePlutusScript(
      { code: scriptCbor, version: "V3" }
    ).address;
    return scriptAddr;
  }

  private getValidatorCbor(validatorIndex: number) {
    return applyParamsToScript(
      this.compiledScript.validators[validatorIndex].compiledCode,
      []
    );
  }
}