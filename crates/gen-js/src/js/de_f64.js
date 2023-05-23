function deF64(de) {
    const view = new DataView(de.source.buffer, de.offset, 8);
    de.offset += 8;
    return view.getFloat64(0, true);
}
