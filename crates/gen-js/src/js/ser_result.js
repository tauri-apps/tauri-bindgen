function serResult(ser, ok, err, val) {
    if (val.Ok) {
        serU8(ser, 0);
        return ok(ser, val.Ok);
    }

    if (val.Err) {
        serU8(ser, 1);
        return err(ser, val.Err);
    }

    throw new Error(`Serialize bad result ${val}`);
}
