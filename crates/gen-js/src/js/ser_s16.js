function serializeS16(out, val) {
    ser_varint(out, 16, (val << 1) ^ (val >> 15))
}
