#!/bin/bash

set -e

if [ "$EUID" -ne 0 ]; then
  echo "❌ This script must be run as root."
  echo "Exec: sudo $0"
  exit 1
fi


echo "=== Installing system dependencies ==="

apt update -y
apt install -y --no-install-recommends \
    curl \
    wget \
    gnupg2 \
    software-properties-common \
    ca-certificates \
    lsb-release

echo "=== Checking Node.js installation ==="

if command -v node &> /dev/null && command -v npm &> /dev/null; then
  echo "Node.js $(node --version) and npm $(npm --version) are already installed"

  # Controlla se è una versione sufficientemente recente (>= 18)
  NODE_VERSION=$(node --version | cut -d'.' -f1 | sed 's/v//')
  if [ "$NODE_VERSION" -lt 18 ]; then
      echo "Node.js version is too old ($NODE_VERSION), updating to latest LTS..."
      # Rimuove versione vecchia e installa quella nuova
      apt remove -y nodejs npm 2>/dev/null || true
      curl -fsSL https://deb.nodesource.com/setup_lts.x | bash -
      apt install -y nodejs
      npm install -g npm@latest
  else
      echo "Node.js version is adequate, updating npm to latest version..."
      npm install -g npm@latest
  fi
else
  echo "Node.js not found, installing latest LTS version..."
  # Rimuove eventuali installazioni incomplete
  apt remove -y nodejs npm 2>/dev/null || true

  # Installa Node.js usando NodeSource (ultima versione LTS)
  curl -fsSL https://deb.nodesource.com/setup_lts.x | bash -
  apt install -y nodejs

  # Aggiorna npm all'ultima versione
  npm install -g npm@latest

  echo "Node.js $(node --version) and npm $(npm --version) installed successfully"
fi

echo "=== Checking Rust installation ==="

# Controlla se Rust è già installato
if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
    echo "Rust $(rustc --version) is already installed"

    # Aggiorna Rust se è già installato
    echo "Updating Rust to latest stable version..."
    rustup update stable 2>/dev/null || {
        echo "rustup not available, Rust might be installed via package manager"
        echo "Current Rust version: $(rustc --version)"
    }
else
    echo "Rust not found, installing latest stable version..."

    # Installa Rust usando rustup
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

    # Carica l'ambiente Rust per la sessione corrente
    source ~/.cargo/env

    # Configura Rust
    rustup default stable
    rustup target add x86_64-unknown-linux-gnu

    echo "Rust $(rustc --version) installed successfully"
fi

# Assicurati che Rust sia disponibile nel PATH corrente
if ! command -v rustc &> /dev/null; then
    source ~/.cargo/env 2>/dev/null || true
fi

echo "=== Installing system build dependencies ==="

apt install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libglib2.0-dev \
    libgtk-3-dev \
    libsoup-3.0-dev \
    libjavascriptcoregtk-4.1-dev \
    libwebkit2gtk-4.1-dev \
    libpq-dev # connecting driver for postgres

echo "=== Installing project dependencies ==="

# Installa dipendenze npm del progetto se package.json esiste
if [ -f "package.json" ]; then
    echo "Installing npm dependencies..."
    npm install
else
    echo "No package.json found, skipping npm install"
fi

echo "=== Cleaning up ==="

# Pulisce cache per risparmiare spazio
apt autoremove -y
apt autoclean
npm cache clean --force 2>/dev/null || true

echo "=== Installation completed successfully! ==="

echo "Final versions:"
echo "Node.js: $(node --version 2>/dev/null || echo 'not available')"
echo "npm: $(npm --version 2>/dev/null || echo 'not available')"
echo "Rust: $(rustc --version 2>/dev/null || echo 'not available')"
echo "Cargo: $(cargo --version 2>/dev/null || echo 'not available')"

echo ""
echo "You can now run:"
echo "  npm run dev          # Start frontend development server"
echo "  cargo tauri dev      # Start Tauri development mode"
echo "  cargo build          # Build Rust backend"
