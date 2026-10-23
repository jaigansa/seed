#!/usr/bin/env bash
# ==============================================================================
# Seed Installer: Modern Terminal Workspace Manager & Scaffolder
# ==============================================================================
set -euo pipefail

BOLD="\033[1m"
GREEN="\033[0;32m"
CYAN="\033[0;36m"
YELLOW="\033[0;33m"
RESET="\033[0m"

echo -e "${CYAN}${BOLD}"
cat << 'EOF'
   _____ ______ ______ _____  
  / ____|  ____|  ____|  __ \ 
 | (___ | |__  | |__  | |  | |
  \___ \|  __| |  __| | |  | |
  ____) | |____| |____| |__| |
 |_____/|______|______|_____/ 
EOF
echo -e "${RESET}"
echo -e "${BOLD}Installing Seed — Workspace Manager & Project Scaffolder${RESET}\n"

# 1. Determine destination directory
if [ "$(id -u)" -eq 0 ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="${HOME}/.local/bin"
fi
mkdir -p "${INSTALL_DIR}"

# 2. Build or locate binary
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_PATH=""

if [ -f "${SCRIPT_DIR}/target/release/seed" ]; then
    BIN_PATH="${SCRIPT_DIR}/target/release/seed"
elif command -v cargo >/dev/null 2>&1; then
    echo -e "${CYAN}→ Building release binary with cargo...${RESET}"
    (cd "${SCRIPT_DIR}" && cargo build --release)
    BIN_PATH="${SCRIPT_DIR}/target/release/seed"
elif [ -f "${SCRIPT_DIR}/seed" ]; then
    BIN_PATH="${SCRIPT_DIR}/seed"
else
    echo -e "${YELLOW}[!] 'cargo' not found and prebuilt binary not found.${RESET}"
    echo "Please install Rust & Cargo (https://rustup.rs) to build Seed from source."
    exit 1
fi

echo -e "${CYAN}→ Installing binary to ${INSTALL_DIR}/seed...${RESET}"
install -m 755 "${BIN_PATH}" "${INSTALL_DIR}/seed"
echo -e "${GREEN}✓ Installed 'seed' executable to ${INSTALL_DIR}/seed${RESET}"

# 3. Install Shell Auto-Completions
echo -e "\n${CYAN}→ Installing shell auto-completions...${RESET}"

# Bash
BASH_COMP_DIR="${HOME}/.local/share/bash-completion/completions"
mkdir -p "${BASH_COMP_DIR}"
if "${INSTALL_DIR}/seed" completions bash > "${BASH_COMP_DIR}/seed" 2>/dev/null; then
    echo -e "${GREEN}✓ Bash completions installed to ${BASH_COMP_DIR}/seed${RESET}"
fi

# Zsh
ZSH_COMP_DIR="${HOME}/.zfunc"
mkdir -p "${ZSH_COMP_DIR}"
if "${INSTALL_DIR}/seed" completions zsh > "${ZSH_COMP_DIR}/_seed" 2>/dev/null; then
    echo -e "${GREEN}✓ Zsh completions installed to ${ZSH_COMP_DIR}/_seed${RESET}"
fi

# Fish
FISH_COMP_DIR="${HOME}/.config/fish/completions"
mkdir -p "${FISH_COMP_DIR}"
if "${INSTALL_DIR}/seed" completions fish > "${FISH_COMP_DIR}/seed.fish" 2>/dev/null; then
    echo -e "${GREEN}✓ Fish completions installed to ${FISH_COMP_DIR}/seed.fish${RESET}"
fi

# 4. PATH verification
echo ""
if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
    echo -e "${YELLOW}[!] Note: '${INSTALL_DIR}' is not currently in your \$PATH.${RESET}"
    echo "Add the following line to your ~/.bashrc or ~/.zshrc:"
    echo -e "  ${BOLD}export PATH=\"${INSTALL_DIR}:\$PATH\"${RESET}\n"
fi

echo -e "${GREEN}${BOLD}✓ Installation complete!${RESET}"
echo -e "Run ${CYAN}seed tui${RESET} to launch the interactive workspace dashboard."
echo -e "Run ${CYAN}seed --help${RESET} for CLI documentation."
