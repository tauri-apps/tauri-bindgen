function deS8(de) {
    const view = new DataView(de.source.buffer, de.offset, 1);
    de.offset += 1;
    return view.getInt8(0);
}
