import { getUtxoByTxHash, wallet } from "./common";
import ContractInterface, { createInitialUTxO } from "./contract";

async function main() {
    console.log("creating initial utxo")
    const originatingUTxO = await getUtxoByTxHash(await createInitialUTxO(50 * 1000000));

    const contract = ContractInterface.fromInitialUTxO(originatingUTxO);

    console.log("deploying")
    contract.deploy(10 * 1000000);
}

main();