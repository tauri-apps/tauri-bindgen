function serializeList(out, inner, val) {
    serializeU64(out, val.length)
    for (const el in val) {
        inner(out, el)
    }
}