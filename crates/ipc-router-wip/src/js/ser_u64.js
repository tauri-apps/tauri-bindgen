function serializeU64(out, val) {
  return ser_varint_big(out, 64, BigInt(val))
}
