# Verifying That No I/O Occurs

## macOS
sudo dtruss -f python3 mscikdf_cli.py generate "pass123"

## Linux
strace -f -e trace=file,network python3 mscikdf_cli.py generate "pass123"

Expected:
- No open()
- No read() / write()
- No socket()
- No connect()
