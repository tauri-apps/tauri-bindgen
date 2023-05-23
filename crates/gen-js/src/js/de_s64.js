function deserializeS64(de) {
    const n = de_varint(de, 64)

    return Number(((n >> 1) & 0xFFFFFFFFFFFFFFFF) ^ (-((n & 0b1) & 0xFFFFFFFFFFFFFFFF)))
}