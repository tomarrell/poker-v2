MIGRATIONS_PATH = ./migrations
DB_PATH = sqlite:///Users/toma/Documents/poker-v2/poker.db

.PHONY: watch migrate status

watch:
	cargo watch -x run -w ./src

migrate-up:
	@dbmigrate --path=$(MIGRATIONS_PATH) --url=$(DB_PATH) up

migrate-down:
	@dbmigrate --path=$(MIGRATIONS_PATH) --url=$(DB_PATH) down

migrate-revert:
	@dbmigrate --path=$(MIGRATIONS_PATH) --url=$(DB_PATH) down

status:
	@dbmigrate --path=$(MIGRATIONS_PATH) --url=$(DB_PATH) status

ifeq (, $(shell which dbmigrate))
	$(error "No `dbmigrate` in $(PATH), please install it using: `cargo install dbmigrate`")
endif
