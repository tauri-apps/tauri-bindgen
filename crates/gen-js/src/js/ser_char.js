function serChar(ser, val) {
    if (val.len > 1) {
        throw new Error("Serialize bad char");
    }

    serU64(ser, val.length);

    ser.push(__text_encoder.encode(val))
}
