function deserializeResult(de, ok, err) {
    const tag = de.pop()

    switch (tag) {
        case 0:
            return { Ok: ok(de) }
        case 1: 
            return { Err: err(de) }
        default:
            throw new Error(`Deserialize bad result ${tag}`)
    }
}