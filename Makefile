.PHONY: install uninstall install4user uninstall4user clean purge

PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin
INSTALL ?= install
PROGREMS = sender receiver
SUDO ?= sudo

build:
	cargo build --release

install: build
	@mkdir -p $(BINDIR)
	@for p in $(PROGREMS); do \
		$(SUDO) $(INSTALL) target/release/$$p $(BINDIR)/$$p; \
	done

uninstall:
	@for p in $(PROGREMS); do \
		$(SUDO) rm -f $(BINDIR)/$$p; \
	done
	@echo "Remember to remove config/cache if necessary."

install4user: PREFIX := $(HOME)/.local
install4user: SUDO :=
install4user: install
	@echo
	@echo "Remember to add $(HOME)/.local/bin to your PATH if it is not already included."
	@echo

uninstall4user: PREFIX := $(HOME)/.local
uninstall4user: SUDO :=
uninstall4user: uninstall

purge:
	@rm -rf ~/.config/deliver/
	@rm -rf ~/.cache/deliver/
	@echo
	@echo "All configuration and cache files related to 'deliver' have been removed."
	@echo