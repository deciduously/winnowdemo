.PHONY: all clean deploy docs help

VERSION=1.4.1

PROJECT=$(notdir $(shell pwd))
BUILDTARGET=$(PROJECT)/$(BUILDDIR)
EXEC=$(BUILDTARGET)$(PROJECT)
RUSTCLEAN=cargo clean
RUSTDOC=cargo doc --document-private-items --no-deps
RUST=cargo build --release
PKGDIR=$(PROJECT)_$(VERSION)_linux_x64
XZTARGET=$(PKGDIR).tar.xz

all: $(EXEC)

clean:
	rm -rf $(PROJECT)_*
	$(RUSTCLEAN)

docs: all
	$(RUSTDOC)
	rm -rf ./docs
	cp -r ./target/doc/ ./docs

$(EXEC):
	$(RUST)

deploy: clean all $(TESTFILES)
	mkdir  -p $(PKGDIR)
	cp *.txt $(PKGDIR)
	cp *.md $(PKGDIR)
	cp ./target/release/$(PROJECT) $(PKGDIR)
	tar -cf - $(PKGDIR) | xz -9 - > $(XZTARGET)

help:
    @echo "Usage: make {all|clean|deploy|docs|help}" 1>&2 && false
