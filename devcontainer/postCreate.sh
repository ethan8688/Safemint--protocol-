#!/usr/bin/env bash
set -e

# Install system deps
sudo apt-get update -y
sudo apt-get install -y pkg-config libssl-dev curl build-essential

# Solana CLI (stable)
if ! command -v solana >/dev/null 2>&1; then
  sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
  echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
  echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.zshrc || true
  export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
fi

# Anchor via avm
if ! command -v avm >/dev/null 2>&1; then
  cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
fi
avm install latest
avm use latest

# Verify
echo "Versions:"
solana --version || true
anchor --version || true
node -v
npm -v
