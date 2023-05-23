function deserializeU64(de) {
    return try_take_varint(de, 64)
}