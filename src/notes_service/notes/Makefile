ifneq (,)
This makefile requires GNU Make.
endif

.EXPORT_ALL_VARIABLES:
DATABASE_URL=postgres://postgres:docker@localhost/notes 

cargo-run:
	cargo run
.PHONY: cargo-run

cargo-test: run-postgres-local-db
	cargo test -- --nocapture
.PHONY: cargo-test

run-postgres-local-db:
	docker stop pg-docker || true
	docker run --rm   --name pg-docker -e POSTGRES_PASSWORD=docker -d -p 5432:5432 -v ${HOME}/docker/volumes/postgres:/var/lib/postgresql/data  postgres
	sleep 5
.PHONY: run-postgres-local-db