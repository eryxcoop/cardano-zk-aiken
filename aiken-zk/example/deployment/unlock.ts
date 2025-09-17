import {BlockchainProvider} from "./blockchain_provider";
import {Contract, mVoid} from "./contract";
import {mConStr0} from "@meshsdk/core";

async function main() {
    const compiledContractPath = "../plutus.json";
    const validatorScriptIndex = 0;
    const txHashFromDeposit = process.argv[2];
    const blockchain_provider = new BlockchainProvider();
    const wallet = blockchain_provider.getWalletUsing("me.sk");

    const contract = new Contract(compiledContractPath, blockchain_provider, wallet);

    const proof = mConStr0([
        "949135c16bda90ad1633a73dc349e3bfa5791d81f66c3bc2de11c58230fff2b24a8c736d72f6e77ad466354c0245a3d9",
        "8cb4dfcf31b1a950656af753975bc46a76c7ff38f9f024e77e93478c92f9d65a5a6c36ecdb566be81b19a37f6240b9a70f295405af43470931b462ca27189ae51e4aceb38ab457e0c19392dccc1d35cc12f61fec33988f49447c1ddac36e46d2",
        "99a38d427161b538c189cbb7720aa1b6a84afb7e3824caad44a5e54bb91d4bd1b292ce8e08a7697c89aa2bf2ccc6e739"
    ]);

    const redeemer = mVoid();
    const zk_redeemer = mConStr0([redeemer, [proof]]);

    await contract.spend(validatorScriptIndex, txHashFromDeposit, zk_redeemer);
}

main();
