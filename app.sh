#!/bin/sh
# Install kbd binary and systemd service
set -e

# Path to built binary (adjust if needed)
KBD_BIN=target/release/kbd
INSTALL_BIN=/usr/local/bin/kbd
SERVICE_FILE=kbd.service
INSTALL_SERVICE=/etc/systemd/system/kbd.service
CONFIG_SRC=config.toml
CONFIG_DIR="/etc/kbd"
CONFIG_DEST="$CONFIG_DIR/config.toml"
CONFIG_HASH="$CONFIG_DIR/config.md5"

if [ ! -f "$KBD_BIN" ]; then
	echo "Error: $KBD_BIN not found. Build the project first (e.g., cargo build --release)."
	exit 1
fi

install_kbd_service() {
	cargo build --release
	strip "$KBD_BIN"
	# Copy config.toml to $HOME/.kbd/config.toml, create directory if needed
	sudo mkdir -p "$CONFIG_DIR"
	sudo cp "$CONFIG_SRC" "$CONFIG_DEST"
	sudo md5sum "$CONFIG_SRC" | awk '{print $1}' | sudo tee "$CONFIG_HASH" > /dev/null

	# Copy binary
	sudo cp "$KBD_BIN" "$INSTALL_BIN"
	sudo chmod 755 "$INSTALL_BIN"

	# Copy service file
	sudo cp "$SERVICE_FILE" "$INSTALL_SERVICE"
	sudo chmod 644 "$INSTALL_SERVICE"

	# Reload systemd and enable service
	sudo systemctl daemon-reload
	sudo systemctl enable kbd.service
	sudo systemctl restart kbd.service

	echo "kbd service installed and started."
}

uninstall_kbd_service() {
	# Stop and disable the service
	sudo systemctl stop kbd.service || true
	sudo systemctl disable kbd.service || true
	# Remove config file only if hash matches
	if [ -f "$CONFIG_DEST" ] && [ -f "$CONFIG_HASH" ]; then
		current_hash=$(md5sum "$CONFIG_DEST" | awk '{print $1}')
		stored_hash=$(cat "$CONFIG_HASH")
		if [ "$current_hash" = "$stored_hash" ]; then
			sudo rm -f "$CONFIG_DEST"
			sudo rm -f "$CONFIG_HASH"
		else
			echo "Config file has been modified; not removing $CONFIG_DEST."
		fi
	else
		sudo rm -f "$CONFIG_DEST"
		sudo rm -f "$CONFIG_HASH"
	fi
	# Remove config directory if empty
	rmdir --ignore-fail-on-non-empty "$CONFIG_DIR" 2>/dev/null || true

	# Remove service file and binary
	sudo rm -f "$INSTALL_SERVICE"
	sudo rm -f "$INSTALL_BIN"

	# Reload systemd
	sudo systemctl daemon-reload

	echo "kbd service uninstalled."
}

case "$1" in
install)
	install_kbd_service
	exit 0
	;;
uninstall)
	uninstall_kbd_service
	exit 0
	;;
*)
	echo "Usage: $0 [install|uninstall]"
	exit 1
	;;
esac
