function serializeResult(out, ok, err, val) {
    if (val.Ok) {
        serializeU8(out, 0);
        return ok(out, val.Ok);
    }

    if (val.Err) {
        serializeU8(out, 1);
        return err(out, val.Err);
    }

    throw new Error(`Serialize bad result ${val}`);
}
