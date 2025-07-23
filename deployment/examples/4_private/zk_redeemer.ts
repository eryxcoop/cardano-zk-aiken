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
            "a6ff73ac3b7e373996ad4e45f4392c70de3da99db07933b44f504d230d2cf36b7f6f6e50761cda8d1297b36c5b216ed4",
            "91ef5fea8f068dbeb1893af96fd52e2d9b7e25cad9a0639c040e45e0a273a54856c0b37d00fc4601340e7028502da6de07079fe9d07c06122124750b63626e5150716a3bd601ea80420860151e5894c3a8c581636dedda8e0071fddd5aa93b9c",
            "a241a69c5c35aa559c9e79e5ee537f56c7fe7d083f7796cce1f8b6d119a1f893739af4a93d7d5743d31201c709cf9b4f"
        ),
    ];
}