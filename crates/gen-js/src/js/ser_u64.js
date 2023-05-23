function serializeU64(out, val) {
    return ser_varint(out, 64, val)
}