function serializeBytes(out, val) {
    serializeU64(out, val.length);
    out.push(...val)
}
