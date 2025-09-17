import {BlockchainProvider} from "./blockchain_provider";
import {Contract} from "./contract";
import {mConStr0} from "@meshsdk/core";

async function main() {
    const compiledContractPath = "../plutus.json";
    const validatorScriptIndex = 0;
    const txHashFromDeposit = process.argv[2];
    const blockchain_provider = new BlockchainProvider();
    const wallet = blockchain_provider.getWalletUsing("me.sk");

    const contract = new Contract(compiledContractPath, blockchain_provider, wallet);

    const proof = mConStr0([
        "b05f0d4533f347d7ff1fa367a42b5e9106d0fdecff354c621dabef90c04553ae6d67ae8c54003b000ad455d1275a159b",
        "823af7d88b20446fd066e563acba82653c2162d01e16a59ed3525d4432cab80870a9490f280b1e261496e9f4dd61e62702937ad4aba1e0be7270e915a1ce2268ee6fbdbc7b76efe96a48b40681289bcc5fc6d2123e5b459584bf3280edf61784",
        "8bb9871f13d5d8b891f25be5e6c8295bff809f333c17c62cf4dfda26d6fde9183cbd6d9519ee0e95161e1e9b85f4e07c"
    ]);

    const zk_redeemer = mConStr0([proof]);

    await contract.spend(validatorScriptIndex, txHashFromDeposit, zk_redeemer);
}

main();
