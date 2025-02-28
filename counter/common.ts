import fs from "node:fs";
import {
  BlockfrostProvider,
  MeshTxBuilder,
  MeshWallet,
  serializePlutusScript,
  UTxO
} from "@meshsdk/core";
import { applyParamsToScript } from "@meshsdk/core-csl"
import blueprint from "./plutus.json";

const blockchainProvider = new BlockfrostProvider(process.env.BLOCKFROST_PROJECT_ID ?? "");
 
// wallet for signing transactions
export const wallet = new MeshWallet({
  networkId: 0,
  fetcher: blockchainProvider,
  submitter: blockchainProvider,
  key: {
    type: "root",
    bech32: fs.readFileSync("me.sk").toString(),
  },
});

export function getScript() {
    const scriptCbor = applyParamsToScript(
      blueprint.validators[0].compiledCode,
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
   
    export async function getUtxoByTxHashAndAddress(txHash: string, address: string): Promise<UTxO> {
      // Fetch the UTxOs related to the provided transaction hash
      const utxos = await blockchainProvider.fetchUTxOs(txHash);
    
      // If no UTxOs are found, throw an error
      if (utxos.length === 0) {
        throw new Error("UTxO not found");
      }
    
      // Filter the UTxOs to find the one matching the given address
      const matchingUtxos = utxos.filter((utxo) => utxo.output.address === address);
    
      // If no UTxO matches the address, throw an error
      if (matchingUtxos.length === 0) {
        throw new Error(`UTxO not found for address: ${address}`);
      }
    
      // If more than one UTxO matches the address, throw an error
      if (matchingUtxos.length > 1) {
        throw new Error(`Multiple UTxOs found for address: ${address}`);
      }
    
      // Return the matching UTxO
      return matchingUtxos[0];
    }
    

  // reusable function to get a UTxO by transaction hash
export async function getUtxoByTxHash(txHash: string): Promise<UTxO> {
    const utxos = await blockchainProvider.fetchUTxOs(txHash);
    if (utxos.length === 0) {
      throw new Error("UTxO not found");
    }
    return utxos[0];
    }