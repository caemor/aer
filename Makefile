check: check_epd4in2 check_2in9

check_2in9:
	cargo check --no-default-features --features epd2in9

build_epd2in9:
	RUST_LOG=info cargo build --features epd2in9 --release

run_epd2in9:
	RUST_LOG=info cargo run --features epd2in9 --release

check_epd4in2:
	cargo check --no-default-features --features epd4in2 

build_epd4in2:
	RUST_LOG=info cargo build --features epd4in2 --release

run_epd4in2:
	RUST_LOG=info cargo run --features epd4in2 --release
