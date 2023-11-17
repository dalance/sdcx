VERSION = $(patsubst "%",%, $(word 3, $(shell grep version ./Cargo.toml)))
BUILD_TIME = $(shell date +"%Y/%m/%d %H:%M:%S")
GIT_REVISION = $(shell git log -1 --format="%h")
RUST_VERSION = $(word 2, $(shell rustc -V))
LONG_VERSION = "$(VERSION) ( rev: $(GIT_REVISION), rustc: $(RUST_VERSION), build at: $(BUILD_TIME) )"
ZIP_NAME = sdcx
BIN_NAMES = sdcx

export LONG_VERSION

.PHONY: all test clean release_lnx release_win release_mac

all:
	cargo build

test:
	cargo test

clean:
	cargo clean

release_lnx:
	cargo build --locked --release --target=x86_64-unknown-linux-musl $(addprefix --bin , ${BIN_NAMES})
	zip -j ${ZIP_NAME}-x86_64-linux.zip $(addprefix target/x86_64-unknown-linux-musl/release/, ${BIN_NAMES})

release_win:
	cargo build --locked --release --target=x86_64-pc-windows-msvc $(addprefix --bin , ${BIN_NAMES})
	mv -v $(addsuffix .exe, $(addprefix target/x86_64-pc-windows-msvc/release/, ${BIN_NAMES})) ./
	7z a ${ZIP_NAME}-x86_64-windows.zip $(addsuffix .exe, ${BIN_NAMES})

release_mac:
	cargo build --locked --release --target=x86_64-apple-darwin $(addprefix --bin , ${BIN_NAMES})
	cargo build --locked --release --target=aarch64-apple-darwin $(addprefix --bin , ${BIN_NAMES})
	zip -j ${ZIP_NAME}-x86_64-mac.zip $(addprefix target/x86_64-apple-darwin/release/, ${BIN_NAMES})
	zip -j ${ZIP_NAME}-aarch64-mac.zip $(addprefix target/aarch64-apple-darwin/release/, ${BIN_NAMES})

release_rpm:
	mkdir -p target
	cargo rpm build
	cp target/x86_64-unknown-linux-musl/release/rpmbuild/RPMS/x86_64/* ./

flamegraph:
	cargo bench --bench benchmark -- --profile-time=5
