# Usage ./build-flatpak.sh

# download and save the vendored crates
mkdir .cargo
cargo vendor >> .cargo/config.toml

# get the current version for the file name
VERSION=$(cargo metadata --format-version 1  | jq -r '.packages[]  | select(.name | test("lemoa")) | .version')

rm -rf target _build build builddir
cd ..

# archive the source code and vendored crates to a tar.xz
tar cfJ "lemoa-$VERSION.tar.xz" lemoa
sha256sum "lemoa-$VERSION.tar.xz" | cut -d ' ' -f 1
mv "lemoa-$VERSION.tar.xz" ~/Downloads/

# cleanup
cd lemoa
git reset --hard &> /dev/null
rm -rf vendor .cargo
