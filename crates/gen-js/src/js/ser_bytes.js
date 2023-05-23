function serBytes(ser, val) {
    serU64(ser, val.length);
    ser.push(val)
}
