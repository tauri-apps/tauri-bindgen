function serializeChar(out, val) {
    if (val.len > 1) {
        throw new Error("Serialize bad char");
    }

    serializeU64(out, val.length);

    const encoder = new TextEncoder();

    out.push(...encoder.encode(val))
}