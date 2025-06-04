import { readFileSync } from "node:fs";
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

export const blockchainProvider = new BlockfrostProvider(process.env.BLOCKFROST_PROJECT_ID);

// wallet for signing transactions
export const wallet = new MeshWallet({
  networkId: 0,
  fetcher: blockchainProvider,
  submitter: blockchainProvider,
  key: {
    type: "root",
    bech32: readFileSync("me.sk").toString(),
  },
});
 
 
// reusable function to get a transaction builder
export function getTxBuilder() {
  return new MeshTxBuilder({
    fetcher: blockchainProvider,
    submitter: blockchainProvider,
  });
}
 
// reusable function to get a UTxO by transaction hash
export async function getUtxoByTxHash(txHash: string): Promise<UTxO> {
  const utxos = await blockchainProvider.fetchUTxOs(txHash);
  if (utxos.length === 0) {
    throw new Error("UTxO not found");
  }
  return utxos[0];
}

export class Contract {
  compiledContractPath: string;

  constructor(compiledContractPath: string) {
    this.compiledContractPath = compiledContractPath;
  }

  async deploy(validatorIndex: number, datum: Data): Promise<string> {
    const assets: Asset[] = [
      {
        unit: "lovelace",
        quantity: "10000000",
      },
    ];

    // get utxo and wallet address
    const utxos = await wallet.getUtxos();
    const walletAddress = (await wallet.getUsedAddresses())[0];

    const { scriptAddr } = this.getScript(validatorIndex, this.compiledContractPath);

    // build transaction with MeshTxBuilder
    const txBuilder = getTxBuilder();
    await txBuilder
      .txOut(scriptAddr, assets) // send assets to the script address
      .txOutInlineDatumValue(datum)
      .changeAddress(walletAddress) // send change back to the wallet address
      .selectUtxosFrom(utxos)
      .complete();
    const unsignedTx = txBuilder.txHex;

    const signedTx = await wallet.signTx(unsignedTx);
    const txHashPromise = wallet.submitTx(signedTx);

    txHashPromise.then(
      (txHash) => {
        console.log(`1 tADA locked into the contract at Tx ID: ${txHash}`);
      }
    );
    
    return txHashPromise;
  }

  async spend(validatorIndex: number, txHashFromDeposit: string, redeemer: Data): Promise<string> {
    const utxos = await wallet.getUtxos();
    const walletAddress = (await wallet.getUsedAddresses())[0];
    const collateral = (await wallet.getCollateral())[0];

    const { scriptCbor } = this.getScript(validatorIndex, this.compiledContractPath);

    // hash of the public key of the wallet, to be used in the datum
    const signerHash = deserializeAddress(walletAddress).pubKeyHash;

    // get the utxo from the script address of the locked funds
    const scriptUtxo = await getUtxoByTxHash(txHashFromDeposit);

    const budget = { mem: 7489, steps: 2303111 };
    // build transaction with MeshTxBuilder
    const txBuilder = getTxBuilder();
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
    const signedTx = await wallet.signTx(unsignedTx);
    const txHashPromise = wallet.submitTx(signedTx);

    txHashPromise.then((txHash) => {
      console.log(`1 tADA unlocked from the contract at Tx ID: ${txHash}`);
    });

    return txHashPromise;
  }

  private getScript(validatorIndex: number, plutusJson: string) {
    const jsonString = readFileSync(plutusJson, 'utf-8');
    const blueprint = JSON.parse(jsonString);

    const scriptCbor = applyParamsToScript(
      blueprint.validators[validatorIndex].compiledCode,
      []
    );
  
    const scriptAddr = serializePlutusScript(
      { code: scriptCbor, version: "V3" },
    ).address;
  
    return { scriptCbor, scriptAddr };
  }
}