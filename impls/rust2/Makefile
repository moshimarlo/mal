STEPS = step0_repl

all: $(STEPS)

%: %.rs
	cargo build --release --bin $*
	cp target/release/$* $@

STEP0_DEPS = Cargo.toml

step0_repl: $(STEP0_DEPS)
