.PHONY: all build release install uninstall clean

BINARY_NAME := treecat
PREFIX := /usr/local
BINDIR := $(PREFIX)/bin
TARGET := target/release/$(BINARY_NAME)

JOBS := $(shell sh -c 'sysctl -n hw.ncpu 2>/dev/null || nproc 2>/dev/null || echo 1')

all: release

build:
	cargo build -j $(JOBS)

release:
	cargo build -j $(JOBS) --release

install:
	@test -x "$(TARGET)" || { \
		echo "treecat: missing $(TARGET). Run: make release"; \
		exit 1; \
	}
	install -d "$(BINDIR)"
	install -m 0755 "$(TARGET)" "$(BINDIR)/$(BINARY_NAME)"

uninstall:
	rm -f "$(BINDIR)/$(BINARY_NAME)"

clean:
	cargo clean
