.PHONY: all clean deploy docs help

VERSION=0.1.0

BUILDDR = build
PROJECT=$(notdir $(shell pwd))
BUILDTARGET=$(PROJECT)/$(BUILDDIR)
DEPLOYDIR=deploy
EXEC=$(BUILDTARGET)$(PROJECT)
LIB=winnow_sm
INPUTFILE=./input.txt
RUSTCLEAN=cargo clean
RUSTDOC=cargo doc --document-private-items
RUST=cargo build --release
PKGDIR=./deploy
XZTARGET=$(PKGDIR)/$(PROJECT)_$(VERSION)_linux_x64.tar.xz

all: $(EXEC)

clean:
	$(RUSTCLEAN)

docs: all
	$(RUSTDOC)
	rm -rf ./docs
	cp -r ./target/doc/ ./docs

$(EXEC):
	$(RUST)

deploy: docs
	mkdir -p $(PKGDIR)
	cp ./target/release/$(PROJECT) $(PKGDIR)
	cp $(INPUTFILE) $(PKGDIR)
	tar -cf - $(PKGDIR) | xz -z - > $(XZTARGET)

help:
    @echo "Usage: make {all|clean|deploy|docs|help}" 1>&2 && false
