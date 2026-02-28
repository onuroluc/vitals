#!/bin/bash
set -euo pipefail

VERSION="$1"
ARCH="$2"
TARGET="$3"

PKG="vitals_${VERSION}_${ARCH}"

mkdir -p "${PKG}/DEBIAN"
mkdir -p "${PKG}/usr/bin"
mkdir -p "${PKG}/usr/share/doc/vitals"

cp "target/${TARGET}/release/vitals" "${PKG}/usr/bin/"
chmod 755 "${PKG}/usr/bin/vitals"

cp LICENSE "${PKG}/usr/share/doc/vitals/copyright"

cat > "${PKG}/DEBIAN/control" <<CTRL
Package: vitals
Version: ${VERSION}
Section: devel
Priority: optional
Architecture: ${ARCH}
Maintainer: onuroluc <github.com/onuroluc>
Homepage: https://github.com/onuroluc/vitals
Description: Universal development environment doctor
 Auto-detects your project stack and diagnoses runtime versions,
 dependencies, Docker services, ports, and environment variables.
CTRL

# Try --root-owner-group first (dpkg >= 1.19.1), fall back to fakeroot
if dpkg-deb --build --root-owner-group "${PKG}" 2>/dev/null; then
  echo "Built with --root-owner-group"
elif command -v fakeroot &>/dev/null; then
  fakeroot dpkg-deb --build "${PKG}"
else
  dpkg-deb --build "${PKG}"
fi
mv "${PKG}.deb" "vitals_${VERSION}_${ARCH}.deb"
echo "Built vitals_${VERSION}_${ARCH}.deb"
