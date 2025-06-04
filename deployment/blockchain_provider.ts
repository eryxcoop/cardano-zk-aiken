import { BlockfrostProvider, MeshTxBuilder, MeshWallet, UTxO } from "@meshsdk/core";
import { PathOrFileDescriptor, readFileSync } from "node:fs";

export class BlockchainProvider {
  blockchainProvider: BlockfrostProvider;
  
  constructor() {
    this.blockchainProvider = new BlockfrostProvider(process.env.BLOCKFROST_PROJECT_ID);
  }

  async isTxDeployed(txHash: string): Promise<boolean> {
    try {
      await this.blockchainProvider.fetchUTxOs(txHash);
      return true;
    } catch {
      return false;
    }
  }

  async getFirstUtxoFromTx(txHash: string): Promise<UTxO> {
    const utxos = await this.blockchainProvider.fetchUTxOs(txHash);
    if (utxos.length === 0) {
      throw new Error("UTxO not found");
    }
    return utxos[0];
  }

  getWalletUsing(walletKeyPath: PathOrFileDescriptor) {
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
