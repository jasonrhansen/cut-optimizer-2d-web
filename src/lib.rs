#![allow(non_snake_case)]

mod utils;

use cut_optimizer_2d as core;

use serde_derive::Serialize;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum PatternDirection {
    None,
    ParallelToWidth,
    ParallelToLength,
}

impl Into<core::PatternDirection> for PatternDirection {
    fn into(self) -> core::PatternDirection {
        match self {
            PatternDirection::None => core::PatternDirection::None,
            PatternDirection::ParallelToWidth => core::PatternDirection::ParallelToWidth,
            PatternDirection::ParallelToLength => core::PatternDirection::ParallelToLength,
        }
    }
}

#[derive(Serialize)]
struct ErrorMessage {
    error: String,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Optimizer {
    inner: core::Optimizer,
}

#[wasm_bindgen]
impl Optimizer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Optimizer {
        utils::set_panic_hook();
        Default::default()
    }

    pub fn addStockPiece(
        &mut self,
        width: usize,
        length: usize,
        patternDirection: PatternDirection,
    ) {
        self.inner.add_stock_piece(core::StockPiece {
            width,
            length,
            pattern_direction: patternDirection.into(),
        });
    }

    pub fn addCutPiece(
        &mut self,
        external_id: usize,
        width: usize,
        length: usize,
        canRotate: bool,
        patternDirection: PatternDirection,
    ) {
        self.inner.add_cut_piece(core::CutPiece {
            external_id,
            width,
            length,
            can_rotate: canRotate,
            pattern_direction: patternDirection.into(),
        });
    }

    pub fn setCutWidth(&mut self, cutWidth: usize) {
        self.inner.set_cut_width(cutWidth);
    }

    pub fn setRandomSeed(&mut self, seed: usize) {
        self.inner.set_random_seed(seed as u64);
    }

    pub fn optimizeGuillotine(&self, progress_callback: js_sys::Function) -> JsValue {
        let result = self.inner.optimize_guillotine(|progress| {
            let this = JsValue::NULL;
            let progress = JsValue::from(progress);
            progress_callback.call1(&this, &progress).unwrap();
        });

        match result {
            Ok(solution) => JsValue::from_serde(&solution).unwrap(),
            Err(e) => JsValue::from_serde(&ErrorMessage {
                error: error_message(e),
            })
            .unwrap(),
        }
    }

    pub fn optimizeNested(&self, progress_callback: js_sys::Function) -> JsValue {
        let result = self.inner.optimize_nested(|progress| {
            let this = JsValue::NULL;
            let progress = JsValue::from(progress);
            progress_callback.call1(&this, &progress).unwrap();
        });

        match result {
            Ok(solution) => JsValue::from_serde(&solution).unwrap(),
            Err(e) => JsValue::from_serde(&ErrorMessage {
                error: error_message(e),
            })
            .unwrap(),
        }
    }
}

fn error_message(error: core::Error) -> String {
    match error {
        core::Error::NoFitForCutPiece(_) => {
            format!("Not all cut pieces can be cut from the stock pieces")
        }
    }
}
