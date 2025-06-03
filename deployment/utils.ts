import { readFileSync } from "node:fs";
import {
  BlockfrostProvider,
  MeshTxBuilder,
  MeshWallet,
  serializePlutusScript,
  UTxO,
  mConStr0,
  Asset,
  deserializeAddress,
} from "@meshsdk/core";

import { applyParamsToScript } from "@meshsdk/core-csl";
import { resolve } from "node:path";

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
 
export function getScript(validatorIndex: number, plutusJson: string) {
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

  async deployWithDatum(validatorIndex: number): Promise<string> {
    return this.deploy(true, validatorIndex)
  }

  async deployWithoutDatum(validatorIndex: number): Promise<string> {
    return this.deploy(false, validatorIndex)
  }

  async spend(validatorIndex: number, txHashFromDeposit: string) {
  
    const utxos = await wallet.getUtxos();
    const walletAddress = (await wallet.getUsedAddresses())[0];
    const collateral = (await wallet.getCollateral())[0];

    const redeemer = { alternative: 0, fields: [7, 5] };

    const { scriptCbor } = getScript(validatorIndex, this.compiledContractPath);

    // hash of the public key of the wallet, to be used in the datum
    const signerHash = deserializeAddress(walletAddress).pubKeyHash;

    // get the utxo from the script address of the locked funds
    const scriptUtxo = await getUtxoByTxHash(txHashFromDeposit);

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
      .txInRedeemerValue(mConStr0([7, 5]), "Mesh", { mem: 98242, steps: 4018841489 }) // provide the required redeemer value `Hello, World!`

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
    const txHash = await wallet.submitTx(signedTx);
    console.log(`1 tADA unlocked from the contract at Tx ID: ${txHash}`);
  }

  private async deploy(hasDatum: boolean, validatorIndex: number): Promise<string> {
    const assets: Asset[] = [
      {
        unit: "lovelace",
        quantity: "10000000",
      },
    ];

    // get utxo and wallet address
    const utxos = await wallet.getUtxos();
    const walletAddress = (await wallet.getUsedAddresses())[0];

    const { scriptAddr } = getScript(validatorIndex, this.compiledContractPath);

    // build transaction with MeshTxBuilder
    const txBuilder = getTxBuilder();
    txBuilder.txOut(scriptAddr, assets); // send assets to the script address

    if (hasDatum) {
      //.txOutDatumHashValue(mConStr0([5*7])) // provide the datum where `"constructor": 0`
      txBuilder.txOutInlineDatumValue(mConStr0([35]));
    }

    await txBuilder
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
}