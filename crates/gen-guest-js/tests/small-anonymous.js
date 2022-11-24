const invoke = window.__TAURI_INVOKE__;
const idlHash = "bee731db80799df9";
/**
 * @returns {Promise<string | null>}
 */
export async function optionTest() {
  const result = await invoke("plugin:anon|option_test", { idlHash });
  return result;
}
