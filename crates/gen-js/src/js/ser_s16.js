function serS16(ser, val) {
    serVarint(ser, 16, (val << 1) ^ (val >> 15))
}
