function serU32(ser, val) {
    return serVarint(ser, 32, val)
}
