function deserializeS128(de) {
  const n = de_varint_big(de, 128)

  return ((n >> 1n) & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFn) ^ (-((n & 0b1n) & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFn))
}
