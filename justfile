all:

setup: migrate frontend backend

migrate:
    sea-orm-cli migrate up

entity:
    sea-orm-cli generate entity -l --with-serde both -o entity/src

frontend:
    cd ./frontend && npm install
    ng build
    cd ..

backend:
    cargo build

clean:
    cargo clean
    rm -rf ./frontend/node_modules
