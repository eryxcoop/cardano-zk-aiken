import {
    deserializeAddress,
    mConStr0,
    stringToHex,
  } from "@meshsdk/core";
  import { getScript, getTxBuilder, getUtxoByTxHash, getUtxoByTxHashAndAddress, wallet } from "./common";
   
  async function main() {
    // get utxo, collateral and address from wallet
    const utxos = await wallet.getUtxos();
    const walletAddress = (await wallet.getUsedAddresses())[0];
    const collateral = (await wallet.getCollateral())[0];
   
    const { scriptCbor, scriptAddr} = getScript();
   
    // hash of the public key of the wallet, to be used in the datum
    const signerHash = deserializeAddress(walletAddress).pubKeyHash;
    // redeemer value to unlock the funds
    const message = 8; // ya está consumido
   
    // get the utxo from the script address of the locked funds
    const txHashFromDesposit = process.argv[2];
    const scriptUtxo = await getUtxoByTxHashAndAddress(txHashFromDesposit, scriptAddr);
   
    // build transaction with MeshTxBuilder
    const txBuilder = getTxBuilder();
    await txBuilder
      .spendingPlutusScript("V3") // we used plutus v3
      .txIn( // spend the utxo from the script address
        scriptUtxo.input.txHash,
        scriptUtxo.input.outputIndex,
        scriptUtxo.output.amount,
        scriptUtxo.output.address
      )
      .txInScript(scriptCbor)
      .txInRedeemerValue(mConStr0([message + 1])) // provide the required redeemer value `Hello, World!`
      //.txInDatumValue(mConStr0([message])) // only the owner of the wallet can unlock the funds
      .txInInlineDatumPresent()
      //.requiredSignerHash(signerHash)
      .txOut(
        scriptUtxo.output.address,
        scriptUtxo.output.amount
      )
      .txOutInlineDatumValue(mConStr0([message + 1]))
      //.txOutDatumHashValue(mConStr0([message + 1]))
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
    console.log(signedTx)
    const txHash = await wallet.submitTx(signedTx);
    console.log(`1 tADA unlocked from the contract at Tx ID: ${txHash}`);
  }
   
  main();

  