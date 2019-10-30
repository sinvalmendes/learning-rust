ifneq (,)
This makefile requires GNU Make.
endif

cargo-run:
	cargo run
.PHONY: cargo-run

cargo-test: run-postgres-local-db
	cargo test -- --nocapture
.PHONY: cargo-test

run-postgres-local-db:
	docker stop pg-docker
	docker run --rm --name pg-docker -e POSTGRES_PASSWORD=docker -d -p 5432:5432 postgres
.PHONY: run-postgres-local-db