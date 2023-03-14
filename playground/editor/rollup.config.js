import typescript from "rollup-plugin-ts"
import {lezer} from "@lezer/generator/rollup"
import terser from '@rollup/plugin-terser';
import { nodeResolve } from '@rollup/plugin-node-resolve';

export default {
  input: "src/index.ts",
  output: [
    {dir: "./dist", format: "es"}
  ],
  plugins: [nodeResolve(), lezer(), typescript(), terser()]
}
