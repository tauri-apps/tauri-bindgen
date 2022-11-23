import { defineConfig } from "rollup";
import typescript from '@rollup/plugin-typescript'
import commonjs from "@rollup/plugin-commonjs";

export default defineConfig({
    input: './index.ts',
    external: /(.node)$/,
    output: [{
        dir: '.',
        entryFileNames: '[name].mjs',
        format: 'esm' 
    },{
        dir: '.',
        entryFileNames: '[name].cjs',
        format: 'cjs' 
    }],
    plugins: [
        typescript(),
        commonjs()
    ]
})