# Offline Demo (Air-Gapped Verification)

This guide walks you through verifying all behaviors in a fully offline setup.

## Step 1 — Disconnect all network interfaces
- Turn off Wi-Fi
- Unplug Ethernet
- Disable Bluetooth

## Step 2 — Generate
python3 mscikdf_cli.py generate "pass123"

## Step 3 — Reboot into offline mode
Reboot the entire machine and stay fully offline.

## Step 4 — Recover
python3 mscikdf_cli.py recover "<mnemonic>" "pass123"

## Expected Output
All addresses remain identical.
No network or file I/O is performed.
