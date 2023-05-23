use heck::{ToLowerCamelCase, ToUpperCamelCase};
use tauri_bindgen_core::{flags_repr, union_case_names, TypeInfos};
use wit_parser::{
    EnumCase, FlagsField, Function, FunctionResult, Interface, RecordField, Type, TypeDefArena,
    TypeDefId, TypeDefKind, UnionCase, VariantCase,
};

pub trait JavaScriptGenerator {
    fn interface(&self) -> &Interface;
    fn infos(&self) -> &TypeInfos;

    fn print_deserialize_function_result(&self, result: &FunctionResult) -> String {
        match result.len() {
            0 => String::new(),
            1 => {
                let inner = self.print_deserialize_ty(result.types().next().unwrap());

                format!(
                    "
        .then(r => r.arrayBuffer())
        .then(bytes => {{
            const de = new Deserializer(new Uint8Array(bytes))

            return {inner}
        }})"
                )
            }
            _ => {
                let tys = result
                    .types()
                    .map(|ty| self.print_deserialize_ty(ty))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!(
                    "
        .then(r => r.arrayBuffer())
        .then(bytes => {{
            const de = new Deserializer(Uint8Array.from(bytes))

            return [{tys}]
        }})"
                )
            }
        }
    }

    fn print_deserialize_ty(&self, ty: &Type) -> String {
        match ty {
            Type::Bool => "deBool(de)".to_string(),
            Type::U8 => "deU8(de)".to_string(),
            Type::U16 => "deU16(de)".to_string(),
            Type::U32 => "deU32(de)".to_string(),
            Type::U64 => "deU64(de)".to_string(),
            Type::S8 => "deS8(de)".to_string(),
            Type::S16 => "deS16(de)".to_string(),
            Type::S32 => "deS32(de)".to_string(),
            Type::S64 => "deS64(de)".to_string(),
            Type::Float32 => "deF32(de)".to_string(),
            Type::Float64 => "deF64(de)".to_string(),
            Type::Char => "deChar(de)".to_string(),
            Type::String => "deString(de)".to_string(),
            Type::Tuple(types) => {
                let types = types
                    .iter()
                    .map(|ty| self.print_deserialize_ty(ty))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("[{types}]")
            }
            Type::List(ty) if **ty == Type::U8 => "deBytes(de)".to_string(),
            Type::List(ty) => {
                let inner = self.print_deserialize_ty(ty);
                format!("deList(de, (de) => {inner})")
            }
            Type::Option(ty) => {
                let ty = self.print_deserialize_ty(ty);
                format!("deOption(de, (de) => {ty})")
            }
            Type::Result { ok, err } => {
                let ok = ok
                    .as_ref()
                    .map_or("() => {}".to_string(), |ty| self.print_deserialize_ty(ty));
                let err = err
                    .as_ref()
                    .map_or("() => {}".to_string(), |ty| self.print_deserialize_ty(ty));

                format!("deResult(de, {ok}, {err})")
            }
            Type::Id(id) => {
                if let TypeDefKind::Resource(_) = self.interface().typedefs[*id].kind {
                    format!(
                        "{}.de(de)",
                        self.interface().typedefs[*id].ident.to_upper_camel_case()
                    )
                } else {
                    format!(
                        "de{}(de)",
                        self.interface().typedefs[*id].ident.to_upper_camel_case()
                    )
                }
            }
        }
    }

    fn print_deserialize_typedef(&self, id: TypeDefId) -> String {
        let typedef = &self.interface().typedefs[id];
        let ident = &typedef.ident.to_upper_camel_case();

        match &typedef.kind {
            TypeDefKind::Alias(ty) => self.print_deserialize_alias(ident, ty),
            TypeDefKind::Record(fields) => self.print_deserialize_record(ident, fields),
            TypeDefKind::Flags(fields) => self.print_deserialize_flags(ident, fields),
            TypeDefKind::Variant(cases) => self.print_deserialize_variant(ident, cases),
            TypeDefKind::Enum(cases) => self.print_deserialize_enum(ident, cases),
            TypeDefKind::Union(cases) => self.print_deserialize_union(ident, cases),
            TypeDefKind::Resource(_) => String::new(),
        }
    }

    fn print_deserialize_alias(&self, ident: &str, ty: &Type) -> String {
        let inner = self.print_deserialize_ty(ty);

        format!(
            r#"function de{ident}(de) {{
    return {inner}
}}"#
        )
    }

    fn print_deserialize_record(&self, ident: &str, fields: &[RecordField]) -> String {
        let fields = fields
            .iter()
            .map(|field| {
                let ident = field.ident.to_lower_camel_case();

                format!("{ident}: {}", self.print_deserialize_ty(&field.ty))
            })
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            r#"function de{ident}(de) {{
    return {{
        {fields}
    }}
}}"#
        )
    }

    fn print_deserialize_flags(&self, ident: &str, fields: &[FlagsField]) -> String {
        let inner = match flags_repr(fields) {
            wit_parser::Int::U8 => "U8",
            wit_parser::Int::U16 => "U16",
            wit_parser::Int::U32 => "U32",
            wit_parser::Int::U64 => "U64",
        };

        format!(
            r#"function de{ident}(de) {{
    return de{inner}(de)
}}"#
        )
    }

    fn print_deserialize_variant(&self, ident: &str, cases: &[VariantCase]) -> String {
        let cases = cases
            .iter()
            .enumerate()
            .map(|(tag, case)| {
                let inner = case
                    .ty
                    .as_ref()
                    .map_or("null".to_string(), |ty| self.print_deserialize_ty(ty));

                let ident = case.ident.to_upper_camel_case();

                format!(
                    "case {tag}:
    return {{ {ident}: {inner} }}
"
                )
            })
            .collect::<String>();

        format!(
            r#"function de{ident}(de) {{
    const tag = deU32(de)

    switch (tag) {{
        {cases}
        default:
            throw new Error(`unknown variant case ${{tag}}`)
    }}
}}"#
        )
    }

    fn print_deserialize_enum(&self, ident: &str, cases: &[EnumCase]) -> String {
        let cases = cases
            .iter()
            .enumerate()
            .map(|(tag, case)| {
                let ident = case.ident.to_upper_camel_case();
                format!(
                    "case {tag}:
    return \"{ident}\"
"
                )
            })
            .collect::<String>();

        format!(
            r#"function de{ident}(de) {{
    const tag = deU32(de)

    switch (tag) {{
        {cases}
        default:
            throw new Error(`unknown enum case ${{tag}}`)
    }}
}}"#
        )
    }

    fn print_deserialize_union(&self, ident: &str, cases: &[UnionCase]) -> String {
        let cases: String = union_case_names(&self.interface().typedefs, cases)
            .into_iter()
            .zip(cases)
            .enumerate()
            .map(|(tag, (name, case))| {
                let inner = self.print_deserialize_ty(&case.ty);

                format!(
                    "case {tag}:
    return {{ {name}: {inner} }}
"
                )
            })
            .collect();

        format!(
            r#"function de{ident}(de) {{
    const tag = deU32(de)

    switch (tag) {{
        {cases}
        default:
            throw new Error(`unknown union case ${{tag}}`)
    }}
}}"#
        )
    }

    fn print_serialize_ty(&self, ident: &str, ty: &Type) -> String {
        match ty {
            Type::Bool => format!("serBool(ser, {ident})"),
            Type::U8 => format!("serU8(ser, {ident})"),
            Type::U16 => format!("serU16(ser, {ident})"),
            Type::U32 => format!("serU32(ser, {ident})"),
            Type::U64 => format!("serU64(ser, {ident})"),
            Type::S8 => format!("serS8(ser, {ident})"),
            Type::S16 => format!("serS16(ser, {ident})"),
            Type::S32 => format!("serS32(ser, {ident})"),
            Type::S64 => format!("serS64(ser, {ident})"),
            Type::Float32 => format!("serF32(ser, {ident})"),
            Type::Float64 => format!("serF64(ser, {ident})"),
            Type::Char => format!("serChar(ser, {ident})"),
            Type::String => format!("serString(ser, {ident})"),
            Type::List(ty) if **ty == Type::U8 => format!("serBytes(ser, {ident})"),
            Type::List(ty) => {
                let inner = self.print_serialize_ty("v", ty);

                format!("serList(ser, (ser, v) => {inner}, {ident})")
            }
            Type::Tuple(tys) if tys.is_empty() => "{}".to_string(),
            Type::Tuple(tys) => {
                let inner = tys
                    .iter()
                    .enumerate()
                    .map(|(idx, ty)| self.print_serialize_ty(&format!("{ident}[{idx}]"), ty))
                    .collect::<Vec<_>>()
                    .join(";");

                format!("{{{inner}}}")
            }
            Type::Option(ty) => {
                let inner = self.print_serialize_ty("v", ty);

                format!("serOption(ser, (ser, v) => {inner}, {ident})")
            }
            Type::Result { ok, err } => {
                let ok = ok
                    .as_ref()
                    .map_or("{}".to_string(), |ty| self.print_serialize_ty("v", ty));
                let err = err
                    .as_ref()
                    .map_or("{}".to_string(), |ty| self.print_serialize_ty("v", ty));

                format!("serResult(ser, (ser, v) => {ok}, (ser, v) => {err}, {ident})")
            }
            Type::Id(id) => {
                if let TypeDefKind::Resource(_) = self.interface().typedefs[*id].kind {
                    format!("{ident}.ser(ser)")
                } else {
                    format!(
                        "ser{}(ser, {ident})",
                        self.interface().typedefs[*id].ident.to_upper_camel_case()
                    )
                }
            }
        }
    }

    fn print_serialize_typedef(&self, id: TypeDefId) -> String {
        let typedef = &self.interface().typedefs[id];
        let ident = &typedef.ident.to_upper_camel_case();

        match &typedef.kind {
            TypeDefKind::Alias(ty) => self.print_serialize_alias(ident, ty),
            TypeDefKind::Record(fields) => self.print_serialize_record(ident, fields),
            TypeDefKind::Flags(fields) => self.print_serialize_flags(ident, fields),
            TypeDefKind::Variant(cases) => self.print_serialize_variant(ident, cases),
            TypeDefKind::Enum(cases) => self.print_serialize_enum(ident, cases),
            TypeDefKind::Union(cases) => self.print_serialize_union(ident, cases),
            TypeDefKind::Resource(_) => String::new(),
        }
    }

    fn print_serialize_alias(&self, ident: &str, ty: &Type) -> String {
        let inner = self.print_serialize_ty("val", ty);

        format!(
            "function ser{ident}(ser, val) {{
    {inner}
}}"
        )
    }

    fn print_serialize_record(&self, ident: &str, fields: &[RecordField]) -> String {
        let inner = fields
            .iter()
            .map(|field| self.print_serialize_ty(&format!("val.{}", field.ident), &field.ty))
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            "function ser{ident}(ser, val) {{
    {inner}
}}"
        )
    }

    fn print_serialize_flags(&self, ident: &str, fields: &[FlagsField]) -> String {
        let inner = match flags_repr(fields) {
            wit_parser::Int::U8 => "U8",
            wit_parser::Int::U16 => "U16",
            wit_parser::Int::U32 => "U32",
            wit_parser::Int::U64 => "U64",
        };

        format!(
            r#"function ser{ident}(ser, val) {{
    return ser{inner}(ser, val)
}}"#
        )
    }

    fn print_serialize_variant(&self, ident: &str, cases: &[VariantCase]) -> String {
        let cases = cases
            .iter()
            .enumerate()
            .map(|(tag, case)| {
                let prop_access = format!("val.{}", case.ident.to_upper_camel_case());

                let inner = case.ty.as_ref().map_or(String::new(), |ty| {
                    self.print_serialize_ty(&prop_access, ty)
                });

                format!(
                    "if ({prop_access}) {{
    serU32(ser, {tag});
    return {inner}
}}
"
                )
            })
            .collect::<String>();

        format!(
            r#"function ser{ident}(ser, val) {{
    {cases}

    throw new Error("unknown variant case")
}}"#
        )
    }

    fn print_serialize_enum(&self, ident: &str, cases: &[EnumCase]) -> String {
        let cases = cases
            .iter()
            .enumerate()
            .map(|(tag, case)| {
                let ident = case.ident.to_upper_camel_case();
                format!(
                    "case \"{ident}\":
    serU32(ser, {tag})
    return
"
                )
            })
            .collect::<String>();

        format!(
            r#"function ser{ident}(ser, val) {{
    switch (val) {{
        {cases}
        default:
            throw new Error("unknown enum case")
    }}
}}"#
        )
    }

    fn print_serialize_union(&self, ident: &str, cases: &[UnionCase]) -> String {
        let cases: String = union_case_names(&self.interface().typedefs, cases)
            .into_iter()
            .zip(cases)
            .enumerate()
            .map(|(tag, (name, case))| {
                let prop_access = format!("val.{name}");
                let inner = self.print_serialize_ty(&prop_access, &case.ty);

                format!(
                    "if ({prop_access}) {{
    serU32(ser, {tag});
    return {inner}
}}
                "
                )
            })
            .collect();

        format!(
            r#"function ser{ident}(ser, val) {{
    {cases}

    throw new Error("unknown union case")
}}"#
        )
    }

    fn size_hint_for_type(&self, ident: &str, ty: &Type) -> String {
        match ty {
            Type::Bool | Type::U8 | Type::S8 => "1".to_string(),
            Type::U16 | Type::S16 => "3".to_string(),
            Type::U32 | Type::S32 => "5".to_string(),
            Type::U64 | Type::S64 => "10".to_string(),
            Type::Float32 => "4".to_string(),
            Type::Float64 => "8".to_string(),
            Type::Char => "4".to_string(),
            Type::String => format!("{ident}.length * 4 + 10"),
            Type::List(ty) if matches!(**ty, Type::Bool | Type::U8 | Type::S8) => {
                format!("{ident}.length + 10")
            }
            Type::List(inner) => {
                let inner = self.size_hint_for_type("cur", &inner);

                format!("{ident}.reduce((acc, cur) => acc + {inner}, 0) + 10")
            }
            Type::Tuple(tys) if tys.is_empty() => "0".to_string(),
            Type::Tuple(tys) => tys
                .iter()
                .enumerate()
                .map(|(idx, ty)| self.size_hint_for_type(&format!("{ident}[{idx}]"), ty))
                .collect::<Vec<_>>()
                .join("+"),
            Type::Option(inner) => {
                let inner = self.size_hint_for_type(ident, &inner);
                format!("{inner} + 1")
            }
            Type::Result { ok, err } => {
                let ok = ok.as_ref().map_or("0".to_string(), |ty| {
                    self.size_hint_for_type(&format!("{ident}.Ok"), ty)
                });
                let err = err.as_ref().map_or("0".to_string(), |ty| {
                    self.size_hint_for_type(&format!("{ident}.Err"), ty)
                });

                format!("{ok} + {err} + 1")
            }
            Type::Id(id) => self.size_hint_for_typedef(ident, *id),
        }
    }

    fn size_hint_for_typedef(&self, ident: &str, id: TypeDefId) -> String {
        let typedef = &self.interface().typedefs[id];

        match &typedef.kind {
            TypeDefKind::Alias(ty) => self.size_hint_for_type(ident, ty),
            TypeDefKind::Record(fields) if fields.is_empty() => "0".to_string(),
            TypeDefKind::Record(fields) => fields
                .iter()
                .map(|field| {
                    self.size_hint_for_type(&format!("{ident}.{}", field.ident), &field.ty)
                })
                .collect::<Vec<_>>()
                .join("+"),
            TypeDefKind::Flags(fields) => match flags_repr(fields) {
                wit_parser::Int::U8 => "1".to_string(),
                wit_parser::Int::U16 => "3".to_string(),
                wit_parser::Int::U32 => "5".to_string(),
                wit_parser::Int::U64 => "10".to_string(),
            },
            TypeDefKind::Variant(cases) if cases.is_empty() => "0".to_string(),
            TypeDefKind::Variant(cases) => {
                let inner = cases
                    .into_iter()
                    .filter_map(|case| {
                        if let Some(ty) = &case.ty {
                            let ident = format!("{ident}.{}", case.ident);
                            let inner = self.size_hint_for_type(&ident, ty);

                            Some(format!("{ident} ? {inner} : 0"))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(",");

                format!("Math.max({inner}) + 5")
            }
            TypeDefKind::Enum(cases) if cases.is_empty() => "0".to_string(),
            TypeDefKind::Enum(_) => "5".to_string(),
            TypeDefKind::Union(cases) if cases.is_empty() => "0".to_string(),
            TypeDefKind::Union(cases) => {
                let inner = union_case_names(&self.interface().typedefs, cases)
                    .into_iter()
                    .zip(cases)
                    .map(|(name, case)| {
                        let ident = format!("{ident}.{name}");
                        let inner = self.size_hint_for_type(&ident, &case.ty);
                    
                        format!("{ident} ? {inner} : 0")
                    })
                    .collect::<Vec<_>>()
                    .join(",");

                format!("Math.max({inner}) + 5")
            }
            TypeDefKind::Resource(_) => "10".to_string(),
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct SerdeUtils: u32 {
        const VARINT_MAX        = 1 << 1;
        const _VARINT           = 1 << 2;

        const BOOl              = 1 << 3;
        const U8                = 1 << 4;
        const _U16              = 1 << 5;
        const _U32              = 1 << 6;
        const _U64              = 1 << 7;
        const S8                = 1 << 8;
        const _S16              = 1 << 9;
        const _S32              = 1 << 10;
        const _S64              = 1 << 11;
        const F32               = 1 << 12;
        const F64               = 1 << 13;
        const _CHAR             = 1 << 14;
        const _STRING           = 1 << 15;
        const _BYTES            = 1 << 16;
        const OPTION           = 1 << 17;
        const RESULT           = 1 << 18;
        const _LIST             = 1 << 19;
        const DE                = 1 << 20;
        const SER               = 1 << 21;
        const STR_UTIL          = 1 << 22;

        const VARINT            = Self::_VARINT.bits() | Self::VARINT_MAX.bits();
        const U16               = Self::_U16.bits() | Self::VARINT.bits();
        const U32               = Self::_U32.bits() | Self::VARINT.bits();
        const U64               = Self::_U64.bits() | Self::VARINT.bits();
        const S16               = Self::_S16.bits() | Self::VARINT.bits();
        const S32               = Self::_S32.bits() | Self::VARINT.bits();
        const S64               = Self::_S64.bits() | Self::VARINT.bits();
        const CHAR              = Self::_CHAR.bits() | Self::U64.bits() | Self::STR_UTIL.bits();
        const STRING            = Self::_STRING.bits() | Self::U64.bits() | Self::STR_UTIL.bits();
        const BYTES             = Self::_BYTES.bits() | Self::U64.bits();
        const LIST              = Self::_LIST.bits() | Self::U64.bits();
    }
}

impl std::fmt::Display for SerdeUtils {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_str(include_str!("./js/deserializer.js"))?;

        if self.contains(SerdeUtils::VARINT_MAX) {
            f.write_str(include_str!("./js/varint_max.js"))?;
        }

        if self.contains(SerdeUtils::DE) {
            f.write_str(include_str!("./js/deserializer.js"))?;
        }

        if self.contains(SerdeUtils::VARINT | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_varint.js"))?;
        }

        if self.contains(SerdeUtils::BOOl | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_bool.js"))?;
        }

        if self.contains(SerdeUtils::U8 | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_u8.js"))?;
        }

        if self.contains(SerdeUtils::_U16 | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_u16.js"))?;
        }

        if self.contains(SerdeUtils::_U32 | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_u32.js"))?;
        }

        if self.contains(SerdeUtils::_U64 | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_u64.js"))?;
        }

        if self.contains(SerdeUtils::S8 | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_s8.js"))?;
        }

        if self.contains(SerdeUtils::_S16 | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_s16.js"))?;
        }

        if self.contains(SerdeUtils::_S32 | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_s32.js"))?;
        }

        if self.contains(SerdeUtils::_S64 | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_s64.js"))?;
        }

        if self.contains(SerdeUtils::F32 | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_f32.js"))?;
        }

        if self.contains(SerdeUtils::F64 | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_f64.js"))?;
        }

        if self.contains(SerdeUtils::_CHAR | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_char.js"))?;
        }

        if self.contains(SerdeUtils::_STRING | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_string.js"))?;
        }

        if self.contains(SerdeUtils::_BYTES | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_bytes.js"))?;
        }

        if self.contains(SerdeUtils::OPTION | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_option.js"))?;
        }

        if self.contains(SerdeUtils::RESULT | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_result.js"))?;
        }

        if self.contains(SerdeUtils::_LIST | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_list.js"))?;
        }

        if self.contains(SerdeUtils::SER) {
            f.write_str(include_str!("./js/serializer.js"))?;
        }

        if self.contains(SerdeUtils::VARINT | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_varint.js"))?;
        }

        if self.contains(SerdeUtils::BOOl | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_bool.js"))?;
        }

        if self.contains(SerdeUtils::U8 | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_u8.js"))?;
        }

        if self.contains(SerdeUtils::_U16 | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_u16.js"))?;
        }

        if self.contains(SerdeUtils::_U32 | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_u32.js"))?;
        }

        if self.contains(SerdeUtils::_U64 | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_u64.js"))?;
        }

        if self.contains(SerdeUtils::S8 | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_s8.js"))?;
        }

        if self.contains(SerdeUtils::_S16 | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_s16.js"))?;
        }

        if self.contains(SerdeUtils::_S32 | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_s32.js"))?;
        }

        if self.contains(SerdeUtils::_S64 | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_s64.js"))?;
        }

        if self.contains(SerdeUtils::F32 | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_f32.js"))?;
        }

        if self.contains(SerdeUtils::F64 | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_f64.js"))?;
        }

        if self.contains(SerdeUtils::_CHAR | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_char.js"))?;
        }

        if self.contains(SerdeUtils::_STRING | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_string.js"))?;
        }

        if self.contains(SerdeUtils::_BYTES | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_bytes.js"))?;
        }

        if self.contains(SerdeUtils::OPTION | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_option.js"))?;
        }

        if self.contains(SerdeUtils::RESULT | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_result.js"))?;
        }

        if self.contains(SerdeUtils::_LIST | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_list.js"))?;
        }

        if self.contains(SerdeUtils::STR_UTIL | SerdeUtils::DE) {
            f.write_str("const __text_decoder = new TextDecoder('utf-8');\n")?;
        }

        if self.contains(SerdeUtils::STR_UTIL | SerdeUtils::SER) {
            f.write_str("const __text_encoder = new TextEncoder();\n")?;
        }

        Ok(())
    }
}

impl SerdeUtils {
    #[must_use]
    pub fn collect_from_functions(typedefs: &TypeDefArena, functions: &[Function]) -> Self {
        let mut info = Self::empty();

        for func in functions {
            for (_, ty) in &func.params {
                info |= SerdeUtils::SER;
                info |= Self::collect_type_info(typedefs, ty);
            }

            match &func.result {
                Some(FunctionResult::Anon(ty)) => {
                    info |= SerdeUtils::DE;
                    info |= Self::collect_type_info(typedefs, ty);
                }
                Some(FunctionResult::Named(results)) => {
                    for (_, ty) in results {
                        info |= SerdeUtils::DE;
                        info |= Self::collect_type_info(typedefs, ty);
                    }
                }
                None => {}
            }
        }

        info
    }

    fn collect_typedef_info(typedefs: &TypeDefArena, id: TypeDefId) -> SerdeUtils {
        let mut info = SerdeUtils::empty();
        match &typedefs[id].kind {
            TypeDefKind::Alias(ty) => {
                info |= Self::collect_type_info(typedefs, ty);
            }
            TypeDefKind::Record(fields) => {
                for field in fields {
                    info |= Self::collect_type_info(typedefs, &field.ty);
                }
            }
            TypeDefKind::Variant(cases) => {
                info |= SerdeUtils::U32;
                for case in cases {
                    if let Some(ty) = &case.ty {
                        info |= Self::collect_type_info(typedefs, ty);
                    }
                }
            }
            TypeDefKind::Union(cases) => {
                info |= SerdeUtils::U32;
                for case in cases {
                    info |= Self::collect_type_info(typedefs, &case.ty);
                }
            }
            TypeDefKind::Enum(_) => {
                info |= SerdeUtils::U32;
            }
            TypeDefKind::Flags(fields) => {
                info |= match flags_repr(fields) {
                    wit_parser::Int::U8 => SerdeUtils::U8,
                    wit_parser::Int::U16 => SerdeUtils::U16,
                    wit_parser::Int::U32 => SerdeUtils::U32,
                    wit_parser::Int::U64 => SerdeUtils::U64,
                };
            }
            TypeDefKind::Resource(_) => {}
        }

        log::debug!("collected info for {:?}: {:?}", typedefs[id].ident, info,);

        info
    }

    fn collect_type_info(typedefs: &TypeDefArena, ty: &Type) -> SerdeUtils {
        match ty {
            Type::Bool => SerdeUtils::BOOl,
            Type::U8 => SerdeUtils::U8,
            Type::U16 => SerdeUtils::U16,
            Type::U32 => SerdeUtils::U32,
            Type::U64 => SerdeUtils::U64,
            Type::S8 => SerdeUtils::S8,
            Type::S16 => SerdeUtils::S16,
            Type::S32 => SerdeUtils::S32,
            Type::S64 => SerdeUtils::S64,
            Type::Float32 => SerdeUtils::F32,
            Type::Float64 => SerdeUtils::F64,
            Type::Char => SerdeUtils::CHAR,
            Type::String => SerdeUtils::STRING,
            Type::Tuple(types) => types
                .iter()
                .map(|ty| Self::collect_type_info(typedefs, ty))
                .collect(),
            Type::List(ty) if **ty == Type::U8 => SerdeUtils::BYTES,
            Type::List(ty) => SerdeUtils::LIST | Self::collect_type_info(typedefs, ty),
            Type::Option(ty) => SerdeUtils::OPTION | Self::collect_type_info(typedefs, ty),
            Type::Result { ok, err } => {
                let ok = ok.as_ref().map_or(SerdeUtils::empty(), |ty| {
                    Self::collect_type_info(typedefs, ty)
                });
                let err = err.as_ref().map_or(SerdeUtils::empty(), |ty| {
                    Self::collect_type_info(typedefs, ty)
                });

                SerdeUtils::RESULT | ok | err
            }
            Type::Id(id) => Self::collect_typedef_info(typedefs, *id),
        }
    }
}
