import { EditorView, basicSetup } from "codemirror"
import { javascript } from "@codemirror/lang-javascript"
import { rust } from "@codemirror/lang-rust"
import { EditorState } from "@codemirror/state"
import { wit } from './wit'

const default_content = `interface greet {
    func greet(name: string) -> string
}`

const outputs = {
  'errors': new EditorView({
    parent: document.getElementById('errors')!,
    extensions: [basicSetup, javascript(), EditorState.readOnly.of(true)],
  }),
  'host': new EditorView({
    parent: document.getElementById('host')!,
    extensions: [basicSetup, rust(), EditorState.readOnly.of(true)],
  }),
  'guest-rust': new EditorView({
    parent: document.getElementById('guest_rust')!,
    extensions: [basicSetup, rust(), EditorState.readOnly.of(true)],
  }),
  'guest-js': new EditorView({
    parent: document.getElementById('guest_js')!,
    extensions: [basicSetup, javascript(), EditorState.readOnly.of(true)],
  }),
  'guest-ts': new EditorView({
    parent: document.getElementById('guest_ts')!,
    extensions: [basicSetup, javascript(), EditorState.readOnly.of(true)],
  }),
}

export function updateOutput(id: 'errors' | 'host' | 'guest-rust' | 'guest-js' | 'guest-ts' | 'markdown', data: string) {
  if (id === "markdown") {
    document.getElementById('markdown')!.innerHTML = data
  } else {
    outputs[id].dispatch({
      changes: {
        from: 0,
        to: outputs[id].state.doc.length,
        insert: data,
      },
    })
  }
}

export function setup(onChange: (data: string) => void) {
  const input = new EditorView({
    parent: document.getElementById('input')!,
    extensions: [basicSetup, wit(), EditorView.updateListener.of((v) => {
      if (v.docChanged) {
        console.log(v.state.doc.toString())

        onChange(v.state.doc.toString())
      }
    })],
  })

  input.dispatch({
    changes: {
      from: 0,
      to: input.state.doc.length,
      insert: default_content,
    },
  })
}
