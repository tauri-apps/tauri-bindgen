function serializeS64(out, val) {
    ser_varint(out, 64, (val << 1) ^ (val >> 63))
}
