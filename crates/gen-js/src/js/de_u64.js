function deserializeU64(de) {
    return de_varint(de, 64)
}