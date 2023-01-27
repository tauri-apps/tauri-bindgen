const invoke = window.__TAURI_INVOKE__;
/**
* @returns {Promise<Result<string | null, Error>>}
*/
export async function optionTest() {
  const result = await invoke("plugin:f831ebf42dd49cbb|option_test",);
  return result
}

