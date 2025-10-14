# `wasi:test` (prototype)

WIT: [`wit/wasi-test.wit`](./wit/wasi-test.wit)

## Example

[`examples/wasi-http-tests/src/lib.rs`](./examples/wasi-http-tests/src/lib.rs):
```rust
fn test_standard_header(
    ctx: &wasi_test::TestContext,
) -> Result<(), wasi::http::types::HeaderError> {
    ctx.log("Some runners don't print logs for passing tests (by default)");
    wasi::http::types::Headers::from_list(&[("authorization".to_string(), vec![])])?;
    Ok(())
}

fn test_exploding_header(
    ctx: &wasi_test::TestContext,
) -> Result<(), wasi::http::types::HeaderError> {
    const NAME: &str = "🤯";
    ctx.log(format!("Trying header name {NAME:?}"));
    wasi::http::types::Headers::from_list(&[("🤯".to_string(), vec![])])?;
    Ok(())
}

wasi_test::suite!(test_standard_header, test_exploding_header);

```

Composing with a test runner makes the test(s) executable:

```console
$ cargo build --release --target wasm32-wasip2 -p wasi-http-tests
...
$ cargo build --release --target wasm32-wasip2 -p wasi-test-runner-cli
...
$ wac plug\
    --plug target/wasm32-wasip2/release/wasi_http_tests.wasm \
    target/wasm32-wasip2/release/wasi-test-runner-cli.wasm \
    -o wasi-http-tests-runner-cli.wasm

$ wasm-tools component wit wasi-http-tests-runner-cli.wasm | grep export
  export wasi:cli/run@0.2.6;
```

This test imports `wasi:http`; it could be further composed to test another
component but here we'll test `wasmtime-wasi-http`:

```
$ wasmtime run -Wcomponent-model-async -Shttp wasi-http-tests-runner-cli.wasm
Test test-standard-header...PASS
Test test-exploding-header...FAIL (HeaderError::InvalidSyntax)
Test logs:
Trying header name "🤯"

Error: "1 test(s) failed"
```

## Future Ideas

- Use existing test runner library for CLI runner (e.g.
  [`libtest-mimic`](https://docs.rs/libtest-mimic/0.8.1/libtest_mimic/) )
- HTTP runner with HTML UI (hosted by `wasmtime serve`)
- Test discovery / code generation that wraps certain exported `func`s into a `wasi:test/suite`
- Adapter to capture test logs from `wasi:cli/stdio`
- A `wasi-test` CLI tool (to simplify composition and be a home for some the above features)
