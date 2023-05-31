use anyhow::ensure;

tauri_bindgen_guest_rust::generate!({
    path: "world.wit"
});

pub async fn empty() -> anyhow::Result<()> {
    let x = roundtrip::Empty {};

    ensure!(roundtrip::empty(x.clone()).await == x);

    Ok(())
}

pub async fn record_scalars() -> anyhow::Result<()> {
    let x = roundtrip::Scalars { a: 6767, b: 75744 };

    ensure!(roundtrip::record_scalars(x.clone()).await == x);

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

    ensure!(roundtrip::record_really_flags(x.clone()).await == x);

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

    let res = roundtrip::record_aggregates(x.clone()).await;

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

    ensure!(roundtrip::float32(x).await == x);

    Ok(())
}
pub async fn float64() -> anyhow::Result<()> {
    let x = 512.68666686;

    ensure!(roundtrip::float64(512.68666686).await == x);

    Ok(())
}
pub async fn u8() -> anyhow::Result<()> {
    let x = 12;

    ensure!(roundtrip::u8(x).await == x);

    Ok(())
}
pub async fn s8() -> anyhow::Result<()> {
    let x = -12;

    ensure!(roundtrip::s8(x).await == x);

    Ok(())
}
pub async fn u16() -> anyhow::Result<()> {
    let x = 177;

    ensure!(roundtrip::u16(x).await == x);

    Ok(())
}
pub async fn s16() -> anyhow::Result<()> {
    let x = -177;

    ensure!(roundtrip::s16(x).await == x);

    Ok(())
}
pub async fn u32() -> anyhow::Result<()> {
    let x = 177777777;

    ensure!(roundtrip::u32(x).await == x);

    Ok(())
}
pub async fn s32() -> anyhow::Result<()> {
    let x = -17777777;

    ensure!(roundtrip::s32(x).await == x);

    Ok(())
}
pub async fn u64() -> anyhow::Result<()> {
    let x = 1777777777267;

    ensure!(roundtrip::u64(x).await == x);

    Ok(())
}
pub async fn s64() -> anyhow::Result<()> {
    let x = -1777777777267;

    ensure!(roundtrip::s64(x).await == x);

    Ok(())
}
pub async fn u128() -> anyhow::Result<()> {
    let x = 1777777777267;

    ensure!(roundtrip::u128(x).await == x);

    Ok(())
}
pub async fn s128() -> anyhow::Result<()> {
    let x = -1777777777267;

    ensure!(roundtrip::s128(x).await == x);

    Ok(())
}
pub async fn list_u8() -> anyhow::Result<()> {
    let x = [16, 32, 42];

    ensure!(roundtrip::list_u8(&x).await == x);

    Ok(())
}
pub async fn list_u16() -> anyhow::Result<()> {
    let x = [16, 32, 42, 666];

    ensure!(roundtrip::list_u16(&x).await == x);

    Ok(())
}
pub async fn list_u32() -> anyhow::Result<()> {
    let x = [16, 32, 42, 176776];

    ensure!(roundtrip::list_u32(&x).await == x);

    Ok(())
}
pub async fn list_u64() -> anyhow::Result<()> {
    let x = [16, 32, 42, 17776276762];

    ensure!(roundtrip::list_u64(&x).await == x);

    Ok(())
}
pub async fn list_u128() -> anyhow::Result<()> {
    let x = [16, 32, 42, 17776276762];

    ensure!(roundtrip::list_u128(&x).await == x);

    Ok(())
}
pub async fn list_s8() -> anyhow::Result<()> {
    let x = [16, 32, 42, -24, -26];

    ensure!(roundtrip::list_s8(&x).await == x);

    Ok(())
}
pub async fn list_s16() -> anyhow::Result<()> {
    let x = [16, 32, 42, 862, -2767];

    ensure!(roundtrip::list_s16(&x).await == x);

    Ok(())
}
pub async fn list_s32() -> anyhow::Result<()> {
    let x = [16, 32, 42, 862868, -1868268];

    ensure!(roundtrip::list_s32(&x).await == x);

    Ok(())
}
pub async fn list_s64() -> anyhow::Result<()> {
    let x = [16, 32, 42, 187878787, -18787827];

    ensure!(roundtrip::list_s64(&x).await == x);

    Ok(())
}
pub async fn list_s128() -> anyhow::Result<()> {
    let x = [16, 32, 42, 187878787, -18787827];

    ensure!(roundtrip::list_s128(&x).await == x);

    Ok(())
}
pub async fn list_float32() -> anyhow::Result<()> {
    let x = [16.0, 32.5, 42.187878787, -18787827.7];

    ensure!(roundtrip::list_float32(&x).await == x);

    Ok(())
}
pub async fn list_float64() -> anyhow::Result<()> {
    let x = [16.0, 32.5, 42373777773.187878787, -18787827.7];

    ensure!(roundtrip::list_float32(&x).await == x);

    Ok(())
}
pub async fn tuple_list() -> anyhow::Result<()> {
    let x = [(12, -12), (26, -26), (42, -42)];

    ensure!(roundtrip::tuple_list(&x).await == x);

    Ok(())
}
pub async fn string_list() -> anyhow::Result<()> {
    let x = ["hello", "world", "this", "is", "a", "test"];

    ensure!(roundtrip::string_list(&x).await == x);

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

    let ret = roundtrip::tuple_string_list(&x).await;

    for (i, str) in x {
        ensure!(ret[i as usize].0 == i);
        ensure!(ret[i as usize].1 == str);
    }

    Ok(())
}
// pub async fn record_list() -> anyhow::Result<()> {
//     let x = [roundtrip::SomeRecord {
//         x: todo!(),
//         y: todo!(),
//         z: todo!(),
//         c1: todo!(),
//         c2: todo!(),
//         c3: todo!(),
//         c4: todo!(),
//     }];

//     ensure!(roundtrip::record_list(&x).await == x);

//     Ok(())
// }
pub async fn all_integers() -> anyhow::Result<()> {
    let x = roundtrip::AllIntegers::I8(67);

    ensure!(roundtrip::all_integers(x.clone()).await == x);

    let x = roundtrip::AllIntegers::I16(-67);

    ensure!(roundtrip::all_integers(x.clone()).await == x);

    Ok(())
}
pub async fn all_floats() -> anyhow::Result<()> {
    let x = roundtrip::AllFloats::F32(67.0);

    ensure!(roundtrip::all_floats(x.clone()).await == x);

    let x = roundtrip::AllFloats::F64(-67.60986086086);

    ensure!(roundtrip::all_floats(x.clone()).await == x);

    Ok(())
}
pub async fn all_text() -> anyhow::Result<()> {
    let x = roundtrip::AllTextParam::String("foobar");

    ensure!(roundtrip::all_text(x).await == roundtrip::AllTextResult::String("foobar".to_string()));

    let x = roundtrip::AllTextParam::Char('a');

    ensure!(roundtrip::all_text(x).await == roundtrip::AllTextResult::Char('a'));

    Ok(())
}
// pub async fn e1(x: E1) -> E1 {
//     #[derive(::serde::Serialize)]
//     #[serde(rename_all = "camelCase")]
//     struct Params {
//         x: E1,
//     }
//     let params = Params { x };
//     ::tauri_bindgen_guest_rust::send("plugin:imports|e1", &params).await
// }
// pub async fn v1(x: V1Param<'_>) -> V1Result {
//     #[derive(::serde::Serialize)]
//     #[serde(rename_all = "camelCase")]
//     struct Params<'a> {
//         x: V1Param<'a>,
//     }
//     let params = Params { x };
//     ::tauri_bindgen_guest_rust::send("plugin:imports|v1", &params).await
// }
pub async fn options() -> anyhow::Result<()> {
    let a = None;
    let b = Some(());
    let c = Some(89629);
    let d = Some(roundtrip::E1::A);
    let e = Some(698698.9759859);
    let f = Some(roundtrip::U1::F32(828629869.997279));
    let g = Some(Some(true));

    let res = roundtrip::options(a, b, c, d.clone(), e, f.clone(), g).await;
    let exp = (a, b, c, d, e, f, g);

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
    let e = Err(roundtrip::V1Param::D("hello world"));
    let f = Ok("foobar");

    let res = roundtrip::results(a, b.clone(), c.clone(), d, e, f).await;
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
