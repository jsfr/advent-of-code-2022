upgrade:
	cargo update
	cargo upgrade

lint:
	cargo clippy --all-targets --all-features -- -W clippy::pedantic
