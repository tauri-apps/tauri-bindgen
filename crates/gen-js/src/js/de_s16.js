function deS16(de) {
    const n = deVarint(de, 16)

    return ((n >> 1) & 0xFFFF) ^ (-((n & 0b1) & 0xFFFF))
}
