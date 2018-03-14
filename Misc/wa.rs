#[wasm_bindgen]
extern {
    type console;

    #[wasm_bindgen(static = console)]
    fn log(s: &str);
}
