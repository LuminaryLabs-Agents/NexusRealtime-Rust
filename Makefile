.PHONY: test apk macos-app host-ffi downloads-page

test:
	cargo test --workspace

apk:
	bash scripts/build-apk.sh

macos-app:
	bash scripts/build-macos-app.sh

host-ffi:
	bash scripts/package-host-ffi.sh

downloads-page:
	python3 scripts/write-download-page.py
