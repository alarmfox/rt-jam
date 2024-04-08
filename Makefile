## up: executes the application using docker-compose
up:
	docker compose  up

## build-images: build docker images
build-images:
	docker compose build

## build: statically build frontend and backend
build:
	cargo build --bin backend --release
	trunk build --release

## dev: creates nats and postgres container; executes backend and frontend locally
dev:
	docker run --network=host nats
	docker run --network=host -e POSTGRES_PASSWORD=postgres postgres
	cargo watch -wq backend/ -w common -x "cargo run --bin backend" &
	trunk serve &

.PHONY: help
## help: prints this help message
help:
	@echo "Usage: "
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' |  sed -e 's/^/ /'
