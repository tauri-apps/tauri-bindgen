use anyhow::ensure;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;

use crate::roundtrip::roundtrip;

mod roundtrip_js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(raw_module = "./roundtrip.js")]
    extern "C" {
        pub async fn empty(x: JsValue) -> JsValue;
        pub async fn recordScalars(x: JsValue) -> JsValue;
        pub async fn recordReallyFlags(x: JsValue) -> JsValue;
        pub async fn recordAggregates(x: JsValue) -> JsValue;
        pub async fn float32(x: JsValue) -> JsValue;
        pub async fn float64(x: JsValue) -> JsValue;
        pub async fn u8(x: JsValue) -> JsValue;
        pub async fn s8(x: JsValue) -> JsValue;
        pub async fn u16(x: JsValue) -> JsValue;
        pub async fn s16(x: JsValue) -> JsValue;
        pub async fn u32(x: JsValue) -> JsValue;
        pub async fn s32(x: JsValue) -> JsValue;
        pub async fn u64(x: JsValue) -> JsValue;
        pub async fn s64(x: JsValue) -> JsValue;
        pub async fn u128(x: JsValue) -> JsValue;
        pub async fn s128(x: JsValue) -> JsValue;
        pub async fn listU8(x: JsValue) -> JsValue;
        pub async fn listU16(x: JsValue) -> JsValue;
        pub async fn listU32(x: JsValue) -> JsValue;
        pub async fn listU64(x: JsValue) -> JsValue;
        pub async fn listU128(x: JsValue) -> JsValue;
        pub async fn listS8(x: JsValue) -> JsValue;
        pub async fn listS16(x: JsValue) -> JsValue;
        pub async fn listS32(x: JsValue) -> JsValue;
        pub async fn listS64(x: JsValue) -> JsValue;
        pub async fn listS128(x: JsValue) -> JsValue;
        pub async fn listFloat32(x: JsValue) -> JsValue;
        pub async fn listFloat64(x: JsValue) -> JsValue;
        pub async fn tupleList(x: JsValue) -> JsValue;
        pub async fn stringList(x: JsValue) -> JsValue;
        pub async fn tupleStringList(x: JsValue) -> JsValue;
        pub async fn allIntegers(x: JsValue) -> JsValue;
        pub async fn allFloats(x: JsValue) -> JsValue;
        pub async fn allText(x: JsValue) -> JsValue;
        pub async fn options(
            a: JsValue,
            b: JsValue,
            c: JsValue,
            d: JsValue,
            e: JsValue,
            f: JsValue,
            g: JsValue,
        ) -> JsValue;
        pub async fn results(
            a: JsValue,
            b: JsValue,
            c: JsValue,
            d: JsValue,
            e: JsValue,
            f: JsValue,
        ) -> JsValue;
    }
}

pub async fn empty() -> anyhow::Result<()> {
    let x = roundtrip::Empty {};

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::empty(jsval).await;

    let other: roundtrip::Empty = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}

pub async fn record_scalars() -> anyhow::Result<()> {
    let x = roundtrip::Scalars { a: 6767, b: 75744 };

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::recordScalars(jsval).await;

    let other: roundtrip::Scalars = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn record_really_flags() -> anyhow::Result<()> {
    let x = roundtrip::ReallyFlags {
        a: true,
        b: true,
        c: true,
        d: false,
        e: false,
        f: false,
        g: true,
        h: false,
        i: true,
    };

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::recordReallyFlags(jsval).await;

    let other: roundtrip::ReallyFlags = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn record_aggregates() -> anyhow::Result<()> {
    let x = roundtrip::AggregatesParam {
        a: roundtrip::Scalars { a: 677, b: 980 },
        b: 42,
        c: roundtrip::Empty {},
        d: "hello world",
        e: roundtrip::ReallyFlags {
            a: true,
            b: true,
            c: true,
            d: false,
            e: false,
            f: false,
            g: true,
            h: false,
            i: true,
        },
    };

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::recordAggregates(jsval).await;

    let res: roundtrip::AggregatesResult = from_value(jsval).unwrap();

    ensure!(res.a == x.a);
    ensure!(res.b == x.b);
    ensure!(res.c == x.c);
    ensure!(res.d == x.d);
    ensure!(res.e == x.e);

    Ok(())
}
// pub async fn flag1() -> anyhow::Result<()> {
//     let x = roundtrip::Flag1::B0;

//     ensure!(roundtrip::flag1(x).await == x);

//     Ok(())
// }
// pub async fn flag2(x: Flag2) -> Flag2 {
//     #[derive(::serde::Serialize)]
//     #[serde(rename_all = "camelCase")]
//     struct Params {
//         x: Flag2,
//     }
//     let params = Params { x };
//     ::tauri_bindgen_guest_rust::send("plugin:imports|flag2", &params).await
// }
// pub async fn flag4(x: Flag4) -> Flag4 {
//     #[derive(::serde::Serialize)]
//     #[serde(rename_all = "camelCase")]
//     struct Params {
//         x: Flag4,
//     }
//     let params = Params { x };
//     ::tauri_bindgen_guest_rust::send("plugin:imports|flag4", &params).await
// }
// pub async fn flag8(x: Flag8) -> Flag8 {
//     #[derive(::serde::Serialize)]
//     #[serde(rename_all = "camelCase")]
//     struct Params {
//         x: Flag8,
//     }
//     let params = Params { x };
//     ::tauri_bindgen_guest_rust::send("plugin:imports|flag8", &params).await
// }
// pub async fn flag16(x: Flag16) -> Flag16 {
//     #[derive(::serde::Serialize)]
//     #[serde(rename_all = "camelCase")]
//     struct Params {
//         x: Flag16,
//     }
//     let params = Params { x };
//     ::tauri_bindgen_guest_rust::send("plugin:imports|flag16", &params).await
// }
// pub async fn flag32(x: Flag32) -> Flag32 {
//     #[derive(::serde::Serialize)]
//     #[serde(rename_all = "camelCase")]
//     struct Params {
//         x: Flag32,
//     }
//     let params = Params { x };
//     ::tauri_bindgen_guest_rust::send("plugin:imports|flag32", &params).await
// }
// pub async fn flag64(x: Flag64) -> Flag64 {
//     #[derive(::serde::Serialize)]
//     #[serde(rename_all = "camelCase")]
//     struct Params {
//         x: Flag64,
//     }
//     let params = Params { x };
//     ::tauri_bindgen_guest_rust::send("plugin:imports|flag64", &params).await
// }
pub async fn float32() -> anyhow::Result<()> {
    let x = 512.68666686;

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::float32(jsval).await;

    let other: f32 = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn float64() -> anyhow::Result<()> {
    let x = 512.68666686;

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::float64(jsval).await;

    let other: f64 = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn u8() -> anyhow::Result<()> {
    let x = 12;

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::u8(jsval).await;

    let other: u8 = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn s8() -> anyhow::Result<()> {
    let x = -12;

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::s8(jsval).await;

    let other: i8 = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn u16() -> anyhow::Result<()> {
    let x = 177;

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::u16(jsval).await;

    let other: u16 = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn s16() -> anyhow::Result<()> {
    let x = -177;

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::s16(jsval).await;

    let other: i16 = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn u32() -> anyhow::Result<()> {
    let x = 177777777;

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::u32(jsval).await;

    let other: u32 = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn s32() -> anyhow::Result<()> {
    let x = -17777777;

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::s32(jsval).await;

    let other: i32 = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn u64() -> anyhow::Result<()> {
    let x = 1777777777267;

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::u64(jsval).await;

    let other: u64 = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn s64() -> anyhow::Result<()> {
    let x = -1777777777267;

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::s64(jsval).await;

    let other: i64 = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn u128() -> anyhow::Result<()> {
    let x = 1777777777267;

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::u128(jsval).await;

    let other: u128 = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn s128() -> anyhow::Result<()> {
    let x = -1777777777267;

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::s128(jsval).await;

    let other: i128 = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn list_u8() -> anyhow::Result<()> {
    let x = [16, 32, 42];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::listU8(jsval).await;

    let other: [u8; 3] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn list_u16() -> anyhow::Result<()> {
    let x = [16, 32, 42, 666];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::listU16(jsval).await;

    let other: [u16; 4] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn list_u32() -> anyhow::Result<()> {
    let x = [16, 32, 42, 176776];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::listU32(jsval).await;

    let other: [u32; 4] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn list_u64() -> anyhow::Result<()> {
    let x = [16, 32, 42, 17776276762];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::listU64(jsval).await;

    let other: [u64; 4] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn list_u128() -> anyhow::Result<()> {
    let x = [16, 32, 42, 17776276762];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::listU64(jsval).await;

    let other: [u128; 4] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn list_s8() -> anyhow::Result<()> {
    let x = [16, 32, 42, -24, -26];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::listS8(jsval).await;

    let other: [i8; 5] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn list_s16() -> anyhow::Result<()> {
    let x = [16, 32, 42, 862, -2767];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::listS16(jsval).await;

    let other: [i16; 5] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn list_s32() -> anyhow::Result<()> {
    let x = [16, 32, 42, 862868, -1868268];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::listS32(jsval).await;

    let other: [i32; 5] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn list_s64() -> anyhow::Result<()> {
    let x = [16, 32, 42, 187878787, -18787827];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::listS64(jsval).await;

    let other: [i64; 5] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn list_s128() -> anyhow::Result<()> {
    let x = [16, 32, 42, 187878787, -18787827];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::listS64(jsval).await;

    let other: [i128; 5] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn list_float32() -> anyhow::Result<()> {
    let x = [16.0, 32.5, 42.187878787, -18787827.7];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::listFloat32(jsval).await;

    let other: [f32; 4] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn list_float64() -> anyhow::Result<()> {
    let x = [16.0, 32.5, 42373777773.187878787, -18787827.7];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::listFloat64(jsval).await;

    let other: [f64; 4] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn tuple_list() -> anyhow::Result<()> {
    let x = [(12, -12), (26, -26), (42, -42)];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::tupleList(jsval).await;

    let other: [(u8, i8); 3] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn string_list() -> anyhow::Result<()> {
    let x = ["hello", "world", "this", "is", "a", "test"];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::stringList(jsval).await;

    let other: [String; 6] = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn tuple_string_list() -> anyhow::Result<()> {
    let x = [
        (0, "hello"),
        (1, "world"),
        (2, "this"),
        (3, "is"),
        (4, "a"),
        (5, "test"),
    ];

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::tupleStringList(jsval).await;

    let other: [(u8, String); 6] = from_value(jsval).unwrap();

    for (i, str) in x {
        ensure!(other[i as usize].0 == i);
        ensure!(other[i as usize].1 == str);
    }

    Ok(())
}
pub async fn all_integers() -> anyhow::Result<()> {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct AllIntegers {
        tag: u32,
        val: AllIntegersInner,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    #[serde(untagged)]
    enum AllIntegersInner {
        Bool(bool),
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        S8(u8),
        S16(u16),
        S32(u32),
        S64(u64),
    }

    let x = roundtrip::AllIntegers::U8(7);

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::allIntegers(jsval).await;

    let other: roundtrip::AllIntegers = from_value(jsval).unwrap();

    ensure!(other == x);

    let x = roundtrip::AllIntegers::I16(-67);

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::allIntegers(jsval).await;

    let other: roundtrip::AllIntegers = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn all_floats() -> anyhow::Result<()> {
    let x = roundtrip::AllFloats::F32(43.888);

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::allFloats(jsval).await;

    let other: roundtrip::AllFloats = from_value(jsval).unwrap();

    ensure!(other == x);

    let x = roundtrip::AllFloats::F64(-67.60986086086);

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::allFloats(jsval).await;

    let other: roundtrip::AllFloats = from_value(jsval).unwrap();

    ensure!(other == x);

    Ok(())
}
pub async fn all_text() -> anyhow::Result<()> {
    let x = roundtrip::AllTextParam::String("foobar");

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::allText(jsval).await;

    let other: roundtrip::AllTextResult = from_value(jsval).unwrap();

    ensure!(other == roundtrip::AllTextResult::String("foobar".to_string()));

    let x = roundtrip::AllTextParam::Char('a');

    let jsval = to_value(&x).unwrap();
    let jsval = roundtrip_js::allText(jsval).await;

    let other: roundtrip::AllTextResult = from_value(jsval).unwrap();

    ensure!(other == roundtrip::AllTextResult::Char('a'));

    Ok(())
}
pub async fn options() -> anyhow::Result<()> {
    let a = None;
    let b = Some(());
    let c = Some(89629);
    let d = Some(roundtrip::E1::A);
    let e = Some(698698.9759859);
    let f = Some(roundtrip::U1::F32(828629869.997279));
    let g = Some(Some(true));

    let jsval = roundtrip_js::options(
        to_value(&a).unwrap(),
        to_value(&b).unwrap(),
        to_value(&c).unwrap(),
        to_value(&d).unwrap(),
        to_value(&e).unwrap(),
        to_value(&f).unwrap(),
        to_value(&g).unwrap(),
    )
    .await;

    let exp = (a, b, c, d, e, f, g);
    let res: (
        Option<bool>,
        Option<()>,
        Option<u32>,
        Option<roundtrip::E1>,
        Option<f32>,
        Option<roundtrip::U1>,
        Option<Option<bool>>,
    ) = from_value(jsval).unwrap();

    // exp (None, Some(()), Some(89629), Some(A), Some(698699.0), Some(F32(828629900.0)), Some(Some(true)))
    // got (None, None, Some(89629), Some(A), Some(698699.0), Some(F32(828629900.0)), Some(Some(true)))

    log::debug!("exp {:?} got {:?}", exp, res);
    ensure!(res == exp);

    Ok(())
}
pub async fn results() -> anyhow::Result<()> {
    let a = Ok(());
    let b = Err(roundtrip::E1::A);
    let c = Err(());
    let d = Ok(());
    let e: Result<u32, roundtrip::V1Param> = Err(roundtrip::V1Param::D("hello world"));
    let f: Result<&str, &[u8]> = Ok("foobar");

    let jsval = roundtrip_js::results(
        to_value(&a).unwrap(),
        to_value(&b).unwrap(),
        to_value(&c).unwrap(),
        to_value(&d).unwrap(),
        to_value(&e).unwrap(),
        to_value(&f).unwrap(),
    )
    .await;

    let res: (
        Result<(), ()>,
        Result<(), roundtrip::E1>,
        Result<roundtrip::E1, ()>,
        Result<(), ()>,
        Result<u32, roundtrip::V1Result>,
        Result<String, Vec<u8>>,
    ) = from_value(jsval).unwrap();
    let exp = (
        a,
        b,
        c,
        d,
        Err(roundtrip::V1Result::D("hello world".to_string())),
        Ok("foobar".to_string()),
    );

    ensure!(res == exp);

    Ok(())
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    #[test]
    fn feature() {
        #[derive(Debug, Serialize, Deserialize)]
        enum AllFloats {
            F32,
            F64,
        }

        println!("{:?}", serde_json::to_string(&AllFloats::F64))
    }
}
