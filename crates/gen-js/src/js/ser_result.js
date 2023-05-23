function serializeResult(out, ok, err, val) {
    switch (val.tag) {
        case 0:
            serializeU8(out, 0);
            return ok(out, val.val);
        case 1:
            serializeU8(out, 1);
            return err(out, val.val);
        default:
            throw new Error('Serialize bad result');
    }
}