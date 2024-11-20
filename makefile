format:
	cargo fmt

lint:
	cargo clippy --all-targets -- -D warnings

test:
	cargo test

run_parse:
	cargo run -- parse test_parser.txt
    
check: format lint

clean:
	cargo clean
