use heck::{ToLowerCamelCase, ToUpperCamelCase};
use tauri_bindgen_core::{flags_repr, TypeInfos};
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

    #[allow(clippy::too_many_lines)]
    fn print_deserialize_ty(&self, ty: &Type) -> String {
        match ty {
            Type::Bool => "deserializeBoolean(de)".to_string(),
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

                format!(
                    "case {tag}:
                return {{ tag: {tag}, value: {inner} }}
            "
                )
            })
            .collect::<String>();

        format!(
            r#"function deserialize{ident}(de) {{
                const disc = deserializeU32(de)

                switch (disc) {{
                    {cases}
                    default:
                        throw new Error("unknown variant case")
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
                const disc = deserializeU32(de)

                switch (disc) {{
                    {cases}
                    default:
                        throw new Error("unknown enum case")
                }}
        }}"#
        )
    }

    fn print_deserialize_union(&self, ident: &str, cases: &[UnionCase]) -> String {
        let cases = cases
            .iter()
            .enumerate()
            .map(|(tag, case)| {
                let inner = self.print_deserialize_ty(&case.ty);

                format!(
                    "case {tag}:
                return {inner}
            "
                )
            })
            .collect::<String>();

        format!(
            r#"function deserialize{ident}(de) {{
                const disc = deserializeU32(de)

                switch (disc) {{
                    {cases}
                    default:
                        throw new Error("unknown union case")
                }}
        }}"#
        )
    }
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct SerdeUtils: u32 {
        const TRY_TAKE_VARINT   = 1 << 1;
        const DE_BOOl           = 1 << 2;
        const DE_U8             = 1 << 3;
        const _DE_U16           = 1 << 4;
        const _DE_U32           = 1 << 5;
        const _DE_U64           = 1 << 6;
        const DE_S8             = 1 << 7;
        const _DE_S16           = 1 << 8;
        const _DE_S32           = 1 << 9;
        const _DE_S64           = 1 << 10;
        const DE_F32            = 1 << 11;
        const DE_F64            = 1 << 12;
        const _DE_CHAR          = 1 << 13;
        const _DE_STRING        = 1 << 14;
        const _DE_BYTES         = 1 << 15;
        const DE_OPTION         = 1 << 16;
        const DE_RESULT         = 1 << 17;
        const _DE_LIST          = 1 << 18;

        const DE_U16            = Self::_DE_U16.bits() | Self::TRY_TAKE_VARINT.bits();
        const DE_U32            = Self::_DE_U32.bits() | Self::TRY_TAKE_VARINT.bits();
        const DE_U64            = Self::_DE_U64.bits() | Self::TRY_TAKE_VARINT.bits();
        const DE_S16            = Self::_DE_S16.bits() | Self::TRY_TAKE_VARINT.bits();
        const DE_S32            = Self::_DE_S32.bits() | Self::TRY_TAKE_VARINT.bits();
        const DE_S64            = Self::_DE_S64.bits() | Self::TRY_TAKE_VARINT.bits();

        const DE_CHAR           = Self::_DE_CHAR.bits() | Self::DE_U64.bits();
        const DE_STRING         = Self::_DE_STRING.bits() | Self::DE_U64.bits();
        const DE_BYTES          = Self::_DE_BYTES.bits() | Self::DE_U64.bits();
        const DE_LIST           = Self::_DE_LIST.bits() | Self::DE_U64.bits();
    }
}

impl std::fmt::Display for SerdeUtils {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(include_str!("./js/deserializer.js"))?;

        if self.contains(SerdeUtils::TRY_TAKE_VARINT) {
            f.write_str(include_str!("./js/try_take_varint.js"))?;
        }

        if self.contains(SerdeUtils::DE_BOOl) {
            f.write_str(include_str!("./js/de_bool.js"))?;
        }

        if self.contains(SerdeUtils::DE_U8) {
            f.write_str(include_str!("./js/de_u8.js"))?;
        }

        if self.contains(SerdeUtils::_DE_U16) {
            f.write_str(include_str!("./js/de_u16.js"))?;
        }

        if self.contains(SerdeUtils::_DE_U32) {
            f.write_str(include_str!("./js/de_u32.js"))?;
        }

        if self.contains(SerdeUtils::_DE_U64) {
            f.write_str(include_str!("./js/de_u64.js"))?;
        }

        if self.contains(SerdeUtils::DE_S8) {
            f.write_str(include_str!("./js/de_s8.js"))?;
        }

        if self.contains(SerdeUtils::_DE_S16) {
            f.write_str(include_str!("./js/de_s16.js"))?;
        }

        if self.contains(SerdeUtils::_DE_S32) {
            f.write_str(include_str!("./js/de_s32.js"))?;
        }

        if self.contains(SerdeUtils::_DE_S64) {
            f.write_str(include_str!("./js/de_s64.js"))?;
        }

        if self.contains(SerdeUtils::DE_F32) {
            f.write_str(include_str!("./js/de_f32.js"))?;
        }

        if self.contains(SerdeUtils::DE_F64) {
            f.write_str(include_str!("./js/de_f64.js"))?;
        }

        if self.contains(SerdeUtils::_DE_CHAR) {
            f.write_str(include_str!("./js/de_char.js"))?;
        }

        if self.contains(SerdeUtils::_DE_STRING) {
            f.write_str(include_str!("./js/de_string.js"))?;
        }

        if self.contains(SerdeUtils::_DE_BYTES) {
            f.write_str(include_str!("./js/de_bytes.js"))?;
        }

        if self.contains(SerdeUtils::DE_OPTION) {
            f.write_str(include_str!("./js/de_option.js"))?;
        }

        if self.contains(SerdeUtils::DE_RESULT) {
            f.write_str(include_str!("./js/de_result.js"))?;
        }

        if self.contains(SerdeUtils::_DE_LIST) {
            f.write_str(include_str!("./js/de_list.js"))?;
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
                info |= Self::collect_type_info(typedefs, ty);
            }

            match &func.result {
                Some(FunctionResult::Anon(ty)) => {
                    info |= Self::collect_type_info(typedefs, ty);
                }
                Some(FunctionResult::Named(results)) => {
                    for (_, ty) in results {
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
                for case in cases {
                    if let Some(ty) = &case.ty {
                        info |= Self::collect_type_info(typedefs, ty);
                    }
                }
            }
            TypeDefKind::Union(cases) => {
                for case in cases {
                    info |= Self::collect_type_info(typedefs, &case.ty);
                }
            }
            _ => {}
        }

        log::debug!("collected info for {:?}: {:?}", typedefs[id].ident, info,);

        info
    }

    fn collect_type_info(typedefs: &TypeDefArena, ty: &Type) -> SerdeUtils {
        match ty {
            Type::Bool => SerdeUtils::DE_BOOl,
            Type::U8 => SerdeUtils::DE_U8,
            Type::U16 => SerdeUtils::DE_U16,
            Type::U32 => SerdeUtils::DE_U32,
            Type::U64 => SerdeUtils::DE_U64,
            Type::S8 => SerdeUtils::DE_S8,
            Type::S16 => SerdeUtils::DE_S16,
            Type::S32 => SerdeUtils::DE_S32,
            Type::S64 => SerdeUtils::DE_S64,
            Type::Float32 => SerdeUtils::DE_F32,
            Type::Float64 => SerdeUtils::DE_F64,
            Type::Char => SerdeUtils::DE_CHAR,
            Type::String => SerdeUtils::DE_STRING,
            Type::Tuple(types) => types
                .iter()
                .map(|ty| Self::collect_type_info(typedefs, ty))
                .collect(),
            Type::List(ty) if **ty == Type::U8 => SerdeUtils::DE_BYTES,
            Type::List(ty) => SerdeUtils::DE_LIST | Self::collect_type_info(typedefs, ty),
            Type::Option(ty) => SerdeUtils::DE_OPTION | Self::collect_type_info(typedefs, ty),
            Type::Result { ok, err } => {
                let ok = ok.as_ref().map_or(SerdeUtils::empty(), |ty| {
                    Self::collect_type_info(typedefs, ty)
                });
                let err = err.as_ref().map_or(SerdeUtils::empty(), |ty| {
                    Self::collect_type_info(typedefs, ty)
                });

                SerdeUtils::DE_RESULT | ok | err
            }
            Type::Id(id) => Self::collect_typedef_info(typedefs, *id),
        }
    }
}
