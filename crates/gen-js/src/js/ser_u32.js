function serializeU32(out, val) {
    return ser_varint(out, 32, val)
}
