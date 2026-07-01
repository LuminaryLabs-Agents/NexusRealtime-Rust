.PHONY: test apk macos-app host-ffi downloads-page package-nexus package-goldrush package-downloads

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

package-nexus:
	bash scripts/package-nexus-project.sh "$(PROJECT)"

package-goldrush:
	PROJECT=/Users/crimsonwheeler/Documents/GitHub/NexusEngine-GoldRush bash scripts/package-nexus-project.sh

package-downloads:
	bash scripts/package-downloads.sh
