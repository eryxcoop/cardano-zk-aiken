import { Asset, deserializeAddress, mConStr0, stringToHex, Transaction, UTxO } from "@meshsdk/core";
import { getScript, getTxBuilder, getUtxoByTxHashAndAddress, wallet } from "./common";
import { sign } from "crypto";

export async function createOriginatingUTxO(amount) {
  const assets = [{ unit: "lovelace", quantity: amount.toString() }];
  const utxos = await wallet.getUtxos();
  const walletAddress = (await wallet.getUsedAddresses())[0];
  console.log(walletAddress)

  const txBuilder = getTxBuilder();
  await txBuilder
    .txOut(walletAddress, assets)
    .changeAddress(walletAddress)
    .selectUtxosFrom(utxos)
    .complete();
  const unsignedTx = await txBuilder.txHex;
  const signedTx = await wallet.signTx(unsignedTx);
  const txHash = await wallet.submitTx(signedTx);
  await waitForTransaction(txHash)
  return txHash;
}

async function waitForTransaction(txHash) {
  console.log(`Waiting for transaction ${txHash} to be confirmed...`);
  while (true) {
    const utxos = await wallet.getUtxos();
    if (utxos.some(utxo => utxo.input.txHash === txHash)) {
      console.log(`Transaction ${txHash} has been confirmed.`);
      break;
    }
    await new Promise(resolve => setTimeout(resolve, 5000));
  }
}

class ContractInterface {
  scriptAddr: string;
  scriptCbor: string;
  originatingUTxO: UTxO

  constructor(originatingUTxO: UTxO) {
    const {scriptAddr, scriptCbor} = getScript(originatingUTxO);
    this.scriptAddr = scriptAddr;
    this.scriptCbor = scriptCbor;
    this.originatingUTxO = originatingUTxO;
  }

  async deploy(amount) {
    let txHash;
    let attempts = 10;

    while (attempts > 0) {
         try {
           const assets = [{ unit: "lovelace", quantity: amount.toString() }];
           const walletAddress = (await wallet.getUsedAddresses())[0];
           const collateral = (await wallet.getCollateral())[0];
           console.log(collateral)

           const txBuilder = getTxBuilder();
           await txBuilder
             .mintPlutusScriptV3()
             .mint("1", this.scriptAddr, stringToHex("tuki"))
             .mintingScript(this.scriptCbor)
             .mintRedeemerValue(mConStr0(['mesh']))
             // .txOut(this.scriptAddr, assets)
             // .txOutInlineDatumValue(mConStr0([0]))
             .changeAddress(walletAddress)
             .selectUtxosFrom([this.originatingUTxO])
             .txInCollateral(
                this.originatingUTxO.input.txHash,
                this.originatingUTxO.input.outputIndex,
                [this.originatingUTxO.output.amount[0]],
                this.originatingUTxO.output.address
             )
             .complete();
     
           const unsignedTx = txBuilder.txHex;
           const signedTx = await wallet.signTx(unsignedTx);
           console.log(signedTx.toString())
           txHash = await wallet.submitTx(signedTx);
           console.log(`Locked ${amount} lovelace into the contract at Tx ID: ${txHash}`);
           break;
         } catch (error) {
           console.error("Transaction failed, retrying...", error);
           attempts--;
           await new Promise(resolve => setTimeout(resolve, 10000));
           if (attempts === 0) throw new Error("Failed to submit transaction after multiple attempts");
         }
    }
    await waitForTransaction(txHash);
    return txHash;
  }

  async next_step() {
  //  const nftUTxO = 
//     console.log("Making step from ", currentCounterValue, " to ", currentCounterValue + 1);
//     const utxos = await wallet.getUtxos();
//     const walletAddress = (await wallet.getUsedAddresses())[0];
//     const collateral = (await wallet.getCollateral())[0];
//     const scriptUtxo = await getUtxoByTxHashAndAddress(currentStateTxHash, this.scriptAddr);

//     const nextCounterValue = currentCounterValue + 1;
//     const txBuilder = getTxBuilder();
//     await txBuilder
//       .spendingPlutusScript("V3")
//       .txIn(
//         scriptUtxo.input.txHash,
//         scriptUtxo.input.outputIndex,
//         scriptUtxo.output.amount,
//         scriptUtxo.output.address
//       )
//       .txInScript(this.scriptCbor)
//       .txInRedeemerValue(mConStr0([""]))
//       .txInInlineDatumPresent()
//       .txOut(
//         scriptUtxo.output.address,
//         scriptUtxo.output.amount
//       )
//       .txOutInlineDatumValue(mConStr0([nextCounterValue]))
//       .changeAddress(walletAddress)
//       .txInCollateral(
//         collateral.input.txHash,
//         collateral.input.outputIndex,
//         collateral.output.amount,
//         collateral.output.address
//       )
//       .selectUtxosFrom(utxos)
//       .complete();
 
//     const unsignedTx = txBuilder.txHex;
//     const signedTx = await wallet.signTx(unsignedTx);
//     const txHash = await wallet.submitTx(signedTx);
 
//     await this.waitForTransaction(txHash);
//     console.log(`New counter value is: `, nextCounterValue);
//     return {nextCounterValue, txHash};
  }

}

export default ContractInterface;
