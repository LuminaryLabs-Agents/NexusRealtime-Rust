.PHONY: test apk

test:
	cargo test --workspace

apk:
	bash scripts/build-apk.sh
