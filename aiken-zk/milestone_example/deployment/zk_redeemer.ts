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
			"b50426ef0e30697180fab703b14eb591e2f4ce8828d550fdba1d17c08ade0971b7c3446dd5319963792fea351a006441",
			"b64ef642a5ae034be02acb5d32d8508080ce12a27d1c39b74068024de778596bd594d74bbb0d5e4c17a473e988aea20d0b2de5227ae5d3e37677c70611052ddadfbb90bbeb2b7882b5e55a3b60e80def49f701a694e8096048cd1f3851b83d9c",
			"ad447908ccf87b340f7fbfa97c43fb42338807245730f21c2fad2eee9e7fd822836823798a98db7dd6d98f95cd586cc4",
		),
    ];
}
