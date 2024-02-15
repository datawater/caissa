THREADS = $(shell nproc --all)
LINKER = $(shell if command -v mold   &> /dev/null; then echo \"-C link-arg=-fuse-ld=mold\"; \
			   elif command -v ld.lld &> /dev/null; then echo \"-C link-arg=-fuse-ld=lld\"; fi)
RUSTCFLAGS = "-C target-cpu=native "$(LINKER)

all: debug

fix:
	cargo fix --allow-dirty

release:
	RUSTFLAGS=$(RUSTCFLAGS) cargo build --release -j$(THREADS) -v

bolt:
	# BOLT Really isn't necessary, I just wanna have fun with it ;pp

	RUSTFLAGS=$(RUSTCFLAGS) cargo build --profile=bolt -j$(THREADS) -v
	llvm-bolt ./target/bolt/caissa -o ./target/bolt/caissa_bolt --hot-data --hot-text --remove-symtab --thread-count=8 -v 2 \
	 --align-blocks --frame-opt=all --frame-opt-rm-stores --inline-memcpy --jump-tables=aggressive --peepholes=all \
	 --stoke

debug:
	RUSTFLAGS=$(RUSTCFLAGS) cargo build --profile=dev -j$(THREADS) -v

clean:
	cargo clean	