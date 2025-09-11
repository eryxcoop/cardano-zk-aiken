import {BlockchainProvider} from "./blockchain_provider";
import {Contract, mVoid} from "./contract";

async function main() {
    const compiledContractPath = "../plutus.json";
    const validatorScriptIndex = 0 ;
    const txHashFromDeposit = process.argv[2];
    const blockchain_provider = new BlockchainProvider();
    const wallet = blockchain_provider.getWalletUsing("me.sk");

    const contract = new Contract(compiledContractPath, blockchain_provider, wallet);
    await contract.spend(validatorScriptIndex, txHashFromDeposit, mVoid());
}

main();
