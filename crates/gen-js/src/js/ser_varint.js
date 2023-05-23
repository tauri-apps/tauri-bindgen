function serVarint(ser, bits, val) {
    for (let i = 0; i < varintMax[bits]; i++) {
        const buffer = new ArrayBuffer(bits / 8);
        const view = new DataView(buffer);
        view.setInt16(0, val, true);
        ser.bytes[ser.offset] = view.getUint8(0);
        if (val < 128) {
            ser.offset += 1
            return;
        }

        ser.bytes[ser.offset] |= 0x80;
        val >>= 7;
        ser.offset += 1
    }
}
