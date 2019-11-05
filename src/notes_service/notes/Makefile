ifneq (,)
This makefile requires GNU Make.
endif

.EXPORT_ALL_VARIABLES:
DATABASE_URL=postgres://postgres:docker@localhost/notes

drill-run:
	docker network create --driver bridge isolated_nw
	docker stop pg-docker || true
	docker run -itd --network=isolated_nw --rm --name pg-docker -e POSTGRES_PASSWORD=docker -d -p 5432:5432 -v ${HOME}/docker/volumes/postgres:/var/lib/postgresql/data postgres
	sleep 5

	docker stop notes-server || true
	docker run -itd --network=isolated_nw --rm --name notes-server -e DATABASE_URL=postgres://postgres:docker@pg-docker/notes -p 8000:8000 sinvalvm/notes_service
	sleep 5
	drill --benchmark drill-benchmark.yaml
.PHONY: drill-run

cargo-run:
	cargo run
.PHONY: cargo-run

cargo-test: run-postgres-local-db
	cargo test -- --nocapture
.PHONY: cargo-test

run-postgres-local-db:
	docker stop pg-docker || true
	docker run -itd --rm --name pg-docker -e POSTGRES_PASSWORD=docker -d -p 5432:5432 -v ${HOME}/docker/volumes/postgres:/var/lib/postgresql/data postgres
	sleep 5
.PHONY: run-postgres-local-db