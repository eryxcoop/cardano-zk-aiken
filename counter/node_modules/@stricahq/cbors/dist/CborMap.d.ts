export default class CborMap extends Map {
    private byteSpan;
    setByteSpan(byteSpan: [number, number]): void;
    getByteSpan(): [number, number];
}
