use std::path::Path;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_file() -> String {
    let path: &Path = file!().as_ref();
    let components = path.components().collect::<Vec<_>>();
    format!("path={:?} components={:?}", path, components)
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(
        get_file(),
        "path=\"src\\\\lib.rs\" components=[Normal(\"src\"), Normal(\"lib.rs\")]"
    );
}
