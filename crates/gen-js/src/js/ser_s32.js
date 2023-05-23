function serS32(ser, val) {
    serVarint(ser, 32, (val << 1) ^ (val >> 31))
}
