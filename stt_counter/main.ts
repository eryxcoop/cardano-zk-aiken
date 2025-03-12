import { getUtxoByTxHash, wallet } from "./common";
import ContractInterface, { createInitialUTxO } from "./contract";

async function main() {
    console.log("creating initial utxo")
    const originatingUTxO = await getUtxoByTxHash(await createInitialUTxO(50 * 1000000));

    const contract = ContractInterface.fromInitialUTxO(originatingUTxO);
    contract.save("contract_interface.json");

    console.log("deploying")
    await contract.deploy();
    
    //const contract = ContractInterface.load("contract_interface.json");
    await contract.next_step();
    await contract.next_step();
    await contract.next_step();
}

main();