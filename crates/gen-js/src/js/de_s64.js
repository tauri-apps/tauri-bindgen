function deS64(de) {
    const n = deVarint(de, 64)

    return ((n >> 1) & 0xFFFFFFFFFFFFFFFF) ^ (-((n & 0b1) & 0xFFFFFFFFFFFFFFFF))
}
