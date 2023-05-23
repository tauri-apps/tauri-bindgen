function serializeOption(out, inner, val) {
    serializeU8(out, !!val ? 1 : 0)
    inner(out, val)
}