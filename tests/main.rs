#[cfg(test)]
mod test {
    use bincode::{Decode, Encode};
    use ww_macro::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[derive(Encode, Decode)]
    pub struct Input(pub i32, pub i32);

    #[worked("worker.js")]
    pub fn pow(i: Input) -> i32 {
        i.0.pow(i.1 as u32)
    }
}