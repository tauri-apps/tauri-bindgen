function deS32(de) {
    const n = deVarint(de, 32)

    return ((n >> 1) & 0xFFFFFFFF) ^ (-((n & 0b1) & 0xFFFFFFFF))
}
