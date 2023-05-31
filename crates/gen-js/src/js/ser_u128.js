function serializeU128(out, val) {
  return ser_varint_big(out, 128, BigInt(val))
}
