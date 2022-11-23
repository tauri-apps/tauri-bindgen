import { bindgen } from "./bindgen";
import type { Plugin } from "rollup";

export default function (): Plugin {
  return {
    name: "tauri-bindgen",
    resolveId(id) {
      if (id.endsWith(".wit")) {
        return id;
      }
    },
    load(id) {
      if (id.endsWith(".wit")) {
        console.log(id);

        return bindgen(id)
      }
    },
  };
}
