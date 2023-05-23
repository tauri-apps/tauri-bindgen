function serOption(ser, inner, val) {
    ser.pushU8(val !== undefined && val !== null ? 1 : 0)
    if (val !== undefined && val !== null) {
        inner(ser, val)
    }
}
