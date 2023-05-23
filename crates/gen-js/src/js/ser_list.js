function serializeList(out, inner, val) {
    serializeU64(out, val.length)
    for (const el of val) {
        inner(out, el)
    }
}