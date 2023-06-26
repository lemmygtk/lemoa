# Usage ./build-flatpak.sh [version]

# First run `cargo vendor` and add the output to ~/.cargo/config.toml

rm -rf target _build build builddir
cd ..
tar cfJ "lemoa-$1.tar.xz" lemoa
sha256sum "lemoa-$1.tar.xz"
cd lemoa
git reset --hard &> /dev/null
