function serU64(ser, val) {
    return serVarint(ser, 64, val)
}
