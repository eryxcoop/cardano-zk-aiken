import {MConStr} from "@meshsdk/common";
import {Data, mConStr0} from "@meshsdk/core";

type Proof = MConStr<any, string[]>;

type ZKRedeemer = MConStr<any, Data[] | Proof[]>;

function mProof(piA: string, piB: string, piC: string): Proof {
    if (piA.length != 96 || piB.length != 192 || piC.length != 96) {
        throw new Error("Wrong proof");
    }

    return mConStr0([piA, piB, piC]);
}

export function mZKRedeemer(redeemer: Data): ZKRedeemer {
    return mConStr0([redeemer, proofs()]);
}

function proofs(): Proof[] {
    return [
		mProof(
			"abfd11f8a5231a64619cfe63cb1de2d2bd31ada2c47d1442ac8bfb3df86717a6348c14a58fc517fd6c2ffa037724c43a",
			"9848f0019fdcec67dda5847ba2ba981cb0314af9a847f3b06a4805af1ec627c94f36ab5e428bb24305ae02011ce9c80a17d38792e8c6f4cef37ccdfedb2e7db34dd1fbc37cb48d5be0276b213ed39eef2233371ae9db3da836b135eb8dd6062b",
			"8d67743ec5119da38a774f62bd4d018d1bd15da083f9247fedd783671685365e4c0e9c6b81c54dd111fab0304430575c",
		),
    ];
}
