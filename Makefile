
# Get version from Cargo.toml
# Tag
# Package
# Release with https://hub.github.com/hub-release.1.html

cross-compile:
	cargo build --target=i686-pc-windows-gnu --release

package:
	zip --junk-paths lamcour2utf8-`timestamp`.zip target/i686-pc-windows-gnu/release/lamcour2utf8.exe
	zip --junk-paths lamcour2utf8.zip target/i686-pc-windows-gnu/release/lamcour2utf8.exe
