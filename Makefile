.PHONY: build bundle

.DEFAULT_GOAL := help


PLUGIN_NAME = db-meter

build: ## build the plugin
	cargo build --release --target aarch64-unknown-linux-gnu

bundle: build ## bundle the plugin
	mkdir -p target/bundle
	cp -r $(PLUGIN_NAME).lv2 target/bundle
	cp target/release/*.so target/bundle/$(PLUGIN_NAME).lv2
	tar -cvzf target/bundle/$(PLUGIN_NAME).lv2.tgz target/bundle/$(PLUGIN_NAME).lv2


help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


