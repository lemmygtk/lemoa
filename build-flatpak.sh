# Usage ./build-flatpak.sh

# First run `cargo vendor` and add the output to ~/.cargo/config.toml
VERSION=$(cargo metadata --format-version 1  | jq -r '.packages[]  | select(.name | test("lemoa")) | .version')

rm -rf target _build build builddir
cd ..
tar cfJ "lemoa-$VERSION.tar.xz" lemoa
sha256sum "lemoa-$VERSION.tar.xz"
cd lemoa
git reset --hard &> /dev/null
rm -rf vendor .cargo
