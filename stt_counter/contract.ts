import { resolveScriptHash, stringToHex, UTxO } from "@meshsdk/core";
import { getScript, getTxBuilder, getUTxOsByAddress, wallet } from "./common";
import * as fs from "fs";

export async function createInitialUTxO(amount) {
  const assets = [{ unit: "lovelace", quantity: amount.toString() }];
  const utxos = await wallet.getUtxos();
  const walletAddress = (await wallet.getUsedAddresses())[0];
  console.log(walletAddress);

  const txBuilder = getTxBuilder();
  await txBuilder
    .txOut(walletAddress, assets)
    .changeAddress(walletAddress)
    .selectUtxosFrom(utxos)
    .complete();
  const unsignedTx = await txBuilder.txHex;
  const signedTx = await wallet.signTx(unsignedTx);
  const txHash = await wallet.submitTx(signedTx);
  if (txHash !== undefined) {
    await waitForTransaction(txHash);
  }
  return txHash;
}

async function waitForTransaction(txHash) {
  console.log(`Waiting for transaction ${txHash} to be confirmed...`);
  while (true) {
    const utxos = await wallet.getUtxos();
    if (utxos.some((utxo) => utxo.input.txHash === txHash)) {
      console.log(`Transaction ${txHash} has been confirmed.`);
      break;
    }
    await new Promise((resolve) => setTimeout(resolve, 5000));
  }
}

class ContractInterface {
  constructor(
    public scriptAddr: string,
    public scriptCbor: string,
    public initialUTxO: UTxO,
    public stateNFTPolicyID: string,
    public stateNFTNameInHex: string
  ) {}

  save(fileName: string) {
    const json = JSON.stringify(this, null, 2);
    fs.writeFileSync("contract_interface.json", json, "utf-8");
  }

  static load(filePath: string): ContractInterface {
    const json = fs.readFileSync(filePath, "utf-8");
    const data = JSON.parse(json);
    return new ContractInterface(
      data.scriptAddr,
      data.scriptCbor,
      data.initialUTxO,
      data.stateNFTPolicyID,
      data.stateNFTNameInHex
    );
  }

  static fromInitialUTxO(initialUTxO: UTxO) {
    const { scriptAddr, scriptCbor } = getScript(initialUTxO);
    return new ContractInterface(
      scriptAddr,
      scriptCbor,
      initialUTxO,
      resolveScriptHash(scriptCbor, "V3"),
      stringToHex("StateNFT")
    );
  }

  contractStateNFT() {
    return this.stateNFTPolicyID + this.stateNFTNameInHex;
  }

  async deploy() {
    let txHash;
    let attempts = 10;

    while (attempts > 0) {
      try {
        const assets = [{ unit: this.contractStateNFT(), quantity: "1" }];
        const walletAddress = (await wallet.getUsedAddresses())[0];
        const collateral = (await wallet.getCollateral())[0];

        const txBuilder = getTxBuilder();
        await txBuilder
          .mintPlutusScriptV3()
          .mint("1", this.stateNFTPolicyID, this.stateNFTNameInHex)
          .mintingScript(this.scriptCbor)
          .mintRedeemerValue("")
          .txOut(this.scriptAddr, assets)
          .txOutInlineDatumValue(0)
          .changeAddress(walletAddress)
          .selectUtxosFrom([this.initialUTxO])
          .txInCollateral(
            collateral.input.txHash,
            collateral.input.outputIndex,
            collateral.output.amount,
            collateral.output.address
          )
          .complete();

        const unsignedTx = txBuilder.txHex;
        const signedTx = await wallet.signTx(unsignedTx);
        txHash = await wallet.submitTx(signedTx);
        console.log(`Contract deployed at Tx ID: ${txHash}`);
        break;
      } catch (error) {
        console.error("Transaction failed, retrying...", error);
        attempts--;
        await new Promise((resolve) => setTimeout(resolve, 10000));
        if (attempts === 0)
          throw new Error(
            "Failed to submit transaction after multiple attempts"
          );
      }
    }
    if (txHash !== undefined) {
      await waitForTransaction(txHash);
    }
    return txHash;
  }

  async getCurrentStateUTxO(): Promise<UTxO> {
    const utxos = await getUTxOsByAddress(this.scriptAddr);
    const utxo = utxos.find((utxo) =>
      utxo.output.amount.some((asset) => asset.unit == this.contractStateNFT())
    );
    if (utxo === undefined) {
      throw new Error("No UTxO with StateNFT was found.");
    }

    return utxo;
  }

  async next_step() {
    let txHash;
    let attempts = 10;

    while (attempts > 0) {
      try {
        const utxos = await wallet.getUtxos();
        const currentStateUTxO = await this.getCurrentStateUTxO();
        const currentCounter = parseInt(
          currentStateUTxO.output.plutusData ?? "0",
          16
        );
        console.log(currentStateUTxO);
        const assets = [{ unit: this.contractStateNFT(), quantity: "1" }];
        const walletAddress = (await wallet.getUsedAddresses())[0];
        const collateral = (await wallet.getCollateral())[0];

        const txBuilder = getTxBuilder();
        await txBuilder
          .spendingPlutusScript("V3")
          .txIn(
            currentStateUTxO.input.txHash,
            currentStateUTxO.input.outputIndex,
            currentStateUTxO.output.amount,
            currentStateUTxO.output.address
          )
          .txInScript(this.scriptCbor)
          .txInRedeemerValue("")
          .txInInlineDatumPresent()
          .txOut(this.scriptAddr, assets)
          .txOutInlineDatumValue(currentCounter + 1)
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
        txHash = await wallet.submitTx(signedTx);
        break;
      } catch (error) {
        console.error("Transaction failed, retrying...");
        attempts--;
        await new Promise((resolve) => setTimeout(resolve, 10000));
        if (attempts === 0)
          throw new Error(
            "Failed to submit transaction after multiple attempts"
          );
      }
    }
    if (txHash !== undefined) {
      await waitForTransaction(txHash);
    }
  }
}

export default ContractInterface;
