import { getUtxoByTxHash, wallet } from "./common";
import ContractInterface, { createOriginatingUTxO } from "./contract";

async function main() {
    console.log("creating initial utxo")
    const originatingUTxO = await getUtxoByTxHash(await createOriginatingUTxO(50 * 1000000));
    console.log("deploying")
    const contract = new ContractInterface(originatingUTxO);
    contract.deploy(10 * 1000000);
}

main();