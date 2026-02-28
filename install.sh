#!/bin/sh
# vitals installer — https://github.com/onuroluc/vitals
#
# Usage:
#   curl -sSfL https://raw.githubusercontent.com/onuroluc/vitals/main/install.sh | sh
#
# Environment variables:
#   VITALS_VERSION  — specific version to install (default: latest)
#   INSTALL_DIR     — where to install (default: /usr/local/bin)

set -e

REPO="onuroluc/vitals"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# ── Detect platform ──────────────────────────────────────────────

detect_platform() {
  OS="$(uname -s)"
  ARCH="$(uname -m)"

  case "$OS" in
    Linux*)  PLATFORM="linux" ;;
    Darwin*) PLATFORM="darwin" ;;
    *)       echo "Error: unsupported OS: $OS"; exit 1 ;;
  esac

  case "$ARCH" in
    x86_64|amd64)  ARCH="amd64" ;;
    arm64|aarch64) ARCH="arm64" ;;
    *)             echo "Error: unsupported architecture: $ARCH"; exit 1 ;;
  esac

  echo "${PLATFORM}-${ARCH}"
}

# ── Get latest version ───────────────────────────────────────────

get_latest_version() {
  if [ -n "$VITALS_VERSION" ]; then
    echo "$VITALS_VERSION"
    return
  fi

  VERSION=$(curl -sSfL "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name"' \
    | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')

  if [ -z "$VERSION" ]; then
    echo "Error: could not determine latest version" >&2
    exit 1
  fi

  echo "$VERSION"
}

# ── Main ─────────────────────────────────────────────────────────

main() {
  PLATFORM=$(detect_platform)
  VERSION=$(get_latest_version)
  BINARY="vitals-${PLATFORM}"
  URL="https://github.com/${REPO}/releases/download/${VERSION}/${BINARY}.tar.gz"

  echo "Installing vitals ${VERSION} (${PLATFORM})..."
  echo "  from: ${URL}"
  echo "  to:   ${INSTALL_DIR}/vitals"
  echo ""

  TMPDIR=$(mktemp -d)
  trap 'rm -rf "$TMPDIR"' EXIT

  curl -sSfL "$URL" -o "${TMPDIR}/vitals.tar.gz"
  tar xzf "${TMPDIR}/vitals.tar.gz" -C "$TMPDIR"
  chmod +x "${TMPDIR}/vitals"

  # Install — try direct first, then sudo
  if [ -w "$INSTALL_DIR" ]; then
    mv "${TMPDIR}/vitals" "${INSTALL_DIR}/vitals"
  else
    echo "  (requires sudo to write to ${INSTALL_DIR})"
    sudo mv "${TMPDIR}/vitals" "${INSTALL_DIR}/vitals"
  fi

  echo ""
  echo "✓ vitals ${VERSION} installed to ${INSTALL_DIR}/vitals"
  echo ""
  echo "  Run 'vitals' in any project directory to check environment health."
}

main
