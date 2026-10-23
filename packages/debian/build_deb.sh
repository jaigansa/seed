#!/usr/bin/env bash
# ==============================================================================
# Helper script to bundle Seed as a Debian (.deb) package
# ==============================================================================
set -euo pipefail

VERSION="0.2.0"
ARCH=$(dpkg --print-architecture 2>/dev/null || echo "amd64")
PKG_NAME="seed_${VERSION}_${ARCH}"
BUILD_ROOT="/tmp/${PKG_NAME}"

echo "Building Debian package for seed v${VERSION} (${ARCH})..."

# 1. Compile release binary if needed
if [ ! -f "target/release/seed" ]; then
    echo "Compiling release binary..."
    cargo build --release
fi

# 2. Prepare package directory tree
rm -rf "${BUILD_ROOT}"
mkdir -p "${BUILD_ROOT}/DEBIAN"
mkdir -p "${BUILD_ROOT}/usr/bin"
mkdir -p "${BUILD_ROOT}/usr/share/man/man1"
mkdir -p "${BUILD_ROOT}/usr/share/bash-completion/completions"
mkdir -p "${BUILD_ROOT}/usr/share/zsh/vendor-completions"
mkdir -p "${BUILD_ROOT}/usr/share/fish/vendor_completions.d"
mkdir -p "${BUILD_ROOT}/usr/share/doc/seed"

# 3. Copy files
install -m 755 target/release/seed "${BUILD_ROOT}/usr/bin/seed"
install -m 644 packages/debian/seed.1 "${BUILD_ROOT}/usr/share/man/man1/seed.1"
install -m 644 completions/seed.bash "${BUILD_ROOT}/usr/share/bash-completion/completions/seed"
install -m 644 completions/_seed "${BUILD_ROOT}/usr/share/zsh/vendor-completions/_seed"
install -m 644 completions/seed.fish "${BUILD_ROOT}/usr/share/fish/vendor_completions.d/seed.fish"
install -m 644 README.md "${BUILD_ROOT}/usr/share/doc/seed/README.md"
install -m 644 LICENSE "${BUILD_ROOT}/usr/share/doc/seed/copyright"

# 4. Generate Debian control file
cat << EOF > "${BUILD_ROOT}/DEBIAN/control"
Package: seed
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: Jaiganesh <jaigansa@users.noreply.github.com>
Description: Modern terminal workspace manager, project scaffolder, and health auditor
 Seed is a fast, keyboard-first terminal tool for organizing workspace projects,
 scaffolding creative and software repositories from templates, monitoring
 project health, and maintaining file hygiene.
EOF

# 5. Build deb archive
dpkg-deb --build --root-owner-group "${BUILD_ROOT}"
mv "/tmp/${PKG_NAME}.deb" .
rm -rf "${BUILD_ROOT}"

echo "Debian package successfully built: ${PKG_NAME}.deb"
