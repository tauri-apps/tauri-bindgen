
declare global {
  interface Window {
    __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;export enum Flag1 {
  B0 = 1,
}
export enum Flag2 {
  B0 = 1,
  B1 = 2,
}
export enum Flag4 {
  B0 = 1,
  B1 = 2,
  B2 = 4,
  B3 = 8,
}
export enum Flag8 {
  B0 = 1,
  B1 = 2,
  B2 = 4,
  B3 = 8,
  B4 = 16,
  B5 = 32,
  B6 = 64,
  B7 = 128,
}
export enum Flag16 {
  B0 = 1,
  B1 = 2,
  B2 = 4,
  B3 = 8,
  B4 = 16,
  B5 = 32,
  B6 = 64,
  B7 = 128,
  B8 = 256,
  B9 = 512,
  B10 = 1024,
  B11 = 2048,
  B12 = 4096,
  B13 = 8192,
  B14 = 16384,
  B15 = 32768,
}
export type Flag32 = typeof Flag32;export const Flag32 = {
  B0: BigInt(1),
  B1: BigInt(2),
  B2: BigInt(4),
  B3: BigInt(8),
  B4: BigInt(16),
  B5: BigInt(32),
  B6: BigInt(64),
  B7: BigInt(128),
  B8: BigInt(256),
  B9: BigInt(512),
  B10: BigInt(1024),
  B11: BigInt(2048),
  B12: BigInt(4096),
  B13: BigInt(8192),
  B14: BigInt(16384),
  B15: BigInt(32768),
  B16: BigInt(65536),
  B17: BigInt(131072),
  B18: BigInt(262144),
  B19: BigInt(524288),
  B20: BigInt(1048576),
  B21: BigInt(2097152),
  B22: BigInt(4194304),
  B23: BigInt(8388608),
  B24: BigInt(16777216),
  B25: BigInt(33554432),
  B26: BigInt(67108864),
  B27: BigInt(134217728),
  B28: BigInt(268435456),
  B29: BigInt(536870912),
  B30: BigInt(1073741824),
  B31: BigInt(2147483648),
}
export type Flag64 = typeof Flag64;export const Flag64 = {
  B0: BigInt(1),
  B1: BigInt(2),
  B2: BigInt(4),
  B3: BigInt(8),
  B4: BigInt(16),
  B5: BigInt(32),
  B6: BigInt(64),
  B7: BigInt(128),
  B8: BigInt(256),
  B9: BigInt(512),
  B10: BigInt(1024),
  B11: BigInt(2048),
  B12: BigInt(4096),
  B13: BigInt(8192),
  B14: BigInt(16384),
  B15: BigInt(32768),
  B16: BigInt(65536),
  B17: BigInt(131072),
  B18: BigInt(262144),
  B19: BigInt(524288),
  B20: BigInt(1048576),
  B21: BigInt(2097152),
  B22: BigInt(4194304),
  B23: BigInt(8388608),
  B24: BigInt(16777216),
  B25: BigInt(33554432),
  B26: BigInt(67108864),
  B27: BigInt(134217728),
  B28: BigInt(268435456),
  B29: BigInt(536870912),
  B30: BigInt(1073741824),
  B31: BigInt(2147483648),
  B32: BigInt(4294967296),
  B33: BigInt(8589934592),
  B34: BigInt(17179869184),
  B35: BigInt(34359738368),
  B36: BigInt(68719476736),
  B37: BigInt(137438953472),
  B38: BigInt(274877906944),
  B39: BigInt(549755813888),
  B40: BigInt(1099511627776),
  B41: BigInt(2199023255552),
  B42: BigInt(4398046511104),
  B43: BigInt(8796093022208),
  B44: BigInt(17592186044416),
  B45: BigInt(35184372088832),
  B46: BigInt(70368744177664),
  B47: BigInt(140737488355328),
  B48: BigInt(281474976710656),
  B49: BigInt(562949953421312),
  B50: BigInt(1125899906842624),
  B51: BigInt(2251799813685248),
  B52: BigInt(4503599627370496),
  B53: BigInt(9007199254740992),
  B54: BigInt(18014398509481984),
  B55: BigInt(36028797018963968),
  B56: BigInt(72057594037927936),
  B57: BigInt(144115188075855872),
  B58: BigInt(288230376151711744),
  B59: BigInt(576460752303423488),
  B60: BigInt(1152921504606846976),
  B61: BigInt(2305843009213693952),
  B62: BigInt(4611686018427387904),
  B63: BigInt(9223372036854775808),
}
export async function roundtripFlag1(x: Flag1): Promise<Flag1> {
  const result = await invoke<Flag1>("plugin:13a360f690a38bbb|roundtrip-flag1",{x: x});
  return result
}
export async function roundtripFlag2(x: Flag2): Promise<Flag2> {
  const result = await invoke<Flag2>("plugin:13a360f690a38bbb|roundtrip-flag2",{x: x});
  return result
}
export async function roundtripFlag4(x: Flag4): Promise<Flag4> {
  const result = await invoke<Flag4>("plugin:13a360f690a38bbb|roundtrip-flag4",{x: x});
  return result
}
export async function roundtripFlag8(x: Flag8): Promise<Flag8> {
  const result = await invoke<Flag8>("plugin:13a360f690a38bbb|roundtrip-flag8",{x: x});
  return result
}
export async function roundtripFlag16(x: Flag16): Promise<Flag16> {
  const result = await invoke<Flag16>("plugin:13a360f690a38bbb|roundtrip-flag16",{x: x});
  return result
}
export async function roundtripFlag32(x: Flag32): Promise<Flag32> {
  const result = await invoke<Flag32>("plugin:13a360f690a38bbb|roundtrip-flag32",{x: x});
  return result
}
export async function roundtripFlag64(x: Flag64): Promise<Flag64> {
  const result = await invoke<Flag64>("plugin:13a360f690a38bbb|roundtrip-flag64",{x: x});
  return result
}

