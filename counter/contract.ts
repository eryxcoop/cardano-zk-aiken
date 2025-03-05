import { Asset, deserializeAddress, mConStr0 } from "@meshsdk/core";
import { getScript, getTxBuilder, getUtxoByTxHashAndAddress, wallet } from "./common";

class ContractInterface {
  scriptAddr: string;
  scriptCbor: string;

  constructor() {
    const {scriptAddr, scriptCbor} = getScript();
    this.scriptAddr = scriptAddr;
    this.scriptCbor = scriptCbor;
  }

  async deploy(counterInit, amount) {
    let txHash;
    let attempts = 10;

    while (attempts > 0) {
        try {
          const assets = [{ unit: "lovelace", quantity: amount.toString() }];
          const utxos = await wallet.getUtxos();
          const walletAddress = (await wallet.getUsedAddresses())[0];

          const txBuilder = getTxBuilder();
          await txBuilder
            .txOut(this.scriptAddr, assets)
            .txOutInlineDatumValue(mConStr0([counterInit]))
            .changeAddress(walletAddress)
            .selectUtxosFrom(utxos)
            .complete();
          
          const unsignedTx = txBuilder.txHex;
          const signedTx = await wallet.signTx(unsignedTx);
          txHash = await wallet.submitTx(signedTx);
          console.log(`Locked ${amount} lovelace into the contract at Tx ID: ${txHash}`);
          break;
        } catch (error) {
          console.error("Transaction failed, retrying...");
          attempts--;
          await new Promise(resolve => setTimeout(resolve, 10000));
          if (attempts === 0) throw new Error("Failed to submit transaction after multiple attempts");
        }
    }
    await this.waitForTransaction(txHash);
    return txHash;
  }

  async next_step(currentCounterValue, currentStateTxHash) {
    console.log("Making step from ", currentCounterValue, " to ", currentCounterValue + 1);
    const utxos = await wallet.getUtxos();
    const walletAddress = (await wallet.getUsedAddresses())[0];
    const collateral = (await wallet.getCollateral())[0];
    const scriptUtxo = await getUtxoByTxHashAndAddress(currentStateTxHash, this.scriptAddr);

    const nextCounterValue = currentCounterValue + 1;
    const txBuilder = getTxBuilder();
    await txBuilder
      .spendingPlutusScript("V3")
      .txIn(
        scriptUtxo.input.txHash,
        scriptUtxo.input.outputIndex,
        scriptUtxo.output.amount,
        scriptUtxo.output.address
      )
      .txInScript(this.scriptCbor)
      .txInRedeemerValue(mConStr0([""]))
      .txInInlineDatumPresent()
      .txOut(
        scriptUtxo.output.address,
        scriptUtxo.output.amount
      )
      .txOutInlineDatumValue(mConStr0([nextCounterValue]))
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
    const signedTx = await wallet.signTx(unsignedTx);
    const txHash = await wallet.submitTx(signedTx);
    
    await this.waitForTransaction(txHash);
    console.log(`New counter value is: `, nextCounterValue);
    return {nextCounterValue, txHash};
  }

  async waitForTransaction(txHash) {
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
}

export default ContractInterface;