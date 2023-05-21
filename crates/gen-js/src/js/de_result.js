function deserializeResult(de, ok, err) {
    const disc = de.pop()

    switch (disc) {
        case 0:
            return ok(de)
        case 1: 
            return err(de)
        default:
            throw new Error('Deserialize bad result')
    }
}