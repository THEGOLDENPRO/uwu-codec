ifeq ($(OS),Windows_NT)     # is Windows_NT on XP, 2000, 7, Vista, 10...
    detected_os := Windows
else
    detected_os := $(shell uname)  # same as "uname -s"
endif

build:
	cargo build --release

install:
ifeq ($(detected_os), Windows)
	copy ".\target\release\uwu-codec.exe" "$(USERPROFILE)\.cargo\bin\"
else
	sudo cp ./target/release/uwu-codec /usr/bin/
endif

test:
	cargo test