export default class CborArray extends Array {
    private byteSpan;
    setByteSpan(byteSpan: [number, number]): void;
    getByteSpan(): [number, number];
}
