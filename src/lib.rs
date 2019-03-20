extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub struct Pallium {
    log_sources: Vec<String>,
}

#[wasm_bindgen]
impl Pallium {
    pub fn new() -> Self {
        Pallium {
            log_sources: vec!["source a".to_owned(), "source b".to_owned()],
        }
    }

    pub fn source_count(&self) -> usize {
        self.log_sources.len()
    }

    pub fn source(&self, idx: usize) -> Option<String> {
        self.log_sources.get(idx).map(|s| s.clone())
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
