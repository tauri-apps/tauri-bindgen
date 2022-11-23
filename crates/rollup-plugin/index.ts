import { Plugin } from 'rollup'
import { process, Kind } from './process'

export function tauriBindgen(kind: Kind): Plugin {
    return {
        name: 'tauri-bindgen',
        resolveId(id) {
            if (id.endsWith('.wit')) {
                return id
            }
        },
        load(id) {
            if (id.endsWith('.wit')) {
                return process(kind, id)
            }
        }
    }
}