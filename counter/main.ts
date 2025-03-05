import ContractInterface from "./contract"

async function main() {
    const contract = new ContractInterface();

    let counterValue = 13;
    let txHash;
    txHash = await contract.deploy(counterValue, 1000000);
    ({txHash, nextCounterValue: counterValue} = await contract.next_step(counterValue, txHash));
    ({txHash, nextCounterValue: counterValue} = await contract.next_step(counterValue, txHash));
    ({txHash, nextCounterValue: counterValue} = await contract.next_step(counterValue, txHash));
}

main();