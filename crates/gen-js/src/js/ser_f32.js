function serF32(ser, val) {
    const view = new DataView(ser.bytes.buffer, ser.offset, 4);
    ser.offset += 4;
    view.setFloat32(0, val, true);
}
