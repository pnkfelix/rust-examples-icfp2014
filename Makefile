default: tested gen-html

EMACS:=/Applications/Emacs.app/Contents/MacOS/Emacs
RUST_MODE_DIR:=$(HOME)/ConfigFiles/Elisp/rust-mode

rwildcard=$(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2) $(filter $(subst *,%,$2),$d))

RS_FILES=$(call rwildcard, src/, *.rs)
HTML_FILES=$(subst .rs,.rs.html,$(RS_FILES))

# $(info RS_FILES $(RS_FILES))
# $(info HTML_FILES $(HTML_FILES))

DEBUG_TARGET=target/debug/fsk-examples
RELEASE_TARGET=target/release/fsk-examples

.PHONY: tested
tested: $(DEBUG_TARGET)
	cargo run

.PHONY: gen-html
gen-html: $(HTML_FILES)

%.rs.html: %.rs emacs-batch-init.el
	$(EMACS) --batch --directory $(RUST_MODE_DIR) --load emacs-batch-init.el --eval '(htmlfontify-file "$<")'

$(DEBUG_TARGET): $(RS_FILES)
	cargo build

$(RELEASE_TARGET): $(RS_FILES)
	cargo build --release


