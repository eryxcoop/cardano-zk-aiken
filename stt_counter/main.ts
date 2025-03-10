import { getUtxoByTxHash, wallet } from "./common";
import ContractInterface, { createInitialUTxO } from "./contract";
import * as fs from "fs";

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

async function main() {
    console.log("creating initial utxo")
    const originatingUTxO = await getUtxoByTxHash(await createInitialUTxO(50 * 1000000));

    const contract = ContractInterface.fromInitialUTxO(originatingUTxO);
    contract.save("contract_interface.json");

    console.log("deploying")
    await contract.deploy(10 * 1000000);
    
    //const contract = ContractInterface.load("contract_interface.json");
    await contract.next_step();
    await contract.next_step();
    await contract.next_step();
}

main();