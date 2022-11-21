const { invoke } = window.__TAURI__.tauri;
/**
 * @returns {Promise<string | null>}
 */
export async function optionTest() {
  const result = await invoke("plugin:imports|option_test");
  return result;
}
