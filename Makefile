RUSTC=rustc
RUSTC_TARGET=thumbv7m-none-eabi
RUSTC_EDITION=2021
RUSTC_CRATE_TYPE=rlib
RUSTC_CRATE_NAME=manna
RUSTC_FLAGS=--target=$(RUSTC_TARGET)         \
			--edition=$(RUSTC_EDITION)       \
			--crate-type=$(RUSTC_CRATE_TYPE) \
			--crate-name=$(RUSTC_CRATE_NAME) \
			-Zcrate-attr=no_std

libmanna.rlib: lib.rs
	$(RUSTC) $(RUSTC_FLAGS) --emit=link=$@ $<

clean:
	rm *.rlib