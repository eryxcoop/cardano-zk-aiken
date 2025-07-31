import {mConStr0} from "@meshsdk/core";
import {BlockchainProvider} from "../../blockchain_provider";
import {Contract} from "../../contract";

async function main() {
    const compiledContractPath = process.argv[2];
    const validatorScriptIndex = parseInt(process.argv[3]);
    const txHashFromDeposit = process.argv[4];
    const blockchain_provider = new BlockchainProvider();
    const wallet = blockchain_provider.getWalletUsing("me.sk");

    const contract = new Contract(compiledContractPath, blockchain_provider, wallet);
    const factors = mConStr0([5, 7]);
    await contract.spend(validatorScriptIndex, txHashFromDeposit, factors, {mem: 97246, steps: 4018447759})
}

main();
