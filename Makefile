all: compile bundle open

release: compile codesign bundle notarize open

open:
	open build/alfred-confluence-workflow.alfredworkflow

compile:
	# exec "rustup target add aarch64-apple-darwin" before running this
	cargo build --release --target=aarch64-apple-darwin
	# exec "rustup target add x86_64-apple-darwin" before running this
	cargo build --release --target=x86_64-apple-darwin

bundle:
	rm -rf build
	mkdir -p build
	cp target/aarch64-apple-darwin/release/alfred-confluence-workflow workflow/acw-arm
	cp target/x86_64-apple-darwin/release/alfred-confluence-workflow workflow/acw-x86
	cd workflow && zip -qR alfred-confluence-workflow.alfredworkflow "*"
	zip -d workflow/alfred-confluence-workflow.alfredworkflow .DS_Store assets/.DS_Store
	mv workflow/alfred-confluence-workflow.alfredworkflow build/
	rm workflow/acw-arm workflow/acw-x86

codesign:
	codesign --force --options runtime --timestamp --sign "${SIGN_CERTIFICATE_NAME}" target/aarch64-apple-darwin/release/alfred-confluence-workflow
	codesign --force --options runtime --timestamp --sign "${SIGN_CERTIFICATE_NAME}" target/x86_64-apple-darwin/release/alfred-confluence-workflow

notarize:
	zip -qj build/alfred-confluence-workflow.alfredworkflow.zip build/alfred-confluence-workflow.alfredworkflow
	xcrun notarytool submit build/alfred-confluence-workflow.alfredworkflow.zip --wait --apple-id "${APPLEID}" --team-id "${TEAMID}"
	#xcrun stapler staple build/alfred-confluence-workflow.alfredworkflow.zip
	#xcrun notarytool log "${ID}" --apple-id "${APPLEID}" --team-id "${TEAMID}"
	rm build/alfred-confluence-workflow.alfredworkflow.zip
