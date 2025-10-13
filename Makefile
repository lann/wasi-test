.PHONY: dummy-tests-run dummy-tests-runner-cli dummy-tests wasi-clocks-tests-run wasi-clocks-tests-runner-cli wasi-clocks-tests wasi-test-runner-cli

dummy-tests-run: dummy-tests-runner-cli
	wasmtime run -Wcomponent-model-async dummy-tests-runner-cli.wasm	

dummy-tests-runner-cli: dummy-tests wasi-test-runner-cli
	wac plug --plug target/wasm32-wasip2/release/dummy_tests.wasm target/wasm32-wasip2/release/wasi-test-runner-cli.wasm -o dummy-tests-runner-cli.wasm

dummy-tests:
	cargo build --release --target wasm32-wasip2 -p dummy-tests


wasi-clocks-tests-run: wasi-clocks-tests-runner-cli
	wasmtime run -Wcomponent-model-async -Sp3=y wasi-clocks-tests-runner-cli.wasm	

wasi-clocks-tests-runner-cli: wasi-clocks-tests cli-test-runner
	wac plug --plug target/wasm32-wasip2/release/wasi_clocks_tests.wasm target/wasm32-wasip2/release/wasi-test-runner-cli.wasm -o wasi-clocks-tests-runner-cli.wasm

wasi-clocks-tests:
	cargo build --release --target wasm32-wasip2 -p wasi-clocks-tests
	

wasi-test-runner-cli:
	cargo build --release --target wasm32-wasip2 -p wasi-test-runner-cli


