.PHONY: build bundle

.DEFAULT_GOAL := help

PLUGIN_NAME = loudness-meter
TARGET = aarch64-unknown-linux-gnu

# CROSS_CONTAINER_OPTS is a workaround for https://github.com/cross-rs/cross/issues/1214

build: ## build the plugin
	CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --release --target $(TARGET)

bundle: ## bundle the plugin
	mkdir -p target/bundle
	cp -r $(PLUGIN_NAME).lv2 target/bundle
	cp target/$(TARGET)/release/*.so target/bundle/$(PLUGIN_NAME).lv2
	tar -C target/bundle -cvzf target/$(PLUGIN_NAME).lv2.tgz $(PLUGIN_NAME).lv2

validate: ## validate the bundle
	# requires lv2-dev sordi
	lv2_validate $(PLUGIN_NAME).lv2/manifest.ttl

clean:
	cargo clean

release: clean ## create a release (default dry-run, use RELEASE_ARGS='--execute patch' make release)
	cargo release --no-publish $(RELEASE_ARGS)
	$(MAKE) build bundle
ifdef RELEASE_ARGS
	gh release create --latest --generate-notes $$(git describe --tags --abbrev=0) ./target/$(PLUGIN_NAME).lv2.tgz
endif

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


