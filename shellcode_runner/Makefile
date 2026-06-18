.PHONY: all runner encryptor pipeline clean fclean re

all: runner encryptor pipeline

runner:
	cargo build --release -p runner

encryptor:
	cargo build --release -p encryptor

pipeline:
	cargo build --release -p pipeline

clean:
	cargo clean

fclean: clean
	rm -f target/release/runner target/release/encryptor target/release/pipeline ../shellcodes/*.aes ../shellcodes/*.xor ../shellcodes/*.key ../shellcodes/*.o
re: fclean all