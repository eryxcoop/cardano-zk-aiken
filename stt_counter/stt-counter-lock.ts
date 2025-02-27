import { 
  Asset,
  deserializeAddress, 
  mConStr0, 
  resolveScriptHash,
  stringToHex,
  AssetMetadata,
} from "@meshsdk/core";
import { getScript, getTxBuilder, wallet } from "./common";
 
async function main() {
  // these are the assets we want to lock into the contract
  const assets: Asset[] = [
    {
      unit: "lovelace",
      quantity: "1000000",
    },
  ];

  const collateral = (await wallet.getCollateral())[0];


  const demoAssetMetadata = {
    name: "STT",
    image: "https://fcb-abj-pre.s3.amazonaws.com/img/jugadors/MESSI.jpg",
    mediaType: "image/jpg",
    description: "example"
  }
 
  // get utxo and wallet address
  const utxos = await wallet.getUtxos();
  const walletAddress = (await wallet.getUsedAddresses())[0];
 
  const { scriptAddr, scriptCbor, compiledCode} = getScript();
  
  console.log(scriptAddr)
  console.log(scriptCbor)

  const tokenName = "STT";

  const tokenNameHex = stringToHex(tokenName)

  const policyId = resolveScriptHash(scriptCbor, "V3")

  const metadata = { [policyId]: { [tokenName]: demoAssetMetadata}}

  //console.log(metadata)

  const datum = mConStr0([7])
 
  // build transaction with MeshTxBuilder
  const txBuilder = getTxBuilder();
  await txBuilder
    .mint("1", policyId, tokenNameHex)
    .mintingScript(scriptCbor)
    .metadataValue(721, metadata)
    .changeAddress(walletAddress)
    .selectUtxosFrom(utxos)
    //.txOut(scriptAddr, assets) // send assets to the script address
    //.txOutInlineDatumValue(datum)
    .complete();
  const unsignedTx = txBuilder.txHex;
 
  const signedTx = await wallet.signTx(unsignedTx);
  console.log(signedTx)
  const txHash = await wallet.submitTx(signedTx);
  console.log(`1 tADA locked into the contract at Tx ID: ${txHash}`);
}
 
main();