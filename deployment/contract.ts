import { readFileSync } from "node:fs";
import {
  MeshTxBuilder,
  MeshWallet,
  serializePlutusScript,
  Asset,
  deserializeAddress,
  Data,
  mConStr0,
  UTxO,
} from "@meshsdk/core";
import { applyParamsToScript } from "@meshsdk/core-csl";
import { BlockchainProvider } from "./blockchain_provider";

export const mVoid = () => mConStr0([]);

export class Contract {
  compiledValidatorScripts: any;
  blockchainProvider: BlockchainProvider;
  wallet: MeshWallet;

  constructor(compiledContractPath: string, blockchainProvider: BlockchainProvider, wallet: MeshWallet) {
    this.blockchainProvider = blockchainProvider;
    this.compiledValidatorScripts = this.loadContractFrom(compiledContractPath);
    this.wallet = wallet;
  }

  async deploy(validatorScriptIndex: number, datum: Data): Promise<string> {
    const txHash = await this.buildTxWithScriptUTxO(validatorScriptIndex, datum);
    const deployedTxHash = await this.deployTx(txHash);

    console.log(`1 tADA locked into the contract at Tx ID: ${deployedTxHash}`);
    
    return deployedTxHash;
  }

  async spend(validatorScriptIndex: number, txHashFromDeposit: string, redeemer: Data, redeemerBudget?: { mem: number, steps: number }): Promise<string> {
    const validatorScriptAddr = this.getValidatorScriptAddress(validatorScriptIndex)
    const scriptUtxo = await this.blockchainProvider.getUtxoByTxHashAndAddress(txHashFromDeposit, validatorScriptAddr);
    
    const collateralTryCount = 5;
    for (let i = 0; i < collateralTryCount; i++) {
      try {
        const txHash = await this.buildTxWithSpendUTxO(scriptUtxo, validatorScriptIndex, redeemer, redeemerBudget);
        const deployedTxHash = await this.deployTx(txHash);
      
        console.log(`1 tADA unlocked from the contract at Tx ID: ${deployedTxHash}`);

        return deployedTxHash;
      } catch (error) {
        if (error.search(/InsufficientCollateral/) === -1) {
          throw("Spend failed: "+ error);
        }
      }
    }

    throw(`Spend failed after ${collateralTryCount} times trying to get unspend collaterals`);
  }

  private async buildTxWithSpendUTxO(onchainScriptUtxo: UTxO, validatorScriptIndex: number, redeemer: Data, redeemerBudget?: { mem: number; steps: number; } | undefined) {
    const collateral = await this.getWalletCollateral();

    const txBuilder = this.blockchainProvider.newTxBuilder();
    txBuilder
      .spendingPlutusScript("V3")
      .txIn(
        onchainScriptUtxo.input.txHash,
        onchainScriptUtxo.input.outputIndex,
        onchainScriptUtxo.output.amount,
        onchainScriptUtxo.output.address
      )
      .txInScript(this.getValidatorScriptCbor(validatorScriptIndex))
      .txInInlineDatumPresent()
      .requiredSignerHash(await this.hashedWalletPublicKey())
      .changeAddress(await this.walletAddress())
      .txInCollateral(
        collateral.input.txHash,
        collateral.input.outputIndex,
        collateral.output.amount,
        collateral.output.address
      )
      .selectUtxosFrom(await this.wallet.getUtxos());

    if (redeemerBudget !== undefined) {
      txBuilder.txInRedeemerValue(redeemer, "Mesh", redeemerBudget);
    } else {
      txBuilder.txInRedeemerValue(redeemer);
    }
    await txBuilder.complete();
    return txBuilder.txHex;
  }

  private async getWalletCollateral(): Promise<UTxO> {
    return (await this.wallet.getCollateral())[0];
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

  private getValidatorScriptAddress(validatorScriptIndex: number) {
    const scriptCbor = this.getValidatorScriptCbor(validatorScriptIndex);

    const scriptAddr = serializePlutusScript(
      { code: scriptCbor, version: "V3" }
    ).address;
    return scriptAddr;
  }

  private getValidatorScriptCbor(validatorScriptIndex: number) {
    return applyParamsToScript(
      this.compiledValidatorScripts.validators[validatorScriptIndex].compiledCode,
      []
    );
  }

  private oneADA(): Asset[] {
    return [
      {
        unit: "lovelace",
        quantity: "1000000",
      },
    ];
  }

  private async buildTxWithScriptUTxO(validatorScriptIndex: number, datum: Data): Promise<string> {
    const txBuilder = this.blockchainProvider.newTxBuilder();
    await txBuilder
      .txOut(this.getValidatorScriptAddress(validatorScriptIndex), this.oneADA())
      .txOutInlineDatumValue(datum)
      .changeAddress(await this.walletAddress())
      .selectUtxosFrom(await this.wallet.getUtxos())
      .complete();
    return txBuilder.txHex;
  }

  private async deployTx(unsignedTx: string): Promise<string> {
    const signedTx = await this.wallet.signTx(unsignedTx);
    return this.wallet.submitTx(signedTx);
  }
}