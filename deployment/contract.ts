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
    const validatorAddr = this.getValidatorAddress(validatorIndex)
    const scriptUtxo = await this.blockchainProvider.getUtxoByTxHashAndAddress(txHashFromDeposit, validatorAddr);

    const txHash = await this.buildTxWithSpendUTxO(scriptUtxo, validatorIndex, redeemer, redeemerBudget);
    const deployedTxHashPromise = this.deployTx(txHash);

    deployedTxHashPromise.then((txHash) => {
      console.log(`1 tADA unlocked from the contract at Tx ID: ${txHash}`);
    });

    return deployedTxHashPromise;
  }

  private async buildTxWithSpendUTxO(onchainScriptUtxo: UTxO, validatorIndex: number, redeemer: Data, redeemerBudget?: { mem: number; steps: number; } | undefined) {
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
      .txInScript(this.getValidatorCbor(validatorIndex))
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

  async getWalletCollateral(): Promise<UTxO> {
    const collaterals = await this.wallet.getCollateral();
    console.log(collaterals);
    console.log(collaterals[0]["output"].amount);
    return (collaterals)[0];
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
        quantity: "1000000",
      },
    ];
  }

  private async buildTxWithScriptUTxO(validatorIndex: number, datum: Data): Promise<string> {
    const txBuilder = this.blockchainProvider.newTxBuilder();
    await txBuilder
      .txOut(this.getValidatorAddress(validatorIndex), this.oneADA())
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