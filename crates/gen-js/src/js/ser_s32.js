function serializeS32(out, val) {
    ser_varint(out, 32, (val << 1) ^ (val >> 31))
}