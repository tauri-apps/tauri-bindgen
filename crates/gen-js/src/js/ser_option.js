function serializeOption(out, inner, val) {
    serializeU8(out, !!val ? 1 : 0)
    if (val) {
        inner(out, val)
    }
}