default: build strip component compose run

.PHONY: build
build:
	cargo build -p primary --target wasm32-unknown-unknown --release
	cargo build -p secondary --target wasm32-unknown-unknown --release

.PHONY: strip
strip:
	wasm-tools strip target/wasm32-unknown-unknown/release/primary.wasm -d ".debug*" -o primary_stripped.wasm
	wasm-tools strip target/wasm32-unknown-unknown/release/secondary.wasm -d ".debug*" -o secondary_stripped.wasm

.PHONY: component
component:
	wasm-tools component new primary_stripped.wasm -o primary_component.wasm
	wasm-tools component new secondary_stripped.wasm -o secondary_component.wasm

.PHONY: compose
compose:
	wasm-tools compose primary_component.wasm -d secondary_component.wasm -o composed.wasm

.PHONY: run
run:
	cargo run -p runtime -- composed.wasm
