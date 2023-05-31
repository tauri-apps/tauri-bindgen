function serializeS128(out, val) {
  val = BigInt(val)
  ser_varint_big(out, 128, (val << 1n) ^ (val >> 127n))
}
