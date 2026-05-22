# -*-Makefile-*-

build:
    cargo build

test colours='':
     cargo {{colours}} nextest run

clean:
    cargo clean

debug bin *args:
    cargo run           --bin {{bin}} -- {{args}}

run bin *args:
    cargo run --release --bin {{bin}} -- {{args}}
