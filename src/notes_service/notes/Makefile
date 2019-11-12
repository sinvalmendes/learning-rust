ifneq (,)
This makefile requires GNU Make.
endif

.EXPORT_ALL_VARIABLES:
DATABASE_URL=postgres://postgres:docker@localhost/notes

clean:
	docker stop pg-docker || true
	docker stop notes-server || true
	docker network rm isolated_nw || true
.PHONY: clean

drill-run: clean
	docker network create --driver bridge isolated_nw
	docker run -itd --network=isolated_nw --rm --name pg-docker -e POSTGRES_PASSWORD=docker -d -p 5432:5432 -v ${HOME}/docker/volumes/postgres:/var/lib/postgresql/data postgres
	sleep 5

	docker run -itd --network=isolated_nw --rm --name notes-server -e DATABASE_URL=postgres://postgres:docker@pg-docker/notes -p 8000:8000 sinvalvm/notes_service
	sleep 5

	drill --benchmark drill-benchmark.yaml
.PHONY: drill-run

cargo-run:
	cargo run
.PHONY: cargo-run

quick-cargo-test:
	cargo test -- --nocapture
.PHONY: cargo-test

cargo-test: run-postgres-local-db
	cargo test -- --nocapture
.PHONY: cargo-test

run-postgres-local-db: clean
	docker run -itd --rm --name pg-docker -e POSTGRES_PASSWORD=docker -d -p 5432:5432 -v ${HOME}/docker/volumes/postgres:/var/lib/postgresql/data postgres
	sleep 5
.PHONY: run-postgres-local-db