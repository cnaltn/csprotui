#!/usr/bin/env bash
set -euo pipefail

REPO="cnaltn/csprotui"
INSTALL_DIR="${HOME}/.local/share/csprotui"
BIN_DIR="${HOME}/.local/bin"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info()  { echo -e "${BLUE}==>${NC} $1"; }
ok()    { echo -e "${GREEN}  ✓${NC} $1"; }
warn()  { echo -e "${YELLOW}  !${NC} $1"; }
fail()  { echo -e "${RED}  ✗${NC} $1"; exit 1; }

command -v curl >/dev/null || command -v wget >/dev/null || fail "curl or wget required"
command -v tar >/dev/null  || fail "tar required"
command -v node >/dev/null || warn "node not found — scraper will not work"

# Detect platform
case "$(uname -sm)" in
  "Linux x86_64")   TARGET="linux-x86_64"; ARCHIVE="tar.gz" ;;
  "Linux aarch64")  TARGET="linux-aarch64"; ARCHIVE="tar.gz" ;;
  "Darwin x86_64")  TARGET="macos-x86_64"; ARCHIVE="tar.gz" ;;
  "Darwin arm64")   TARGET="macos-aarch64"; ARCHIVE="tar.gz" ;;
  *) fail "Unsupported platform: $(uname -sm)" ;;
esac

info "Detected platform: ${TARGET}"

# Fetch latest release tag
info "Fetching latest release..."
API_URL="https://api.github.com/repos/${REPO}/releases/latest"
if command -v curl >/dev/null; then
  TAG=$(curl -fsSL "${API_URL}" | grep -o '"tag_name": "[^"]*"' | cut -d'"' -f4)
else
  TAG=$(wget -qO- "${API_URL}" | grep -o '"tag_name": "[^"]*"' | cut -d'"' -f4)
fi

[ -z "${TAG}" ] && fail "Could not determine latest release tag"
ok "Latest release: ${TAG}"

# Download
ARCHIVE_NAME="csprotui-${TARGET}.${ARCHIVE}"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${TAG}/${ARCHIVE_NAME}"
TMP_DIR=$(mktemp -d)
trap 'rm -rf "${TMP_DIR}"' EXIT

info "Downloading ${ARCHIVE_NAME}..."
if command -v curl >/dev/null; then
  curl -fsSL --progress-bar "${DOWNLOAD_URL}" -o "${TMP_DIR}/${ARCHIVE_NAME}"
else
  wget -q --show-progress "${DOWNLOAD_URL}" -O "${TMP_DIR}/${ARCHIVE_NAME}"
fi
ok "Downloaded"

# Extract
info "Extracting..."
cd "${TMP_DIR}"
tar -xzf "${ARCHIVE_NAME}"
ok "Extracted"

# Install scraper deps
if [ -d "scraper" ] && command -v npm >/dev/null; then
  info "Installing scraper dependencies..."
  cd scraper
  npm install --silent
  cd ..
  ok "Scraper ready"
fi

# Move to install dir
mkdir -p "${INSTALL_DIR}"
rm -rf "${INSTALL_DIR}/csprotui" "${INSTALL_DIR}/scraper"
mv csprotui "${INSTALL_DIR}/"
[ -d "scraper" ] && mv scraper "${INSTALL_DIR}/"
ok "Installed to ${INSTALL_DIR}"

# Create wrapper
mkdir -p "${BIN_DIR}"
cat > "${BIN_DIR}/csprotui" << 'EOF'
#!/usr/bin/env bash
export CSPROTUI_SCRAPER_DIR="${HOME}/.local/share/csprotui/scraper"
exec "${HOME}/.local/share/csprotui/csprotui" "$@"
EOF
chmod +x "${BIN_DIR}/csprotui"
ok "Wrapper created at ${BIN_DIR}/csprotui"

# PATH check
if [[ ":${PATH}:" != *":${BIN_DIR}:"* ]]; then
  warn "${BIN_DIR} is not in your PATH"
  echo "    Add this to your shell config:"
  echo "    export PATH=\"\${HOME}/.local/bin:\${PATH}\""
fi

echo ""
echo -e "${GREEN}CSPROTUI ${TAG} installed!${NC}"
echo "    Run: csprotui"
