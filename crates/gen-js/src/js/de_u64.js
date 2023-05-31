function deserializeU64(de) {
  return de_varint_big(de, 64)
}
