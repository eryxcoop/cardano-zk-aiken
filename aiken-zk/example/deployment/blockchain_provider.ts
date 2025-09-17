import { BlockfrostProvider, MeshTxBuilder, MeshWallet, UTxO } from "@meshsdk/core";
import { PathOrFileDescriptor, readFileSync } from "node:fs";

export class BlockchainProvider {
  blockfrostProvider: BlockfrostProvider;
  
  constructor() {
    this.blockfrostProvider = new BlockfrostProvider(process.env.BLOCKFROST_PROJECT_ID);
  }

  async hasTxBeenImpactedOnBlockchain(txHash: string): Promise<boolean> {
    try {
      const fetchedUTxOS = await this.blockfrostProvider.fetchUTxOs(txHash);
      return true;
    } catch {
      return false;
    }
  }

  async getFirstUtxoFromTx(txHash: string): Promise<UTxO> {
    const utxos = await this.blockfrostProvider.fetchUTxOs(txHash);
    if (utxos.length === 0) {
      throw new Error("UTxO not found");
    }
    return utxos[0];
  }

  async getUtxoByTxHashAndAddress(txHash: string, address: string): Promise<UTxO> {
    const utxos = await this.blockfrostProvider.fetchUTxOs(txHash);
  
    if (utxos.length === 0) {
      throw new Error("UTxO not found");
    }
  
    const matchingUtxos = utxos.filter((utxo) => utxo.output.address === address);
  
    if (matchingUtxos.length === 0 || matchingUtxos.length > 1) {
      throw new Error(`UTxO not found for address: ${address}`);
    }
  
    return matchingUtxos[0];
  }

  getWalletUsing(walletKeyPath: PathOrFileDescriptor) {
    return new MeshWallet({
      networkId: 0,
      fetcher: this.blockfrostProvider,
      submitter: this.blockfrostProvider,
      key: {
        type: "root",
        bech32: readFileSync(walletKeyPath).toString(),
      },
    });
  }

  newTxBuilder() {
    return new MeshTxBuilder({
      fetcher: this.blockfrostProvider,
      submitter: this.blockfrostProvider,
    });
  }
}
