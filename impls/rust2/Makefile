STEPS = step0_repl step1_read_print

all: $(STEPS)

%: %.rs
	cargo build --release --bin $*
	cp target/release/$* $@

STEP0_DEPS = Cargo.toml
STEP1_DEPS = $(STEP0_DEPS) reader.rs printer.rs types.rs

step0_repl: $(STEP0_DEPS)

step1_read_print: $(STEP1_DEPS)
