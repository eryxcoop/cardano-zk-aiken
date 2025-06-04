import { readFileSync } from "node:fs";
import {
  MeshTxBuilder,
  MeshWallet,
  serializePlutusScript,
  Asset,
  deserializeAddress,
  Data,
  mConStr0,
} from "@meshsdk/core";

import { applyParamsToScript } from "@meshsdk/core-csl";
import { BlockchainProvider } from "./blockchain_provider";

export const mVoid = () => mConStr0([]);

export class Contract {
  compiledValidators: any;
  blockchainProvider: BlockchainProvider;
  wallet: MeshWallet;
  txBuilder: MeshTxBuilder;

  constructor(compiledContractPath: string) {
    this.blockchainProvider = new BlockchainProvider();
    this.compiledValidators = this.loadContractFrom(compiledContractPath);
    this.txBuilder = this.blockchainProvider.newTxBuilder();
    this.wallet = this.blockchainProvider.getWalletUsing("me.sk");
  }

  async deploy(validatorIndex: number, datum: Data): Promise<string> {
    const txHash = await this.buildTxWithScriptUTxO(validatorIndex, datum);
    const deployedTxHashPromise = this.deployTx(txHash);

    deployedTxHashPromise.then(
      (txHash) => {
        console.log(`1 tADA locked into the contract at Tx ID: ${txHash}`);
      }
    );
    
    return deployedTxHashPromise;
  }

  async spend(validatorIndex: number, txHashFromDeposit: string, redeemer: Data, redeemerBudget?: { mem: number, steps: number }): Promise<string> {
    const walletAddress = await this.walletAddress();

    const collateral = (await this.wallet.getCollateral())[0];
    // get the utxo from the script address of the locked funds
    const scriptUtxo = await this.blockchainProvider.getFirstUtxoFromTx(txHashFromDeposit);

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
      .txInScript(this.getValidatorCbor(validatorIndex))
      .txInRedeemerValue(redeemer)
      //.txInDatumValue(mConStr0([signerHash])) // only the owner of the wallet can unlock the funds
      .txInInlineDatumPresent()
      .requiredSignerHash(await this.hashedWalletPublicKey())
      .changeAddress(walletAddress)
      .txInCollateral(
        collateral.input.txHash,
        collateral.input.outputIndex,
        collateral.output.amount,
        collateral.output.address
      )
      .selectUtxosFrom(await this.wallet.getUtxos())
      .complete();

    const deployedTxHashPromise = this.deployTx(txBuilder.txHex);

    deployedTxHashPromise.then((txHash) => {
      console.log(`1 tADA unlocked from the contract at Tx ID: ${txHash}`);
    });

    return deployedTxHashPromise;
  }

  private loadContractFrom(compiledContractPath: string) {
    const jsonString = readFileSync(compiledContractPath, 'utf-8');
    return JSON.parse(jsonString);
  }

  private async walletAddress() {
    return (await this.wallet.getUsedAddresses())[0];
  }

  private async hashedWalletPublicKey() {
    return deserializeAddress(await this.walletAddress()).pubKeyHash;
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
      this.compiledValidators.validators[validatorIndex].compiledCode,
      []
    );
  }

  private oneADA(): Asset[] {
    return [
      {
        unit: "lovelace",
        quantity: "10000000",
      },
    ];
  }

  private async buildTxWithScriptUTxO(validatorIndex: number, datum: Data): Promise<string> {
    const txBuilder = this.blockchainProvider.newTxBuilder();
    await txBuilder
      .txOut(this.getValidatorAddress(validatorIndex), this.oneADA()) // send assets to the script address
      .txOutInlineDatumValue(datum)
      .changeAddress(await this.walletAddress()) // send change back to the wallet address
      .selectUtxosFrom(await this.wallet.getUtxos())
      .complete();
    return txBuilder.txHex;
  }

  private async deployTx(unsignedTx: string): Promise<string> {
    const signedTx = await this.wallet.signTx(unsignedTx);
    return this.wallet.submitTx(signedTx);
  }
}