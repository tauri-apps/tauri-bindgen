import { parser } from "./wit.grammar"
import { LRLanguage, LanguageSupport, /* foldNodeProp, foldInside, delimitedIndent */ } from "@codemirror/language"
import { styleTags, tags as t } from "@lezer/highlight"

export const witLanguage = LRLanguage.define({
  parser: parser.configure({
    props: [
      styleTags({
        Identifier: t.variableName,

        "u8 u16 u32 u64 s8 s16 s32 s64 float32 float64 char string list tuple option result": t.typeName,
        "interface type record flags variant enum union resource func": t.keyword,
        "=": t.definitionOperator,
        "-> : _ ,": t.punctuation,
        "( )": t.paren,
        "< >": t.angleBracket,
        "{ }": t.brace,
      })
    ]
  }),
  languageData: {
    commentTokens: { line: "//", block: { open: "/*", close: "*/" } },
    indentOnInput: /^\s*(?:\{|\})$/,
  }
})

export function wit() {
  return new LanguageSupport(witLanguage, [])
}
