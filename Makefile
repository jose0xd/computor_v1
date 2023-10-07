NAME = computor

SRC = src/main.rs

all: ${NAME}

${NAME}: ${SRC}
	cargo build
	@cp target/debug/computor_v1 ${NAME}

clean:
	cargo clean

fclean: clean
	rm ${NAME}

re: fclean
	@${MAKE}

.PHONY: re fclean clean all
