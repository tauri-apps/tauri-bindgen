function serList(ser, inner, val) {
    serU64(ser, val.length)
    for (const el of val) {
        inner(ser, el)
    }
}
