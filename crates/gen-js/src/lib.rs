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
            Type::Bool => "deserializeBool(de)".to_string(),
            Type::U8 => "deserializeU8(de)".to_string(),
            Type::U16 => "deserializeU16(de)".to_string(),
            Type::U32 => "deserializeU32(de)".to_string(),
            Type::U64 => "deserializeU64(de)".to_string(),
            Type::S8 => "deserializeS8(de)".to_string(),
            Type::S16 => "deserializeS16(de)".to_string(),
            Type::S32 => "deserializeS32(de)".to_string(),
            Type::S64 => "deserializeS64(de)".to_string(),
            Type::Float32 => "deserializeF32(de)".to_string(),
            Type::Float64 => "deserializeF64(de)".to_string(),
            Type::Char => "deserializeChar(de)".to_string(),
            Type::String => "deserializeString(de)".to_string(),
            Type::Tuple(types) => {
                let types = types
                    .iter()
                    .map(|ty| self.print_deserialize_ty(ty))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("[{types}]")
            }
            Type::List(ty) if **ty == Type::U8 => "deserializeBytes(de)".to_string(),
            Type::List(ty) => {
                let inner = self.print_deserialize_ty(ty);
                format!("deserializeList(de, (de) => {inner})")
            }
            Type::Option(ty) => {
                let ty = self.print_deserialize_ty(ty);
                format!("deserializeOption(de, (de) => {ty})")
            }
            Type::Result { ok, err } => {
                let ok = ok
                    .as_ref()
                    .map_or("() => {}".to_string(), |ty| self.print_deserialize_ty(ty));
                let err = err
                    .as_ref()
                    .map_or("() => {}".to_string(), |ty| self.print_deserialize_ty(ty));

                format!("deserializeResult(de, {ok}, {err})")
            }
            Type::Id(id) => {
                if let TypeDefKind::Resource(_) = self.interface().typedefs[*id].kind {
                    format!(
                        "{}.deserialize(de)",
                        self.interface().typedefs[*id].ident.to_upper_camel_case()
                    )
                } else {
                    format!(
                        "deserialize{}(de)",
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
            r#"function deserialize{ident}(de) {{
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
            r#"function deserialize{ident}(de) {{
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
            r#"function deserialize{ident}(de) {{
                return deserialize{inner}(de)
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
            r#"function deserialize{ident}(de) {{
                const tag = deserializeU32(de)

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
            r#"function deserialize{ident}(de) {{
                const tag = deserializeU32(de)

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
            r#"function deserialize{ident}(de) {{
                const tag = deserializeU32(de)

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
            Type::Bool => format!("serializeBool(out, {ident})"),
            Type::U8 => format!("serializeU8(out, {ident})"),
            Type::U16 => format!("serializeU16(out, {ident})"),
            Type::U32 => format!("serializeU32(out, {ident})"),
            Type::U64 => format!("serializeU64(out, {ident})"),
            Type::S8 => format!("serializeS8(out, {ident})"),
            Type::S16 => format!("serializeS16(out, {ident})"),
            Type::S32 => format!("serializeS32(out, {ident})"),
            Type::S64 => format!("serializeS64(out, {ident})"),
            Type::Float32 => format!("serializeF32(out, {ident})"),
            Type::Float64 => format!("serializeF64(out, {ident})"),
            Type::Char => format!("serializeChar(out, {ident})"),
            Type::String => format!("serializeString(out, {ident})"),
            Type::List(ty) => {
                let inner = self.print_serialize_ty("v", ty);

                format!("serializeList(out, (out, v) => {inner}, {ident})")
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

                format!("serializeOption(out, (out, v) => {inner}, {ident})")
            }
            Type::Result { ok, err } => {
                let ok = ok
                    .as_ref()
                    .map_or("{}".to_string(), |ty| self.print_serialize_ty("v", ty));
                let err = err
                    .as_ref()
                    .map_or("{}".to_string(), |ty| self.print_serialize_ty("v", ty));

                format!("serializeResult(out, (out, v) => {ok}, (out, v) => {err}, {ident})")
            }
            Type::Id(id) => {
                if let TypeDefKind::Resource(_) = self.interface().typedefs[*id].kind {
                    format!("{}.serialize(out)", ident)
                } else {
                    format!(
                        "serialize{}(out, {ident})",
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
            "function serialize{ident}(out, val) {{
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
            "function serialize{ident}(out, val) {{
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
            r#"function serialize{ident}(out, val) {{
                return serialize{inner}(out, val)
        }}"#
        )
    }

    fn print_serialize_variant(&self, ident: &str, cases: &[VariantCase]) -> String {
        let cases = cases
            .iter()
            .enumerate()
            .map(|(tag, case)| {
                let prop_access = format!("val.{}", case.ident.to_upper_camel_case());

                let inner = case.ty.as_ref().map_or("".to_string(), |ty| {
                    self.print_serialize_ty(&prop_access, ty)
                });

                format!(
                    "if ({prop_access}) {{
                    serializeU32(out, {tag});
                    return {inner}
                }}
                "
                )
            })
            .collect::<String>();

        format!(
            r#"function serialize{ident}(out, val) {{
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
                    serializeU32(out, {tag})
                    return
            "
                )
            })
            .collect::<String>();

        format!(
            r#"function serialize{ident}(out, val) {{
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
                    serializeU32(out, {tag});
                    return {inner}
                }}
                "
                )
            })
            .collect();

        format!(
            r#"function serialize{ident}(out, val) {{
                {cases}

                throw new Error("unknown union case")
        }}"#
        )

        // if (val.Ok) {
        //     serializeU8(out, 0);
        //     return ok(out, val.Ok);
        // }

        // if (val.Err) {
        //     serializeU8(out, 1);
        //     return err(out, val.Err);
        // }

        // throw new Error('Serialize bad result');
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
        const _OPTION           = 1 << 17;
        const _RESULT           = 1 << 18;
        const _LIST             = 1 << 19;
        const DE                = 1 << 20;
        const SER               = 1 << 21;

        const VARINT            = Self::_VARINT.bits() | Self::VARINT_MAX.bits();
        const U16               = Self::_U16.bits() | Self::VARINT.bits();
        const U32               = Self::_U32.bits() | Self::VARINT.bits();
        const U64               = Self::_U64.bits() | Self::VARINT.bits();
        const S16               = Self::_S16.bits() | Self::VARINT.bits();
        const S32               = Self::_S32.bits() | Self::VARINT.bits();
        const S64               = Self::_S64.bits() | Self::VARINT.bits();
        const CHAR              = Self::_CHAR.bits() | Self::U64.bits();
        const STRING            = Self::_STRING.bits() | Self::U64.bits();
        const BYTES             = Self::_BYTES.bits() | Self::U64.bits();
        const OPTION            = Self::_OPTION.bits() | Self::U32.bits();
        const RESULT            = Self::_RESULT.bits() | Self::U32.bits();
        const LIST              = Self::_LIST.bits() | Self::U64.bits();
    }
}

impl std::fmt::Display for SerdeUtils {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(include_str!("./js/deserializer.js"))?;

        if self.contains(SerdeUtils::VARINT_MAX) {
            f.write_str(include_str!("./js/varint_max.js"))?;
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

        if self.contains(SerdeUtils::_OPTION | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_option.js"))?;
        }

        if self.contains(SerdeUtils::_RESULT | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_result.js"))?;
        }

        if self.contains(SerdeUtils::_LIST | SerdeUtils::DE) {
            f.write_str(include_str!("./js/de_list.js"))?;
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

        if self.contains(SerdeUtils::_OPTION | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_option.js"))?;
        }

        if self.contains(SerdeUtils::_RESULT | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_result.js"))?;
        }

        if self.contains(SerdeUtils::_LIST | SerdeUtils::SER) {
            f.write_str(include_str!("./js/ser_list.js"))?;
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
            _ => {}
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
