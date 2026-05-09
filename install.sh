#!/usr/bin/env sh
set -eu

PROJECT_NAME="agent-linux-control"
DEFAULT_BASE_URL="https://raw.githubusercontent.com/antharmaya-labs/agent-linux-control/main"
BASE_URL="${AGENT_LINUX_CONTROL_BASE_URL:-$DEFAULT_BASE_URL}"
PREFIX="${AGENT_LINUX_CONTROL_PREFIX:-$HOME/.local}"
BIN_DIR="$PREFIX/bin"
PRIMARY_SKILL_DIR="${AGENT_LINUX_CONTROL_SKILL_DIR:-$HOME/.agents/skills/agent-linux-control}"
SOURCE_DIR="${AGENT_LINUX_CONTROL_SOURCE_DIR:-}"
NO_DEPS="${AGENT_LINUX_CONTROL_NO_DEPS:-0}"
ASSUME_YES="${AGENT_LINUX_CONTROL_ASSUME_YES:-1}"

info() { printf '%s\n' "[$PROJECT_NAME] $*"; }
warn() { printf '%s\n' "[$PROJECT_NAME] warning: $*" >&2; }
die() { printf '%s\n' "[$PROJECT_NAME] error: $*" >&2; exit 1; }
have() { command -v "$1" >/dev/null 2>&1; }

sudo_cmd() {
  if [ "$(id -u)" -eq 0 ]; then
    "$@"
  elif have sudo; then
    sudo "$@"
  elif have doas; then
    doas "$@"
  else
    return 127
  fi
}

install_packages() {
  [ "$NO_DEPS" = "1" ] && return 0
  if have dnf; then
    pkgs="python3 ydotool wtype wl-clipboard xclip xsel spectacle grim slurp scrot ImageMagick"
    [ "$ASSUME_YES" = "1" ] && sudo_cmd dnf install -y $pkgs || sudo_cmd dnf install $pkgs || true
  elif have apt-get; then
    pkgs="python3 ydotool wtype wl-clipboard xclip xsel spectacle grim slurp scrot imagemagick gnome-screenshot"
    sudo_cmd apt-get update || true
    [ "$ASSUME_YES" = "1" ] && sudo_cmd apt-get install -y $pkgs || sudo_cmd apt-get install $pkgs || true
  elif have pacman; then
    pkgs="python ydotool wtype wl-clipboard xclip xsel spectacle grim slurp scrot imagemagick gnome-screenshot"
    [ "$ASSUME_YES" = "1" ] && sudo_cmd pacman -S --needed --noconfirm $pkgs || sudo_cmd pacman -S --needed $pkgs || true
  elif have zypper; then
    pkgs="python3 ydotool wtype wl-clipboard xclip xsel spectacle grim slurp scrot ImageMagick"
    [ "$ASSUME_YES" = "1" ] && sudo_cmd zypper --non-interactive install $pkgs || sudo_cmd zypper install $pkgs || true
  elif have apk; then
    pkgs="python3 ydotool wtype wl-clipboard xclip xsel grim slurp scrot imagemagick"
    sudo_cmd apk add $pkgs || true
  else
    warn "no supported package manager found; continuing with existing tools"
  fi
}

install_uinput_rule() {
  if [ -w /dev/uinput ]; then
    return 0
  fi
  if ! sudo_cmd true >/dev/null 2>&1; then
    warn "no sudo/doas access; skipping uinput udev rule"
    return 0
  fi
  tmp="${TMPDIR:-/tmp}/80-agent-linux-control-uinput.rules"
  printf '%s\n' 'KERNEL=="uinput", SUBSYSTEM=="misc", TAG+="uaccess", OPTIONS+="static_node=uinput"' > "$tmp"
  sudo_cmd install -m 0644 "$tmp" /etc/udev/rules.d/80-agent-linux-control-uinput.rules
  sudo_cmd modprobe uinput || true
  sudo_cmd udevadm control --reload-rules || true
  sudo_cmd udevadm trigger /dev/uinput || true
}

fetch_or_copy() {
  src="$1"
  dst="$2"
  mkdir -p "$(dirname "$dst")"
  if [ -n "$SOURCE_DIR" ] && [ -f "$SOURCE_DIR/$src" ]; then
    cp "$SOURCE_DIR/$src" "$dst"
    return 0
  fi
  if have curl; then
    curl -fsSL "$BASE_URL/$src" -o "$dst"
  elif have wget; then
    wget -qO "$dst" "$BASE_URL/$src"
  else
    die "curl or wget is required when AGENT_LINUX_CONTROL_SOURCE_DIR is not set"
  fi
}

install_runtime() {
  mkdir -p "$BIN_DIR"
  fetch_or_copy "bin/agent-linux-control" "$BIN_DIR/agent-linux-control"
  chmod +x "$BIN_DIR/agent-linux-control"
}

install_skill() {
  tmp_skill="${TMPDIR:-/tmp}/agent-linux-control-SKILL.md"
  fetch_or_copy "skills/agent-linux-control/SKILL.md" "$tmp_skill"
  dirs="
$PRIMARY_SKILL_DIR
${CODEX_HOME:-$HOME/.codex}/skills/agent-linux-control
$HOME/.claude/skills/agent-linux-control
$HOME/.gemini/skills/agent-linux-control
$HOME/.qwen/skills/agent-linux-control
$HOME/.config/opencode/skills/agent-linux-control
$HOME/.config/goose/skills/agent-linux-control
$HOME/.config/crush/skills/agent-linux-control
$HOME/.codeium/windsurf/skills/agent-linux-control
"
  for dir in $dirs; do
    mkdir -p "$dir"
    cp "$tmp_skill" "$dir/SKILL.md"
  done
}

main() {
  install_packages
  install_uinput_rule
  install_runtime
  install_skill
  info "installed CLI: $BIN_DIR/agent-linux-control"
  info "installed skills for generic agents, Codex, Claude Code, Gemini, Qwen, OpenCode, Goose, Crush, and Windsurf-style skill folders"
  if ! printf '%s' "$PATH" | grep -q "$BIN_DIR"; then
    warn "$BIN_DIR is not in PATH; add it or call $BIN_DIR/agent-linux-control"
  fi
  "$BIN_DIR/agent-linux-control" doctor || true
}

main "$@"
