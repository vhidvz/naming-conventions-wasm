//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use naming_conventions_wasm::*;
use wasm_bindgen_test::*;

/**
 * No Case
 */

#[wasm_bindgen_test]
fn test_to_no_case() {
    let result = to_no_case("no case");
    assert_eq!(result, "no case")
}

#[wasm_bindgen_test]
fn test_is_no_case() {
    let result = is_no_case("no case");
    assert_eq!(result, true)
}

/**
 * Snake Case
 */

#[wasm_bindgen_test]
fn test_to_snake_case() {
    let result = to_snake_case("snake case");
    assert_eq!(result, "snake_case")
}

#[wasm_bindgen_test]
fn test_is_snake_case() {
    let result = is_snake_case("snake case");
    assert_eq!(result, false)
}

/**
 * Camel Case
 */

#[wasm_bindgen_test]
fn test_to_camel_case() {
    let result = to_camel_case("camel case");
    assert_eq!(result, "camelCase")
}

#[wasm_bindgen_test]
fn test_is_camel_case() {
    let result = is_camel_case("camel case");
    assert_eq!(result, false)
}

/**
 * Pascal Case
 */

#[wasm_bindgen_test]
fn test_to_pascal_case() {
    let result = to_pascal_case("pascal case");
    assert_eq!(result, "PascalCase")
}

#[wasm_bindgen_test]
fn test_is_pascal_case() {
    let result = is_pascal_case("pascal case");
    assert_eq!(result, false)
}

/**
 * Macro Case
 */

#[wasm_bindgen_test]
fn test_to_macro_case() {
    let result = to_macro_case("macro case");
    assert_eq!(result, "MACRO_CASE")
}

#[wasm_bindgen_test]
fn test_is_macro_case() {
    let result = is_macro_case("macro case");
    assert_eq!(result, false)
}

/**
 * Kebab Case
 */

#[wasm_bindgen_test]
fn test_to_kebab_case() {
    let result = to_kebab_case("kebab case");
    assert_eq!(result, "kebab-case")
}

#[wasm_bindgen_test]
fn test_is_kebab_case() {
    let result = is_kebab_case("kebab case");
    assert_eq!(result, false)
}

/**
 * Flat Case
 */

#[wasm_bindgen_test]
fn test_to_flat_case() {
    let result = to_flat_case("flat case");
    assert_eq!(result, "flatcase")
}

#[wasm_bindgen_test]
fn test_is_flat_case() {
    let result = is_flat_case("flat case");
    assert_eq!(result, false)
}

/**
 * Train Case
 */

#[wasm_bindgen_test]
fn test_to_train_case() {
    let result = to_train_case("train case");
    assert_eq!(result, "Train-Case")
}

#[wasm_bindgen_test]
fn test_is_train_case() {
    let result = is_train_case("train case");
    assert_eq!(result, false)
}
