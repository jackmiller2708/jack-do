#!/bin/bash

# Jack-Do Installation Script (Unix-like)

INSTALL_DIR="$HOME/.jack-do"
BIN_DIR="$INSTALL_DIR/bin"
EXEC_NAME="jack-do"
DEST_PATH="$BIN_DIR/$EXEC_NAME"

# Colors and symbols
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

INFO_SYM="ℹ️"
SUCCESS_SYM="✅"
WARNING_SYM="⚠️"
ERROR_SYM="❌"

log() {
    local level=$1
    local msg=$2
    case $level in
        "INFO") echo -e "${CYAN}${INFO_SYM} [INFO] ${msg}${NC}" ;;
        "SUCCESS") echo -e "${GREEN}${SUCCESS_SYM} [SUCCESS] ${msg}${NC}" ;;
        "WARNING") echo -e "${YELLOW}${WARNING_SYM} [WARNING] ${msg}${NC}" ;;
        "ERROR") echo -e "${RED}${ERROR_SYM} [ERROR] ${msg}${NC}" ;;
    esac
}

check_dependencies() {
    log "INFO" "Checking dependencies..."
    local missing=()
    if ! command -v rustc &> /dev/null; then missing+=("rustc"); fi
    if ! command -v cargo &> /dev/null; then missing+=("cargo"); fi

    if [ ${#missing[@]} -ne 0 ]; then
        log "ERROR" "Missing dependencies: ${missing[*]}"
        echo -e "\nPlease install Rust and Cargo from https://rustup.rs/ before continuing."
        exit 1
    fi
    log "SUCCESS" "Dependencies satisfied."
}

cleanup_on_failure() {
    log "WARNING" "Installation encountered an error. Initiating recovery..."
    if [ -f "$DEST_PATH" ]; then
        log "INFO" "Removing partial binary: $DEST_PATH"
        rm -f "$DEST_PATH"
    fi
    log "INFO" "Recovery complete. Please check the error above and try again."
}

# Set trap for cleanup on error
trap cleanup_on_failure ERR

install_jack_do() {
    check_dependencies

    log "INFO" "Building Jack-Do in release mode..."
    if ! cargo build --release; then
        log "ERROR" "Cargo build failed. Ensure you have a stable internet connection and valid Rust installation."
        exit 1
    fi

    log "INFO" "Setting up directory structure..."
    mkdir -p "$BIN_DIR"

    log "INFO" "Installing binary to $BIN_DIR..."
    if [ ! -f "target/release/$EXEC_NAME" ]; then
        log "ERROR" "Could not find compiled binary at target/release/$EXEC_NAME"
        exit 1
    fi
    cp "target/release/$EXEC_NAME" "$DEST_PATH"
    chmod +x "$DEST_PATH"

    log "INFO" "Configuring PATH..."
    local current_shell=$(basename "$SHELL")
    local config_file=""
    
    case "$current_shell" in
        bash) config_file="$HOME/.bashrc" ;;
        zsh) config_file="$HOME/.zshrc" ;;
        fish) config_file="$HOME/.config/fish/config.fish" ;;
        *) log "WARNING" "Unknown shell: $current_shell. Please add $BIN_DIR to your PATH manually." ;;
    esac

    if [ -n "$config_file" ] && [ -f "$config_file" ]; then
        if ! grep -q "$BIN_DIR" "$config_file" 2>/dev/null; then
            if [ "$current_shell" == "fish" ]; then
                echo "set -U fish_user_paths $BIN_DIR \$fish_user_paths" >> "$config_file"
            else
                echo -e "\n# Jack-Do" >> "$config_file"
                echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$config_file"
            fi
            log "SUCCESS" "PATH updated in $config_file. Please restart your terminal or source your config file."
        else
            log "SUCCESS" "$BIN_DIR is already in PATH."
        fi
    elif [ -n "$config_file" ]; then
        log "WARNING" "Config file $config_file not found. Please add $BIN_DIR to your PATH manually."
    fi

    log "SUCCESS" "Jack-Do installed successfully! 🎉"
    echo "Try running: jack-do --help"
}

# Start installation
install_jack_do
