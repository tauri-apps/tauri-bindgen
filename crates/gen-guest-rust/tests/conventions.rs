pub mod conventions {
    
    use ::tauri_bindgen_guest_rust::tauri_bindgen_abi;
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct LudicrousSpeed {
        how_fast_are_you_going: u32,
        i_am_going_extremely_slow: u64,
    }
    pub async fn kebab_case() {
        todo!()
    }
    pub async fn foo(_x: LudicrousSpeed) {
        todo!()
    }
    pub async fn function_with_underscores() {
        todo!()
    }
    pub async fn function_with_no_weird_characters() {
        todo!()
    }
    pub async fn apple() {
        todo!()
    }
    pub async fn apple_pear() {
        todo!()
    }
    pub async fn apple_pear_grape() {
        todo!()
    }
    pub async fn a0() {
        todo!()
    }
    pub async fn is_xml() {
        todo!()
    }
    pub async fn explicit() {
        todo!()
    }
    pub async fn explicit_snake() {
        todo!()
    }
    pub async fn bool() {
        todo!()
    }
}
