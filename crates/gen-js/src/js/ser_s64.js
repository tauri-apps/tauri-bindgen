function serS64(ser, val) {
    serVarint(ser, 64, (val << 1) ^ (val >> 63))
}
