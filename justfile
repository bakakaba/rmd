run:
    cargo run

watch:
    cargo-watch -x run

cli:
    cargo run -p cli

watch-cli:
    cargo-watch -x "run -p cli"

test:
    cargo test
