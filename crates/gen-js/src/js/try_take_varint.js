function varint_max(type) {
    const BITS_PER_BYTE = 8;
    const BITS_PER_VARINT_BYTE = 7;

    const bits = type * BITS_PER_BYTE;

    const roundup_bits = bits + (BITS_PER_BYTE - 1);

    return Math.floor(roundup_bits / BITS_PER_VARINT_BYTE);
}

function max_of_last_byte(type) {
    let extra_bits = type % 7;
    return (1 << extra_bits) - 1;
}

function try_take_varint(de, type) {
    let out = 0n;

    for (let i = 0; i < varint_max(type); i++) {
        const val = de.pop();
        const carry = BigInt(val & 0x7F);
        out |= carry << (7n * BigInt(i));

        if ((val & 0x80) === 0) {
            if (i === varint_max(type) - 1 && val > max_of_last_byte(type)) {
                throw new Error('deserialize bad variant')
            } else {
                return out
            }
        }
    }

    throw new Error('deserialize bad variant')
}