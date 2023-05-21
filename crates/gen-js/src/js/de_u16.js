function deserializeU16(de) {
    return try_take_varint(de, 16)
}