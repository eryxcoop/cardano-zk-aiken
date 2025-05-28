import { getScript, getTxBuilder, getUtxoByTxHash, wallet } from "./two_prime_factors_utils";
import { Asset, deserializeAddress, mConStr0 } from "@meshsdk/core";



async function main() {
  // these are the assets we want to lock into the contract
  const assets: Asset[] = [
    {
      unit: "lovelace",
      quantity: "1000000",
    },
  ];
 
  // get utxo and wallet address
  const utxos = await wallet.getUtxos();
  const walletAddress = (await wallet.getUsedAddresses())[0];
 
  const { scriptAddr } = getScript();
  
  // build transaction with MeshTxBuilder
  const txBuilder = getTxBuilder();
  await txBuilder
    .txOut(scriptAddr, assets) // send assets to the script address
    //.txOutDatumHashValue(mConStr0([5*7])) // provide the datum where `"constructor": 0`
    .txOutInlineDatumValue(mConStr0([35]))
    .changeAddress(walletAddress) // send change back to the wallet address
    .selectUtxosFrom(utxos)
    .complete();
  const unsignedTx = txBuilder.txHex;
 
  const signedTx = await wallet.signTx(unsignedTx);
  const txHash = await wallet.submitTx(signedTx);
  console.log(`1 tADA locked into the contract at Tx ID: ${txHash}`);
}
 
main();