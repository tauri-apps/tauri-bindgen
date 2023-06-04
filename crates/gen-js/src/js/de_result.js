function deserializeResult(de, ok, err) {
    const tag = de.pop()

    switch (tag) {
        case 0:
            return { tag: 'ok', val: ok(de) }
        case 1: 
            return { tag: 'err', val: err(de) }
        default:
            throw new Error(`Deserialize bad result ${tag}`)
    }
}
