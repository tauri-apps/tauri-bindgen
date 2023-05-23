function deserializeOption(de, inner) {
    const disc = de.pop()

    switch (disc) {
        case 0:
            return null
        case 1: 
            return inner(de)
        default:
            throw new Error('Deserialize bad option')
    }
}