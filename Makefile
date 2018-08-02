
VERSION := $(shell grep version Cargo.toml | sed -E 's/.*"(.*)"/\1/')
TARGET=i686-pc-windows-gnu

cross-compile:
	cargo build --target=$(TARGET) --release

package:
	zip --junk-paths dist/lamcour2utf8-$(VERSION).zip target/$(TARGET)/release/lamcour2utf8.exe
	zip --junk-paths dist/lamcour2utf8.zip target/$(TARGET)/release/lamcour2utf8.exe

version:
	@echo $(VERSION)

release: cross-compile package
	hub release create \
		--messenge $(VERSION) \
		--draft --browse \
		--attach dist/lamcour2utf8-$(VERSION).zip \
		$(VERSION)
