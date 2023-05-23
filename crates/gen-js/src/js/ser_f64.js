function serF64(ser, val) {
    const view = new DataView(ser.bytes.buffer, ser.offset, 8);
    ser.offset += 8;
    view.setFloat64(0, val, true);
}
