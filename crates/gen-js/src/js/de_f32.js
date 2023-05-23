function deF32(de) {
    const view = new DataView(de.source.buffer, de.offset, 4);
    de.offset += 4;
    return view.getFloat32(0, true);
}
