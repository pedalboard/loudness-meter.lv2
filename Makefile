.PHONY: build bundle

.DEFAULT_GOAL := help

build: ## build the plugin
	cargo build --release --target aarch64-unknown-linux-gnu

bundle: build ## bundle the plugin
	cp target/release/*.so eg-amp-rs.lv2
	tar -cvzf eg-amp-rs.lv2.tgz eg-amp-rs.lv2


help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


