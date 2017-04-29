deafult: build

vet:
	go tool vet .

build:
	go build -o ./cmd/airhorn/airhorn ./cmd/airhorn

test:
	go test ./...

docker:
	docker build .

run:
	make && ./cmd/airhorn/airhorn