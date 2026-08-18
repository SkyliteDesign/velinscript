//! Unit tests for codegen::lowering (also covered via #[cfg(test)] in the module).

use velin_compiler::codegen::lowering::{normalize_path, to_snake_case};

#[test]
fn path_param_normalization() {
    assert_eq!(normalize_path("/items/:id/edit"), "/items/{id}/edit");
}

#[test]
fn snake_case_conversion() {
    assert_eq!(to_snake_case("getUser"), "get_user");
}
