all:

setup: docker migrate frontend backend

migrate:
    sea-orm-cli migrate up

entity:
    sea-orm-cli generate entity -l --with-serde both -o entity/src

docker:
    docker compose up -d

frontend:
    cd ./frontend && npm install
    cd ./frontend && ng build

backend:
    cargo build

purge:
    cargo clean
    rm -rf ./backend/static
    rm -rf ./frontend/node_modules
    rm -rf ./frontend/.angular
    docker compose down -v --remove-orphans
