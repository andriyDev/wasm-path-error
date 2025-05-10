## Dependencies

```
cargo install wasm-pack
```

## Instructions 

```
wasm-pack build --target web
http-server -o
```

## Results

On Windows: `cargo test` passes.

    `file!()` -> `path="src\\lib.rs" components=[Normal("src"), Normal("lib.rs")]`

On WASM that was built on Windows: `cargo test` would fail.

    `file!()` -> `path="src\\lib.rs" components=[Normal("src\\lib.rs")]`
