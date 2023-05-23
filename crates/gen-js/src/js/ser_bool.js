function serBool(out, val) {
    out.pushU8(val === true ? 1 : 0)
}
