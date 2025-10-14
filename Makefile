.PHONY: dummy-tests-run dummy-tests-runner-cli dummy-tests wasi-test-runner-cli

dummy-tests-run: dummy-tests-runner-cli
	wasmtime run -Wcomponent-model-async dummy-tests-runner-cli.wasm	

dummy-tests-runner-cli: dummy-tests wasi-test-runner-cli
	wac plug --plug target/wasm32-wasip2/release/dummy_tests.wasm target/wasm32-wasip2/release/wasi-test-runner-cli.wasm -o dummy-tests-runner-cli.wasm

dummy-tests:
	cargo build --release --target wasm32-wasip2 -p dummy-tests


wasi-test-runner-cli:
	cargo build --release --target wasm32-wasip2 -p wasi-test-runner-cli

