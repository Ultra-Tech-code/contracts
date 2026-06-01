.PHONY: build optimize test deploy-testnet

build:
	cargo build --target wasm32-unknown-unknown --release

optimize: build
	stellar contract optimize --wasm target/wasm32-unknown-unknown/release/project_registry.wasm
	stellar contract optimize --wasm target/wasm32-unknown-unknown/release/investment_vault.wasm

test:
	cargo test

deploy-testnet: optimize
	stellar contract deploy \
	  --wasm target/wasm32-unknown-unknown/release/project_registry.optimized.wasm \
	  --source $(STELLAR_SECRET_KEY) \
	  --network testnet
	stellar contract deploy \
	  --wasm target/wasm32-unknown-unknown/release/investment_vault.optimized.wasm \
	  --source $(STELLAR_SECRET_KEY) \
	  --network testnet
