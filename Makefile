.PHONY: build test deploy-testnet

build:
	stellar contract build

test:
	cargo test

deploy-testnet: build
	stellar contract deploy \
	  --wasm target/wasm32v1-none/release/project_registry.wasm \
	  --source $(STELLAR_SECRET_KEY) \
	  --network testnet \
	  -- \
	  --admin $(ADMIN_ADDRESS) \
	  --whitelister $(WHITELISTER_ADDRESS)
	stellar contract deploy \
	  --wasm target/wasm32v1-none/release/investment_vault.wasm \
	  --source $(STELLAR_SECRET_KEY) \
	  --network testnet \
	  -- \
	  --admin $(ADMIN_ADDRESS) \
	  --usdc_sac $(USDC_SAC_ADDRESS) \
	  --registry $(REGISTRY_CONTRACT_ID)
