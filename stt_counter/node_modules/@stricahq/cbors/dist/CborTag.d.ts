export default class CborTag {
    value: any;
    tag: number;
    constructor(value: any, tag: number);
    private byteSpan;
    setByteSpan(byteSpan: [number, number]): void;
    getByteSpan(): [number, number];
}
