import {mConStr0} from "@meshsdk/core";
import {BlockchainProvider} from "./blockchain_provider";
import {Contract, mVoid} from "./contract";
import {mZKRedeemer} from "./zk_redeemer";

async function main() {
    const compiledContractPath = "../plutus.json";
    const validatorScriptIndex = 0 ;
    const txHashFromDeposit = process.argv[2];
    const blockchain_provider = new BlockchainProvider();
    const wallet = blockchain_provider.getWalletUsing("me.sk");

    const contract = new Contract(compiledContractPath, blockchain_provider, wallet);
    const second_fibonacci = 3
    await contract.spend(validatorScriptIndex, txHashFromDeposit, mZKRedeemer(second_fibonacci))
}

main();
