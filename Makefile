.PHONY: install uninstall install4user uninstall4user clean

PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
INSTALL ?= install
PROGREMS = sender receiver

build:
	cargo build --release

install: build
	for p in $(PROGREMS); do \
		sudo $(INSTALL) -D target/release/$$p $(BINDIR)/$$p; \
	done

uninstall:
	for p in $(PROGREMS); do \
		rm -f $(BINDIR)/$$p; \
	done
	@echo
	@echo "Remember to remove any related configuration and cache files if necessary."
	@echo "cache files located in ~/.cache/deliver/"
	@echo

install4user:
	$(MAKE) install PREFIX=$(HOME)/.local
	@echo
	@echo "Remember to add $HOME/.local/bin to your PATH if it is not already included."
	@echo

uninstall4user:
	$(MAKE) uninstall PREFIX=$(HOME)/.local

clean:
	rm -rf target
