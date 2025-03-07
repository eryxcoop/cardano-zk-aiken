import { getUtxoByTxHash, wallet } from "./common";
import ContractInterface, { createOriginatingUTxO } from "./contract";

async function main() {
    const utxo = await wallet.getUtxos();
    console.log("creating initial utxo")
    const originatingUTxO = await getUtxoByTxHash(await createOriginatingUTxO(10 * 1000000));
    console.log("deploying")
    const contract = new ContractInterface(originatingUTxO);
    contract.deploy(1 * 1000000);
}

main();