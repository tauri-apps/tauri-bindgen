#![allow(clippy::must_use_candidate, clippy::unused_self)]

use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use std::{fmt::Write, path::PathBuf, sync::atomic::{AtomicU32, Ordering}};
use tauri_bindgen_core::{
    flags_repr, postprocess, Generate, GeneratorBuilder, TypeInfo, TypeInfos,
};
use wit_parser::{
    EnumCase, FlagsField, Function, FunctionResult, Interface, RecordField, Type, TypeDefId,
    TypeDefKind, UnionCase, VariantCase,
};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
#[cfg_attr(feature = "clap", clap(group(
    clap::ArgGroup::new("fmt")
        .args(&["prettier", "romefmt"]),
)))]
pub struct Builder {
    /// Run `prettier` to format the generated code. This requires a global installation of `prettier`.
    #[cfg_attr(feature = "clap", clap(long))]
    pub prettier: bool,
    /// Run `rome format` to format the generated code. This formatter is much faster that `prettier`. Requires a global installation of `prettier`.
    #[cfg_attr(feature = "clap", clap(long))]
    pub romefmt: bool,
}

impl GeneratorBuilder for Builder {
    fn build(self, interface: Interface) -> Box<dyn Generate> {
        let mut infos = TypeInfos::new();

        for func in &interface.functions {
            infos.collect_param_info(&interface.typedefs, &func.params);
            if let Some(result) = &func.result {
                infos.collect_result_info(&interface.typedefs, result);
            }
        }

        for (id, typedef) in &interface.typedefs {
            log::debug!("type info: {} {:#?}", typedef.ident, infos[id]);
        }

        Box::new(JavaScript {
            opts: self,
            interface,
            infos,
            serde_utils: 0.into(),
        })
    }
}

#[derive(Debug)]
pub struct JavaScript {
    opts: Builder,
    interface: Interface,
    infos: TypeInfos,
    // TODO: this is really awkward
    serde_utils: AtomicU32,
}

impl JavaScript {
    fn print_function(&self, intf_name: &str, func: &Function) -> String {
        let docs = self.print_docs(func);
        let ident = func.ident.to_lower_camel_case();
        let name = func.ident.to_snake_case();
        let params = print_function_params(&func.params);

        let deserialize_result = func
            .result
            .as_ref()
            .map(|res| self.print_deserialize_function_result(&res))
            .unwrap_or_default();

        format!(
            r#"
            {docs}
            export async function {ident} ({params}) {{
                return fetch('ipc://localhost/{intf_name}/{name}', {{ method: "POST", body: JSON.stringify([{params}]) }}){deserialize_result}
            }}
        "#
        )
    }

    fn print_deserialize_function_result(&self, result: &FunctionResult) -> String {
        match result.len() {
            0 => String::new(),
            1 => {
                let inner = self.print_deserialize_ty(result.types().next().unwrap());

                format!(
                    "
                .then(r => r.arrayBuffer())
                .then(bytes => {{
                    const de = new Deserializer(Uint8Array.from(bytes))

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

    fn print_resource(
        &self,
        docs: &str,
        ident: &str,
        functions: &[Function],
        info: TypeInfo,
    ) -> String {
        let ident = ident.to_upper_camel_case();

        let functions: String = functions
            .iter()
            .map(|func| {
                let docs = self.print_docs(func);
                let ident = func.ident.to_lower_camel_case();
                let params = print_function_params(&func.params);

                format!(
                    r#"
                {docs}
                async {ident} ({params}) {{
                }}
            "#
                )
            })
            .collect();

        let deserialize = if info.contains(TypeInfo::RESULT) {
            format!(
                "deserialize(de) {{
                            const self = new {ident}();
                            self.#id = deserializeU64(de);
                            return self
                        }}"
            )
        } else {
            String::new()
        };

        format!(
            "{docs}\nclass {ident} {{
            #id;
            {functions}
            {deserialize}
        }}"
        )
    }

    fn print_docs(&self, func: &Function) -> String {
        let docs = func
            .docs
            .lines()
            .map(|line| format!(" * {line} \n"))
            .collect::<String>();

        let param_docs = func
            .params
            .iter()
            .map(|(name, ty)| {
                let ident = &name.to_lower_camel_case();
                let ty = self.print_ty(ty);

                format!("* @param {{{ty}}} {ident} \n")
            })
            .collect::<String>();

        let result_docs = func
            .result
            .as_ref()
            .map(|result| match result {
                FunctionResult::Anon(ty) => {
                    let ty = self.print_ty(ty);
                    format!("* @returns {{Promise<{ty}>}} \n")
                }
                FunctionResult::Named(types) => {
                    let types = types
                        .iter()
                        .map(|(_, ty)| self.print_ty(ty))
                        .collect::<Vec<_>>()
                        .join(", ");

                    format!("* @returns {{Promise<[{types}]>}} \n")
                }
            })
            .unwrap_or_default();

        format!("/**\n{docs}{param_docs}{result_docs}*/")
    }

    fn print_ty(&self, ty: &Type) -> String {
        match ty {
            Type::Bool => "boolean".to_string(),
            Type::U8
            | Type::U16
            | Type::U32
            | Type::S8
            | Type::S16
            | Type::S32
            | Type::Float32
            | Type::Float64 => "number".to_string(),
            Type::U64 | Type::S64 => "bigint".to_string(),
            Type::Char | Type::String => "string".to_string(),
            Type::Tuple(types) => {
                let types = types
                    .iter()
                    .map(|ty| self.print_ty(ty))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("[{types}]")
            }
            Type::List(ty) => {
                let ty = self.array_ty(ty).unwrap_or(self.print_ty(ty));
                format!("{ty}[]")
            }
            Type::Option(ty) => {
                let ty = self.print_ty(ty);

                format!("{ty} | null")
            }
            Type::Result { ok, err } => {
                let ok = ok.as_ref().map_or("_".to_string(), |ty| self.print_ty(ty));
                let err = err.as_ref().map_or("_".to_string(), |ty| self.print_ty(ty));

                format!("Result<{ok}, {err}>")
            }
            Type::Id(id) => self.interface.typedefs[*id].ident.to_upper_camel_case(),
        }
    }

    fn array_ty(&self, ty: &Type) -> Option<String> {
        match ty {
            Type::U8 => Some("Uint8Array".to_string()),
            Type::S8 => Some("Int8Array".to_string()),
            Type::U16 => Some("Uint16Array".to_string()),
            Type::S16 => Some("Int16Array".to_string()),
            Type::U32 => Some("Uint32Array".to_string()),
            Type::S32 => Some("Int32Array".to_string()),
            Type::U64 => Some("BigUint64Array".to_string()),
            Type::S64 => Some("BigInt64Array".to_string()),
            Type::Float32 => Some("Float32Array".to_string()),
            Type::Float64 => Some("Float64Array".to_string()),
            Type::Id(id) => match &self.interface.typedefs[*id].kind {
                TypeDefKind::Alias(t) => self.array_ty(t),
                _ => None,
            },
            Type::Bool
            | Type::Tuple(_)
            | Type::List(_)
            | Type::Option(_)
            | Type::Result { .. }
            | Type::Char
            | Type::String => None,
        }
    }

    fn print_deserialize_ty(&self, ty: &Type) -> String {
        match ty {
            Type::Bool => {
                self.serde_utils.fetch_or(SerdeUtils::DE_BOOl.bits(), Ordering::Relaxed);
                "deserializeBoolean(de)".to_string()
            }
            Type::U8 => {
                self.serde_utils.fetch_or(SerdeUtils::DE_U8.bits(), Ordering::Relaxed);
                "deserializeU8(de)".to_string()
            }
            Type::U16 => {
                self.serde_utils.fetch_or(SerdeUtils::DE_U16.bits(), Ordering::Relaxed);
                "deserializeU16(de)".to_string()
            }
            Type::U32 => {
                self.serde_utils.fetch_or(SerdeUtils::DE_U32.bits(), Ordering::Relaxed);
                "deserializeU32(de)".to_string()
            }
            Type::U64 => {
                self.serde_utils.fetch_or(SerdeUtils::DE_U64.bits(), Ordering::Relaxed);
                "deserializeU64(de)".to_string()
            }
            Type::S8 => {
                self.serde_utils.fetch_or(SerdeUtils::DE_S8.bits(), Ordering::Relaxed);
                "deserializeS8(de)".to_string()
            }
            Type::S16 => {
                self.serde_utils.fetch_or(SerdeUtils::DE_S16.bits(), Ordering::Relaxed);
                "deserializeS16(de)".to_string()
            }
            Type::S32 => {
                self.serde_utils.fetch_or(SerdeUtils::DE_S32.bits(), Ordering::Relaxed);
                "deserializeS32(de)".to_string()
            }
            Type::S64 => {
                self.serde_utils.fetch_or(SerdeUtils::DE_S64.bits(), Ordering::Relaxed);
                "deserializeS64(de)".to_string()
            }
            Type::Float32 => {
                self.serde_utils.fetch_or(SerdeUtils::DE_F32.bits(), Ordering::Relaxed);
                "deserializeF32(de)".to_string()
            }
            Type::Float64 => {
                self.serde_utils.fetch_or(SerdeUtils::DE_F64.bits(), Ordering::Relaxed);
                "deserializeF64(de)".to_string()
            }
            Type::Char => {
                self.serde_utils.fetch_or(SerdeUtils::DE_CHAR.bits(), Ordering::Relaxed);
                "deserializeChar(de)".to_string()
            }
            Type::String => {
                self.serde_utils.fetch_or(SerdeUtils::DE_STRING.bits(), Ordering::Relaxed);
                "deserializeString(de)".to_string()
            }
            Type::Tuple(types) => {
                let types = types
                    .iter()
                    .map(|ty| self.print_deserialize_ty(ty))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("[{types}]")
            }
            Type::List(ty) if **ty == Type::U8 => {
                self.serde_utils.fetch_or(SerdeUtils::DE_BYTES.bits(), Ordering::Relaxed);
                "deserializeBytes(de)".to_string()
            }
            Type::List(ty) => {
                self.serde_utils.fetch_or(SerdeUtils::DE_LIST.bits(), Ordering::Relaxed);
                let ty = self.print_deserialize_ty(ty);
                format!("deserializeList(de, (de) => {ty})")
            }
            Type::Option(ty) => {
                self.serde_utils.fetch_or(SerdeUtils::DE_OPTION.bits(), Ordering::Relaxed);
                let ty = self.print_deserialize_ty(ty);
                format!("deserializeOption(de, (de) => {ty})")
            }
            Type::Result { ok, err } => {
                self.serde_utils.fetch_or(SerdeUtils::DE_RESULT.bits(), Ordering::Relaxed);
                let ok = ok
                    .as_ref()
                    .map_or("() => {}".to_string(), |ty| self.print_deserialize_ty(ty));
                let err = err
                    .as_ref()
                    .map_or("() => {}".to_string(), |ty| self.print_deserialize_ty(ty));

                format!("deserializeResult(de, {ok}, {err})")
            }
            Type::Id(id) => {
                if let TypeDefKind::Resource(_) = self.interface.typedefs[*id].kind {
                    format!(
                        "{}.deserialize(de)",
                        self.interface.typedefs[*id].ident.to_upper_camel_case()
                    )
                } else {
                    format!(
                        "deserialize{}(de)",
                        self.interface.typedefs[*id].ident.to_upper_camel_case()
                    )
                }
            }
        }
    }

    fn print_deserialize_typedef(&self, id: TypeDefId) -> String {
        let typedef = &self.interface.typedefs[id];
        let ident = &typedef.ident.to_upper_camel_case();

        match typedef.kind.clone() {
            TypeDefKind::Alias(ty) => self.print_deserialize_alias(ident, ty),
            TypeDefKind::Record(fields) => self.print_deserialize_record(ident, &fields),
            TypeDefKind::Flags(fields) => self.print_deserialize_flags(ident, &fields),
            TypeDefKind::Variant(cases) => self.print_deserialize_variant(ident, &cases),
            TypeDefKind::Enum(cases) => self.print_deserialize_enum(ident, &cases),
            TypeDefKind::Union(cases) => self.print_deserialize_union(ident, &cases),
            TypeDefKind::Resource(_) => String::new(),
        }
    }

    fn print_deserialize_alias(&self, ident: &str, ty: Type) -> String {
        let inner = self.print_deserialize_ty(&ty);

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
                    .map(|ty| self.print_deserialize_ty(&ty))
                    .unwrap_or("null".to_string());

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

impl Generate for JavaScript {
    fn to_file(&mut self) -> (std::path::PathBuf, String) {

        let mut deserializers = String::new();
        for (id, _) in self.interface.typedefs.iter() {
            let info = self.infos[id];

            if info.contains(TypeInfo::RESULT) {
                deserializers.push_str(&self.print_deserialize_typedef(id))
            }
        }

        let functions: String = self
            .interface
            .functions
            .iter()
            .map(|func| self.print_function(&self.interface.ident.to_snake_case(), func))
            .collect();

        let resources: String = self
            .interface
            .typedefs
            .iter()
            .filter_map(|(id, typedef)| {
                let info = self.infos[id];
                if let TypeDefKind::Resource(functions) = &typedef.kind {
                    Some(self.print_resource(&typedef.docs, &typedef.ident, functions, info))
                } else {
                    None
                }
            })
            .collect();

        let bits = self.serde_utils.load(Ordering::Acquire);

        println!("{} {:?}", bits, SerdeUtils::from_bits_retain(bits));

        let serde_util = print_serde_utils(&SerdeUtils::from_bits_retain(bits)).unwrap();

        let mut contents = format!("{serde_util}{deserializers}\n{functions}\n{resources}");

        if self.opts.prettier {
            postprocess(&mut contents, "prettier", ["--parser=babel"])
                .expect("failed to run `prettier`");
        } else if self.opts.romefmt {
            postprocess(
                &mut contents,
                "rome",
                ["format", "--stdin-file-path", "index.js"],
            )
            .expect("failed to run `rome format`");
        }

        let mut filename = PathBuf::from(self.interface.ident.to_kebab_case());
        filename.set_extension("js");

        (filename, contents)
    }
}

fn print_function_params(params: &[(String, Type)]) -> String {
    params
        .iter()
        .map(|(name, _)| name.to_lower_camel_case())
        .collect::<Vec<_>>()
        .join(", ")
}

bitflags::bitflags! {
    #[derive(Debug)]
    struct SerdeUtils: u32 {
        const VARINT_TYPE       = 1 << 0;
        const VARINT_MAX        = 1 << 1;
        const MAX_OF_LAST_BYTE  = 1 << 2;
        const _TRY_TAKE_VARINT   = 1 << 3;
        const DE_BOOl           = 1 << 4;
        const DE_U8             = 1 << 5;
        const _DE_U16           = 1 << 6;
        const _DE_U32           = 1 << 7;
        const _DE_U64           = 1 << 8;
        const DE_S8             = 1 << 9;
        const _DE_S16           = 1 << 10;
        const _DE_S32           = 1 << 11;
        const _DE_S64           = 1 << 12;
        const DE_F32            = 1 << 13;
        const DE_F64            = 1 << 14;
        const _DE_CHAR          = 1 << 15;
        const _DE_STRING        = 1 << 16;
        const _DE_BYTES         = 1 << 17;
        const DE_OPTION         = 1 << 18;
        const DE_RESULT         = 1 << 19;
        const _DE_LIST          = 1 << 20;

        const TRY_TAKE_VARINT   = Self::_TRY_TAKE_VARINT.bits() | Self::VARINT_TYPE.bits() | Self::VARINT_MAX.bits() | Self::MAX_OF_LAST_BYTE.bits();
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

fn print_serde_utils(serde_utils: &SerdeUtils) -> Result<String, std::fmt::Error> {
    let mut out = "export class Deserializer {
        constructor(bytes) {
            this.source = bytes
            this.offset = 0
        }
    
        pop() {
            return this.source[this.offset++]
        }
    
        try_take_n(len) {
            const out = this.source.slice(this.offset, this.offset + len)
            this.offset += len
            return out
        }
    }
    "
    .to_string();

    if serde_utils.contains(SerdeUtils::VARINT_MAX) {
        out.write_str(
            "function varint_max(type) {
            const BITS_PER_BYTE = 8;
            const BITS_PER_VARINT_BYTE = 7;
        
            const bits = type * BITS_PER_BYTE;
    
            const roundup_bits = bits + (BITS_PER_BYTE - 1);
        
            return Math.floor(roundup_bits / BITS_PER_VARINT_BYTE);
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::MAX_OF_LAST_BYTE) {
        out.write_str(
            "function max_of_last_byte(type) {
            let extra_bits = type % 7;
            return (1 << extra_bits) - 1;
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::_TRY_TAKE_VARINT) {
        out.write_str(
            "function try_take_varint(de, type) {
            let out = 0n;

            for (let i = 0; i < varint_max(type); i++) {
                const val = de.pop();
                const carry = BigInt(val & 0x7F);
                out |= carry << (7n * BigInt(i));
        
                if ((val & 0x80) === 0) {
                    if (i === varint_max(type) - 1 && val > max_of_last_byte(type)) {
                        throw new Error('deserialize bad variant')
                    } else {
                        return out
                    }
                }
            }
        
            throw new Error('deserialize bad variant')
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::DE_BOOl) {
        out.write_str(
            "function deserializeBool(de) {
            const val = de.pop();
        
            return val != 0
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::DE_U8) {
        out.write_str(
            "function deserializeU8(de) {
            return de.pop()
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::_DE_U16) {
        out.write_str(
            "function deserializeU16(de) {
            return try_take_varint(de, 16)
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::_DE_U32) {
        out.write_str(
            "function deserializeU32(de) {
            return try_take_varint(de, 32)
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::_DE_U64) {
        out.write_str(
            "function deserializeU64(de) {
            return try_take_varint(de, 64)
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::DE_S8) {
        out.write_str(
            "function deserializeI8(de) {
            return de.pop()
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::_DE_S16) {
        out.write_str(
            "function deserializeI16(de) {
            const n = try_take_varint(de, 16)

            return Number(((n >> 1n) & 0xFFFFn) ^ (-((n & 0b1n) & 0xFFFFn)))
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::_DE_S32) {
        out.write_str(
            "function deserializeI32(de) {
            const n = try_take_varint(de, 32)

            return Number(((n >> 1n) as & 0xFFFFFFFFn) ^ (-((n & 0b1n) as & 0xFFFFFFFFn)))
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::_DE_S64) {
        out.write_str(
            "function deserializeI64(de) {
            const n = try_take_varint(de, u64)

            return Number(((n >> 1n) & 0xFFFFFFFFFFFFFFFFn) ^ (-((n & 0b1n) & 0xFFFFFFFFFFFFFFFFn)))
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::DE_F32) {
        out.write_str(
            "function deserializeF32(de) {
            const bytes = de.try_take_n(4);

            const buf = new ArrayBuffer(4);
            const view = new DataView(buf);
        
            bytes.reverse().forEach((v, i) => view.setUint8(i, v));
        
            return view.getFloat32(0);
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::DE_F64) {
        out.write_str(
            "function deserializeF64(de) {
            const bytes = de.try_take_n(8);

            const buf = new ArrayBuffer(8);
            const view = new DataView(buf);
        
            bytes.reverse().forEach((v, i) => view.setUint8(i, v));
        
            return view.getFloat64(0);
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::_DE_CHAR) {
        out.write_str(
            r#"function deserializeChar(de) {
            const sz = deserializeU64(de);
            if (sz > 4) {
                throw new Error("Deserialize bad char");
            }
            const bytes = de.try_take_n(Number(sz));
        
            const decoder = new TextDecoder('utf-8');
        
            return decoder.decode(bytes);
        }
        "#,
        )?;
    }

    if serde_utils.contains(SerdeUtils::_DE_STRING) {
        out.write_str(
            "function deserializeString(de) {
            const sz = deserializeU64(de);
        
            let bytes = de.try_take_n(Number(sz));
        
            const decoder = new TextDecoder('utf-8');
        
            return decoder.decode(bytes);
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::_DE_BYTES) {
        out.write_str(
            "function deserializeBytes(de) {
            const sz = deserializeU64(de);
        
            let bytes = de.try_take_n(Number(sz));
        
            const decoder = new TextDecoder('utf-8');
        
            return decoder.decode(bytes);
        }
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::DE_OPTION) {
        out.write_str(
            "function deserializeOption(de, inner) {
            const disc = de.pop()
        
            switch (disc) {
                case 0:
                    return null
                case 1: 
                    return inner(de)
                default:
                    throw new Error('Deserialize bad option')
            }
        } 
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::DE_RESULT) {
        out.write_str(
            "function function deserializeResult(de, ok, err) {
            const disc = de.pop()
        
            switch (disc) {
                case 0:
                    return ok(de)
                case 1: 
                    return err(de)
                default:
                    throw new Error('Deserialize bad result')
            }
        } 
        ",
        )?;
    }

    if serde_utils.contains(SerdeUtils::_DE_LIST) {
        out.write_str(
            "function deserializeList(de, inner) {
            const len = deserializeU64(de);
        
            let out: T[] = [];

            for (let i = 0; i < len; i++) {
                out.push(inner(de));   
            }
        
            return out;
        } 
        ",
        )?;
    }

    Ok(out)
}
