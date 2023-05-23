function serializeF32(out, val) {
    const buf = new Uint8Array(4);
    const view = new DataView(buf);

    view.setFloat32(0, val, true);

    out.push([...buf])
}