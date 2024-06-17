# ==================================================================================== #
# VARIABLES
# ==================================================================================== #
BINARY_NAME := todo
HTTP_MAIN_PACKAGE_PATH := src/http
MIGRATION_FOLDER := src/internal/adapter/storage/postgres/migration
DB_URL := postgresql://postgres:P@ssw0rd@localhost:5432/postgres?sslmode=disable

# ==================================================================================== #
# DB
# ==================================================================================== #
create-db:
	docker exec -it postgres createdb --username=root --owner=root simple_bank

drop-db:
	docker exec -it postgres dropdb postgres

migrate-up:
	diesel migration run --migration-dir=$(MIGRATION_FOLDER) --database-url=$(DB_URL)

migrate-up1:
	diesel migration run --migration-dir=$(MIGRATION_FOLDER) --database-url=$(DB_URL) --step 1

migrate-down:
	diesel migration revert --migration-dir=$(MIGRATION_FOLDER) --database-url=$(DB_URL)

migrate-down1:
	diesel migration revert --migration-dir=$(MIGRATION_FOLDER) --database-url=$(DB_URL) --step 1

new-migration:
	diesel migration generate $(name) --migration-dir=$(MIGRATION_FOLDER)

migrate-force:
	diesel migration run --migration-dir=$(MIGRATION_FOLDER) --database-url=$(DB_URL) --force

# ==================================================================================== #
# HELPERS
# ==================================================================================== #
## help: print this help message
.PHONY: help
help:
	@echo 'Usage:'
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' | sed -e 's/^/ /'

.PHONY: confirm
confirm:
	@echo -n 'Are you sure? [y/N] ' && read ans && [ $${ans:-N} = y ]

.PHONY: no-dirty
no-dirty:
	git diff --exit-code

# ==================================================================================== #
# QUALITY CONTROL
# ==================================================================================== #
## format: format code
.PHONY: format
format:
	cargo fmt

## tidy: format code and tidy dependencies
.PHONY: tidy
tidy:
	cargo fmt
	cargo check

## audit: run quality control checks
.PHONY: audit
audit:
	cargo check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo audit

## clean: clean-up
.PHONY: clean
clean:
	cargo clean

# ==================================================================================== #
# DEVELOPMENT
# ==================================================================================== #
## test: run all tests
.PHONY: test
test:
	cargo test --verbose

## test-cover: run all tests and display coverage
.PHONY: test-cover
test-cover:
	cargo tarpaulin --verbose --out Html

# ==================================================================================== #
# BUILD & RUN
# ==================================================================================== #
## build: build the application
.PHONY: build
build:
	cargo build --release --bin ${BINARY_NAME}

## run: run the application
.PHONY: run
run: build
	./target/release/${BINARY_NAME}

## run-http: run the http application
.PHONY: run-http
run-http:
	cargo run --bin ${HTTP_MAIN_PACKAGE_PATH}

# ==================================================================================== #
# Docker
# ==================================================================================== #
## docker-compose: run docker-compose
docker-compose: docker-compose-stop docker-compose-start
.PHONY: docker-compose

.PHONY: docker-compose-start
docker-compose-start:
	docker-compose up --build

.PHONY: docker-dependency-start
docker-dependency-start:
	docker-compose -f docker-compose-core.yaml up -d

.PHONY: docker-compose-stop
docker-compose-stop:
	docker-compose down
