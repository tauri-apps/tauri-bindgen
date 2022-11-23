import { process } from './process';
export function tauriBindgen(kind) {
    return {
        name: 'tauri-bindgen',
        resolveId(id) {
            if (id.endsWith('.wit')) {
                return id;
            }
        },
        load(id) {
            if (id.endsWith('.wit')) {
                process(kind, id);
            }
        }
    };
}
