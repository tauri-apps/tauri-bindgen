function serializeF64(out, val) {
    const buf = new Uint8Array(8);
    const view = new DataView(buf);

    view.setFloat64(0, val, true);

    out.push([...buf])
}