# ==================================================================================== #
# VARIABLES
# ==================================================================================== #
BINARY_NAME := todo
HTTP_MAIN_PACKAGE_PATH := src/http
MIGRATION_FOLDER := src/migrations
DB_URL := postgres://postgres:pass@localhost:5432/todo?sslmode=disable

# ==================================================================================== #
# DB
# ==================================================================================== #
create-db:
	docker exec -it postgres createdb --username=root --owner=root simple_bank

drop-db:
	docker exec -it postgres dropdb postgres

migrate-up:
	migrate -path "$(MIGRATION_FOLDER)" -database "$(DB_URL)" -verbose up

migrate-up1:
	migrate -path "$(MIGRATION_FOLDER)" -database "$(DB_URL)" -verbose up 1

migrate-down:
	migrate -path "$(MIGRATION_FOLDER)" -database "$(DB_URL)" -verbose down

migrate-down1:
	migrate -path "$(MIGRATION_FOLDER)" -database "$(DB_URL)" -verbose down 1

new-migration:
	migrate create -ext sql -dir "$(MIGRATION_FOLDER)" -seq $(name)

migrate-force:
	migrate -path $(MIGRATION_FOLDER) -database "$(DB_URL)" force 1

db-docs:
	dbdocs build doc/db.dbml

# ==================================================================================== #
# HELPERS
# ==================================================================================== #
## help: print this help message
.PHONY: help
help:
	@echo 'Usage:'
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' | sed -e 's/^/ /'

## confirm: prompt for confirmation
.PHONY: confirm
confirm:
	@echo -n 'Are you sure? [y/N] ' && read ans && [ $${ans:-N} = y ]

## no-dirty: ensure no dirty state in git
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

## test-watch: run tests continuously when code changes
.PHONY: test-watch
test-watch:
	cargo watch -x test

## test-release: run tests in release mode
.PHONY: test-release
test-release:
	cargo test --release --verbose

## test-specific: run a specific test
.PHONY: test-specific
test-specific:
	@echo "Usage: make test-specific TEST=<test_name>"
	cargo test --verbose $(TEST)

## test-doc: run documentation tests
.PHONY: test-doc
test-doc:
	cargo test --doc --verbose

## test-lib: run library tests
.PHONY: test-lib
test-lib:
	cargo test --lib --verbose

## test-bin: run binary tests
.PHONY: test-bin
test-bin:
	cargo test --bin $(BINARY_NAME) --verbose

## test-examples: run examples tests
.PHONY: test-examples
test-examples:
	cargo test --examples --verbose

## test-bench: run benchmarks
.PHONY: test-bench
test-bench:
	cargo bench

## test-features: run tests with all features
.PHONY: test-features
test-features:
	cargo test --all-features --verbose

## test-no-default-features: run tests without default features
.PHONY: test-no-default-features
test-no-default-features:
	cargo test --no-default-features --verbose

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

## run-http: run the HTTP application
.PHONY: run-http
run-http:
	cargo run --bin ${HTTP_MAIN_PACKAGE_PATH}

# ==================================================================================== #
# Docker
# ==================================================================================== #
## docker-compose: run docker-compose
.PHONY: docker-compose
docker-compose: docker-compose-stop docker-compose-start

## docker-compose-start: start docker-compose
.PHONY: docker-compose-start
docker-compose-start:
	docker-compose up --build

## docker-dependency-start: start core docker dependencies
.PHONY: docker-dependency-start
docker-dependency-start:
	docker-compose -f docker-compose-core.yaml up -d

## docker-compose-stop: stop docker-compose
.PHONY: docker-compose-stop
docker-compose-stop:
	docker-compose down
