
VERSION := $(shell grep version Cargo.toml | sed -E 's/.*"(.*)"/\1/')
TARGET=i686-pc-windows-gnu

build:
	cargo build --target=$(TARGET) --release

clean:
	cargo clean --target=$(TARGET) --release

package:
	zip --junk-paths dist/lamcour2utf8-$(VERSION)-win64.zip target/$(TARGET)/release/lamcour2utf8.exe
	zip --junk-paths dist/lamcour2utf8.zip target/$(TARGET)/release/lamcour2utf8.exe

version:
	@echo $(VERSION)

release: clean build package
	hub release create \
		--messenge $(VERSION) \
		--draft --browse \
		--attach dist/lamcour2utf8-$(VERSION)-win64.zip \
		$(VERSION)
