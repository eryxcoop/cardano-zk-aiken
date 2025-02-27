/// <reference types="node" />
export default class PublicKey {
    pubKey: Buffer;
    constructor(pubKey: Buffer);
    toBytes(): Buffer;
    hash(): Buffer;
    verify(signature: Buffer, data: Buffer): any;
}
