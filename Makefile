all:
	cargo build --release
	# exec "rustup target add x86_64-apple-darwin" before running this
	cargo build --release --target=x86_64-apple-darwin
	rm -f build/workflow/alfred-confluence-workflow.alfredworkflow
	mkdir -p build
	cp target/release/alfred-confluence-workflow workflow/acw-arm
	cp target/x86_64-apple-darwin/release/alfred-confluence-workflow workflow/acw-x86
	cd workflow && zip -qR alfred-confluence-workflow.alfredworkflow "*"
	rm workflow/acw-arm workflow/acw-x86
	mv workflow/alfred-confluence-workflow.alfredworkflow build/
