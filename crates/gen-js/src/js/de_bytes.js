function deserializeBytes(de) {
    const sz = deserializeU64(de);

    let bytes = de.try_take_n(Number(sz));

    return bytes;
}