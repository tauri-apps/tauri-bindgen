#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::unused_self
)]

use heck::{ToLowerCamelCase, ToUpperCamelCase, ToSnakeCase};
use std::path::PathBuf;
use tauri_bindgen_core::{Generate, GeneratorBuilder};
use wit_parser::{
    EnumCase, FlagsField, Function, FunctionResult, Interface, RecordField, Type, TypeDefId,
    TypeDefKind, UnionCase, VariantCase, TypeDef,
};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Builder {}

impl GeneratorBuilder for Builder {
    fn build(self, interface: Interface) -> Box<dyn Generate> {
        Box::new(
            Dart {
                interface,
            }
        )
    }
}

#[derive(Debug)]
pub struct Dart {
    interface: Interface,
}

impl Dart {
    pub fn print_imports(&self) -> String {
        format!("import 'dart:core';\nimport 'dart:typed_data';\n\n")
    }

    fn print_definitions(&self) -> String {
        let result: String = format!(
            r#"
class Result<T, E> {{
    const Result._(this._ok, this._err);

    factory Result.ok(final T ok) => Result._(ok, null);
    factory Result.err(final E err) => Result._(null, err);

    final T? _ok;
    final E? _err;

    bool get hasError => err != null;
    bool get isOk => ok != null;

    dynamic get result => isOk ? _ok : _err;

    E get err {{
      if (hasError) {{
        return _err!;
      }}
      throw StateError('Result has completed with no error.');
    }}

    T get ok {{
      if (isOk) {{
        return _ok!;
      }}
      throw StateError('Result has completed with error.');
    }}

    E? get maybeErr => _err;

    T? get maybeOk => _ok;

    T unwrap() => ok;
    T unwrapOr(final T def) => _ok ?? def;
    T? unwrapOrNull() => _ok;
    T unwrapOrElse(final T Function(E err) orElse) => _ok ?? orElse(err);
    T? unwrapOrMaybeElse(final T? Function(E err) orElse) => _ok ?? orElse(err);
}}
"#
        );
        format!("typedef EmptyTuple = ();\n\n{result}")
    }

    pub fn postprocess<I, S>(
        cmd: impl AsRef<std::ffi::OsStr>,
        args: I,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let mut child: std::process::Child = std::process::Command::new(cmd)
            .args(args)
            .spawn()?;
        let status: std::process::ExitStatus = child.wait()?;
        assert!(status.success());

        Ok(())
    }

    pub fn print_function(&self, func: &Function) -> String {
        let docs: String = print_docs(&func.docs);

        let ident: String = func.ident.to_lower_camel_case();

        let params: String = self.print_function_params(&func.params);
        let params: String = match params.len() == 0 {
            true => format!("{params}"),
            false => format!("{{{params}}}"),
        };
        let result: String = func
            .result
            .as_ref()
            .map(|result| self.print_function_result(result))
            .unwrap_or(String::from("Future<void>"));

        format!("{docs}{result} {ident}({params}) async => throw UnimplementedError();\n\n")
    }

    fn print_function_params(&self, params: &[(String, Type)]) -> String {
        params
            .iter()
            .map(|(ident, ty)| {
                let ident: String = ident.to_lower_camel_case();
                let is_required: bool = match ty {
                    Type::Option(_) => false,
                    _ => true,
                };

                let ty: String = self.print_type(ty);

                if is_required {
                    format!("required final {ty} {ident},")
                } else {
                    format!("final {ty} {ident},")
                }
            })
            .collect::<Vec<_>>()
            .join("")
    }

    fn print_function_result(&self, result: &FunctionResult) -> String {
        match result.len() {
            0 => String::from("Future<void>"),
            1 => {
                let ty: String = self.print_type(result.types().next().unwrap());
                format!("Future<{ty}>")
            }
            _ => {
                let tys: Vec<String> = result
                    .types()
                    .map(|ty| self.print_type(ty))
                    .collect::<Vec<_>>();
                let tys: String = match tys.len() > 1 {
                    true => {
                        let joined: String = tys.join(", ");
                        format!("({joined})") // as tuple
                    },
                    false => tys.join("")
                };
                format!("Future<{tys}>")
            }
        }
    }

    fn print_type(&self, ty: &Type) -> String {
        match ty {
            Type::Bool => "bool".to_string(),
            Type::U8
            | Type::U16
            | Type::U32
            | Type::S8
            | Type::S16
            | Type::S32
            | Type::U64
            | Type::S64 => "int".to_string(),
            Type::Float32
            | Type::Float64 => "double".to_string(),
            Type::Char | Type::String => "String".to_string(),
            Type::Tuple(types) => {
                let types: Vec<String> = types
                    .iter()
                    .map(|ty| self.print_type(ty))
                    .collect::<Vec<_>>();
                let joined: String = types.join(",");
                let types: String = match types.len() == 1 {
                    true => format!("{joined},"),
                    false => format!("{joined}")
                };
                // Not supported yet - will be supported by the summer of 2023 when dart 3.0.0 will be released
                // How to handle until then?

                format!("({types})")
            }
            Type::List(ty) => {
                let def: String = self.print_type(ty);
                let def: String = format!("List<{def}>");
                let ty: String = self.array_ty(ty).unwrap_or(def);
                format!("{ty}")
            }
            Type::Option(ty) => {
                let empty_tuple: String = String::from("()");
                let ty: String = self.print_type(ty);

                if ty.cmp(&empty_tuple) == std::cmp::Ordering::Equal || ty.cmp(&format!("{empty_tuple}?")) == std::cmp::Ordering::Equal {
                    return String::from("EmptyTuple?")
                }

                if ty.contains('?') {
                    return ty
                }

                format!("{ty}?")
            }
            Type::Result { ok, err } => {
                let ok: String = ok
                    .as_ref()
                    .map_or("void".to_string(), |ty| self.print_type(ty));
                let err: String = err
                    .as_ref()
                    .map_or("void".to_string(), |ty| self.print_type(ty));

                format!("Result<{ok}, {err}>")
            }
            Type::Id(id) => self.interface.typedefs[*id].ident.to_upper_camel_case(),
        }
    }

    fn print_typedef(&self, id: TypeDefId) -> String {
        let typedef: &TypeDef = &self.interface.typedefs[id];
        let ident: &String = &typedef.ident.to_upper_camel_case();
        let docs: String = print_docs(&typedef.docs);

        match &typedef.kind {
            TypeDefKind::Alias(ty) => self.print_alias(&docs, ident, ty),
            TypeDefKind::Record(fields) => self.print_record(&docs, ident, fields),
            TypeDefKind::Flags(fields) => self.print_flags(&docs, ident, fields),
            TypeDefKind::Variant(cases) => self.print_variant(&docs, ident, cases),
            TypeDefKind::Enum(cases) => self.print_enum(&docs, ident, cases),
            TypeDefKind::Union(cases) => self.print_union(&docs, ident, cases),
            TypeDefKind::Resource(functions) => self.print_resource(&docs, ident, functions),
        }
    }

    fn print_alias(&self, docs: &str, ident: &str, ty: &Type) -> String {
        let ty: String = self.print_type(ty);

        format!("{docs}typedef {ident} = {ty};\n")
    }

    fn print_record(&self, docs: &str, ident: &str, fields: &[RecordField]) -> String {
        let fields: String = fields
            .iter()
            .map(|field| {
                let docs: String = print_docs(&field.docs);
                let ident: String = field.ident.to_lower_camel_case();
                let ty: String = self.print_type(&field.ty);

                // By making every field a getter we allow the user to use the mixin
                // anyway intented. For example, if the user wants the fields to be
                // unmodifiable, he/she can override them as final fields.
                // On the other hand, this also gives the user the advantage of not
                // implementing fields that aren't going to be used in a specific use case.
                format!("{docs}{ty} get {ident} => throw UnimplementedError();\n")
            })
            .collect();

        format!("{docs}mixin {ident} {{{fields}}}\n")
    }

    fn print_flags(&self, docs: &str, ident: &str, fields: &[FlagsField]) -> String {
        let fields: String = fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let docs = print_docs(&field.docs);
                let ident = field.ident.to_lower_camel_case();
                let value: u64 = 2 << i;
                let max: u64 = 9_223_372_036_854_775_807;
                let value = match max > value {
                    true => value,
                    false => max,
                };
                // 9223372036854775807 is the biggest representable integer in dart
                // for numbers larger than that, BigInt should be used.
                // Unfortunately, BigInt cannot be used as an enum parameter since it has not any
                // const constructor
                // NOTE: This might still have issues on web, where the largest representable integer is
                // (2 ** 53) - 1

                if i == fields.len() - 1 {
                    format!("{docs}{ident}({value});")
                } else {
                    format!("{docs}{ident}({value}),\n")
                }
            })
            .collect();

        format!("{docs}enum {ident} {{{fields}const {ident}(this.flag);final int flag;}}")
    }

    // requires dart 3
    fn print_variant(&self, docs: &str, ident: &str, cases: &[VariantCase]) -> String {
        let interfaces: String = cases
            .iter()
            .enumerate()
            .map(
                |(i, case)| {
                    let docs: String = print_docs(&case.docs);
                    let case_ident: String = case.ident.to_upper_camel_case();
                    let ty: String = case
                        .ty
                        .as_ref()
                        .map(
                            |ty| {
                                let str_ty: String = self.print_type(ty);
                                let ty: String = match ty {
                                    Type::Option(_) => String::from(format!("{str_ty}")),
                                    _ => str_ty,
                                };
                                format!("{ty}")
                            },
                        )
                        .unwrap_or(String::from("Never"));

                    format!(
                        r#"
{docs}
final class {ident}{case_ident} extends {ident}<{ty}> {{
  const {ident}{case_ident}([super.value]);

  @override
  int get tag => {i};

  @override
  {ident}{case_ident} copyWith([final {ty}? value]) => {ident}{case_ident}(
    value ?? super.value,
  );
}}
"#
                    )
                },
            )
            .collect();

        format!(
            r#"
{interfaces}
{docs}
sealed class {ident}<T> {{
  const {ident}([this.value]);

  final T? value;

  abstract final int tag;

  {ident} copyWith([final T? value]);
}}
"#
        )
    }

    fn print_enum(&self, docs: &str, ident: &str, cases: &[EnumCase]) -> String {
        let cases: String = cases
            .iter()
            .map(|case| {
                let docs = print_docs(&case.docs);
                let ident = case.ident.to_lower_camel_case();

                format!("{docs}{ident},\n")
            })
            .collect();

        format!("{docs}enum {ident} {{ {cases} }}\n")
    }

    fn print_union(&self, _docs: &str, _ident: &str, _cases: &[UnionCase]) -> ! {
        panic!("Union types are not supported by Dart.")
    }

    fn print_resource(&self, docs: &str, ident: &str, functions: &[Function]) -> String {
        let functions: String = functions
            .iter()
            .map(|func| {
                let docs = print_docs(&func.docs);

                let ident = func.ident.to_lower_camel_case();

                let params = self.print_function_params(&func.params);
                let params = match params.len() == 0 {
                    true => format!("{params}"),
                    false => format!("{{{params}}}"),
                };
                let result = func
                    .result
                    .as_ref()
                    .map(|result| self.print_function_result(result))
                    .unwrap_or_default();

                format!("{docs}{result} {ident}({params}) async => throw UnimplementedError();\n")
            })
            .collect();

        format!(
            r#"
{docs}
class {ident} {{
  const {ident}({{final int? id}}) : _id = id;

  final int? _id;

  {functions}
}}
"#
        )
    }

    fn array_ty(&self, ty: &Type) -> Option<String> {
        match ty {
            Type::U8 => Some("Uint8List".to_string()),
            Type::S8 => Some("Int8List".to_string()),
            Type::U16 => Some("Uint16List".to_string()),
            Type::S16 => Some("Int16List".to_string()),
            Type::U32 => Some("Uint32List".to_string()),
            Type::S32 => Some("Int32List".to_string()),
            Type::U64 => Some("Uint64List".to_string()),
            Type::S64 => Some("Int64List".to_string()),
            Type::Float32 => Some("Float32List".to_string()),
            Type::Float64 => Some("Float64List".to_string()),
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
}

fn print_docs(docs: &str) -> String {
    if docs.is_empty() {
        return String::new();
    }

    let docs = docs
        .lines()
        .map(|line| format!("/// {line} \n"))
        .collect::<String>();

    format!("{docs}")
}

impl Generate for Dart {
    fn to_file(&self) -> (std::path::PathBuf, String) {
        let result_ty: &str = self
            .interface
            .functions
            .iter()
            .any(Function::throws)
            .then_some(
                // "export type Result<T, E> = { tag: 'ok', val: T } | { tag: 'err', val: E };\n",
                "",
            )
            .unwrap_or_default();

        let typedefs: String = self
            .interface
            .typedefs
            .iter()
            .map(|(id, _)| self.print_typedef(id))
            .collect();

        let functions: String = self
            .interface
            .functions
            .iter()
            .map(|func| self.print_function(func))
            .collect();

        let imports: String = self.print_imports();
        let definitions: String = self.print_definitions();

        let contents: String = format!("{imports}\n{definitions}\n{result_ty}\n{typedefs}\n{functions}");

        let mut filename: PathBuf = PathBuf::from(self.interface.ident.to_snake_case());
        filename.set_extension("dart");

        (filename, contents)
    }
}
