function deserializeU32(de) {
    return try_take_varint(de, 32)
}