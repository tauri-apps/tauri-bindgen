function serializeU16(out, val) {
    return ser_varint(out, 16, val)
}