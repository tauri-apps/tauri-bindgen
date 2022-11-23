import { defineConfig } from "rollup";
import tauriBindgen from 'rollup-plugin-tauri-bindgen'
import typescript from '@rollup/plugin-typescript'

export default defineConfig({
    input: './index.ts',
    output: {
        dir: 'dist',
        format: 'esm' 
    },
    plugins: [
        tauriBindgen('typescript'),
        typescript()
    ]
})