function serializeS16(out, val) {
    ser_varint(out, 16, BigInt((val << 1) ^ (val >> 15)))
}