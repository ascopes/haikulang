#!/usr/bin/env bash
set -o errexit
set -o nounset
[[ -v DEBUG ]] && set -o xtrace

readonly preferred_llvm_version=21

function exe_exists() {
  command -v "${1}" &> /dev/null
}

function log() {
  printf "\033[1;33m[[%s]]::\033[0;33m%s\033[0m\n" "$(date)" "${*}" 2>/dev/null
}

function get() {
  if exe_exists curl; then
    curl -O "${1}" --silent --fail
  elif exe_exists wget; then
    wget "${1}"
  else
    log "Error: cannot find curl or wget on system \$PATH"
    exit 2
  fi
}

if exe_exists termux-info && exe_exists apt-get; then
  log "Installing llvm dependencies for termux..."
  apt-get install clang libcompiler-rt llvm llvm-tools

elif exe_exists apt-get; then
  log "Installing llvm dependencies for Debian-based distributions..."
  get https://apt.llvm.org/llvm.sh
  trap 'rm llvm.sh; trap - EXIT INT TERM' EXIT INT TERM
  chmod +x llvm.sh
  sudo ./llvm.sh "${preferred_llvm_version}"

  # Point llvm-config to llvm-config-$LLVM_VERSION so Rust doesn't whinge about it
  log "Configuring symlink for llvm-config..."
  sudo ln -sf "$(command -v "llvm-config-${preferred_llvm_version}")" /usr/bin/llvm-config

elif exe_exists dnf; then
  # TODO: pull from llvm-snapshots COPR repository instead?
  # https://copr.fedorainfracloud.org/coprs/g/fedora-llvm-team/llvm-snapshots/
  log "Installing llvm dependencies for Fedora-based distributions..."
  sudo dnf install llvm-devel clang-devel

else
  log "Error: unsupported OS $(uname -a)"
  exit 2
fi

log "LLVM path: $(command -v llvm-config)"
for attr in version prefix host-target; do
  log "LLVM ${attr//-/ }: $(llvm-config "--${attr}")"
done
