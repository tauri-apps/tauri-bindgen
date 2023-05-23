function serString(ser, val) {
    serVarint(ser, 64, val.length)
    ser.push(__text_encoder.encode(val))
}
