all:
	cargo build --release
	rm -f build/workflow/alfred-confluence-workflow.alfredworkflow
	mkdir -p build
	cp target/release/alfred-confluence-workflow workflow/
	cd workflow && zip -qR alfred-confluence-workflow.alfredworkflow "*"
	rm workflow/alfred-confluence-workflow
	mv workflow/alfred-confluence-workflow.alfredworkflow build/
