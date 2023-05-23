function deVarint(de, bits) {
    let out = 0;

    for (let i = 0; i < varintMax[bits]; i++) {
        const val = de.pop();
        const carry = val & 0x7F;
        out |= carry << (7 * i);

        if ((val & 0x80) === 0) {
            if (i === varintMax[bits] - 1 && val > ((1 << bits % 7) - 1)) {
                throw new Error('deserialize bad variant')
            } else {
                return out
            }
        }
    }

    throw new Error('deserialize bad variant')
}