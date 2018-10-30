.PHONY: watch

watch:
	cargo watch -x run

migrations:
	@echo "Unimplemented"

ifeq (, $(shell which dbmigrate))
	$(error "No `dbmigrate` in $(PATH), please install it using: `cargo install dbmigrate`")
endif
