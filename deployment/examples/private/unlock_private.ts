import {BlockchainProvider} from "../../blockchain_provider";
import {Contract, mVoid} from "../../contract";
import {mZKRedeemer} from "./zk_redeemer";


async function main() {
    const compiledContractPath = process.argv[2];
    const validatorScriptIndex = parseInt(process.argv[3]);
    const txHashFromDeposit = process.argv[4];
    const blockchain_provider = new BlockchainProvider();
    const wallet = blockchain_provider.getWalletUsing("me.sk");

    const contract = new Contract(compiledContractPath, blockchain_provider, wallet);
    const myRedeemer = mVoid();
    const redeemer = mZKRedeemer(myRedeemer);
    await contract.spend(validatorScriptIndex, txHashFromDeposit, redeemer, {mem: 154984, steps: 4284159244});
}

main();
