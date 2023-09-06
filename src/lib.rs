mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

/**
 * No Case
 */

#[wasm_bindgen]
pub fn to_no_case(string: &str) -> String {
    set_panic_hook();
    naming_conventions::to_no_case(string).unwrap()
}

#[wasm_bindgen]
pub fn is_no_case(string: &str) -> bool {
    set_panic_hook();
    naming_conventions::is_no_case(string).unwrap()
}

/**
 * Snake Case
 */

#[wasm_bindgen]
pub fn to_snake_case(string: &str) -> String {
    set_panic_hook();
    naming_conventions::to_snake_case(string).unwrap()
}

#[wasm_bindgen]
pub fn is_snake_case(string: &str) -> bool {
    set_panic_hook();
    naming_conventions::is_snake_case(string).unwrap()
}

/**
 * Camel Case
 */

#[wasm_bindgen]
pub fn to_camel_case(string: &str) -> String {
    set_panic_hook();
    naming_conventions::to_camel_case(string).unwrap()
}

#[wasm_bindgen]
pub fn is_camel_case(string: &str) -> bool {
    set_panic_hook();
    naming_conventions::is_camel_case(string).unwrap()
}

/**
 * Pascal Case
 */

#[wasm_bindgen]
pub fn to_pascal_case(string: &str) -> String {
    set_panic_hook();
    naming_conventions::to_pascal_case(string).unwrap()
}

#[wasm_bindgen]
pub fn is_pascal_case(string: &str) -> bool {
    set_panic_hook();
    naming_conventions::is_pascal_case(string).unwrap()
}

/**
 * Macro Case
 */

#[wasm_bindgen]
pub fn to_macro_case(string: &str) -> String {
    set_panic_hook();
    naming_conventions::to_macro_case(string).unwrap()
}

#[wasm_bindgen]
pub fn is_macro_case(string: &str) -> bool {
    set_panic_hook();
    naming_conventions::is_macro_case(string).unwrap()
}

/**
 * Kebab Case
 */

#[wasm_bindgen]
pub fn to_kebab_case(string: &str) -> String {
    set_panic_hook();
    naming_conventions::to_kebab_case(string).unwrap()
}

#[wasm_bindgen]
pub fn is_kebab_case(string: &str) -> bool {
    set_panic_hook();
    naming_conventions::is_kebab_case(string).unwrap()
}

/**
 * Train Case
 */

#[wasm_bindgen]
pub fn to_train_case(string: &str) -> String {
    set_panic_hook();
    naming_conventions::to_train_case(string).unwrap()
}

#[wasm_bindgen]
pub fn is_train_case(string: &str) -> bool {
    set_panic_hook();
    naming_conventions::is_train_case(string).unwrap()
}

/**
 * Flat Case
 */

#[wasm_bindgen]
pub fn to_flat_case(string: &str) -> String {
    set_panic_hook();
    naming_conventions::to_flat_case(string).unwrap()
}

#[wasm_bindgen]
pub fn is_flat_case(string: &str) -> bool {
    set_panic_hook();
    naming_conventions::is_flat_case(string).unwrap()
}
