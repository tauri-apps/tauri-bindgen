function deString(de) {
    const sz = deU64(de);

    let bytes = de.try_take_n(sz);

    return __text_decoder.decode(bytes);
}
