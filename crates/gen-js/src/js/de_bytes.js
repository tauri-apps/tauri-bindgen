function deBytes(de) {
    const sz = deU64(de);

    let bytes = de.try_take_n(Number(sz));

    return bytes;
}
