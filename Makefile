.PHONY: all clean deploy help

BUILDDR = build
PROJECT=$(notdir $(shell pwd))
BUILDTARGET=$(PROJECT)/$(BUILDDIR)
DEPLOYDIR=deploy
EXEC=$(BUILDTARGET)$(PROJECT)
INPUTFILE=./input.txt
RUSTCLEAN=cargo clean
RUST=cargo build --release
PKGDIR=./deploy
XZTARGET=$(PROJECT)_linux_c64.tar.xz

all: $(EXEC)

clean:
	$(RUSTCLEAN)

$(EXEC):
	$(RUST)

deploy: all
	mkdir -p $(PKGDIR)
	cp ./target/release/$(PROJECT) $(PKGDIR)
	cp $(INPUTFILE) $(PKGDIR)
	tar -cf - $(PKGDIR) | xz -z - > $(XZTARGET)

help:
    @echo "Usage: make {all|clean|deploy|help}" 1>&2 && false
