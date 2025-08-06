import {MeshWallet} from "@meshsdk/core";
import {writeFileSync} from "node:fs";

async function main() {
    const secret_key = MeshWallet.brew(true) as string;

    writeFileSync("me.sk", secret_key);

    const wallet = new MeshWallet({
        networkId: 0,
        key: {
            type: "root",
            bech32: secret_key,
        },
    });

    writeFileSync("me.addr", (await wallet.getUnusedAddresses())[0]);
}

main();