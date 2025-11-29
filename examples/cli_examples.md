```bash
# Check version
# ./mscikdf version
$ ./mscikdf version
MSCIKDF Version: MSCIKDF v1.0-2025.11.10

# Generate a wallet
# $ ./mscikdf generate <passphrase>
$ ./mscikdf generate abc
mnemonic : cherry awake human fatigue tag major pause bike morning south breeze gravity plug orange shield glimpse own oppose foot orange trip absent absurd pretty
solana   : 4cGkDenw9emK46wGz53gs4YCoLVMe1hNgMgcVXbiQmXV
evm      : 0x33f27c08257e405bfb065efafe893b08767b5d93
bitcoin  : bc1t37twq5p82esa2wztv6vserxyrfksnc5axxddf
cosmos   : cosmos1t37twq5p82esa2wztv6vserxyrfksnc549e4un
polkadot : 5DGznzfSZXYLiAfgUPcmkvdRd4jHooaXw3qo62kzd8MgfTaE
xidentity: NZwxMu88Kc13JmA6xkQekXFoyGDzwWMtlpPkYpk/DAA=

# Restore a wallet 
# $ ./mscikdf restore <passphrase> <words of the mnemonic>
$ ./mscikdf restore abc cherry awake human fatigue tag major pause bike morning south breeze gravity plug orange shield glimpse own oppose foot orange trip absent absurd pretty
mnemonic : cherry awake human fatigue tag major pause bike morning south breeze gravity plug orange shield glimpse own oppose foot orange trip absent absurd pretty
solana   : 4cGkDenw9emK46wGz53gs4YCoLVMe1hNgMgcVXbiQmXV
evm      : 0x33f27c08257e405bfb065efafe893b08767b5d93
bitcoin  : bc1t37twq5p82esa2wztv6vserxyrfksnc5axxddf
cosmos   : cosmos1t37twq5p82esa2wztv6vserxyrfksnc549e4un
polkadot : 5DGznzfSZXYLiAfgUPcmkvdRd4jHooaXw3qo62kzd8MgfTaE
xidentity: NZwxMu88Kc13JmA6xkQekXFoyGDzwWMtlpPkYpk/DAA=

# Change passphrase for the wallet
# $ ./mscikdf rekey <old_passphrase> <new_passphrase> <words of the mnemonic>
$ ./mscikdf rekey abc cde cherry awake human fatigue tag major pause bike morning south breeze gravity plug orange shield glimpse own oppose foot orange trip absent absurd pretty
New mnemonic: drink grunt pitch royal weird opera economy penalty village undo toy smooth quote auto swing ecology goat advance baby wheel picture dinner manual banana

# Restore wallet from new mnemonic and new passphrase
# $ ./mscikdf restore <new_passphrase> <words of the new mnemonic>
$ ./mscikdf restore cde drink grunt pitch royal weird opera economy penalty village undo toy smooth quote auto swing ecology goat advance baby wheel picture dinner manual banana
mnemonic : drink grunt pitch royal weird opera economy penalty village undo toy smooth quote auto swing ecology goat advance baby wheel picture dinner manual banana
solana   : 4cGkDenw9emK46wGz53gs4YCoLVMe1hNgMgcVXbiQmXV
evm      : 0x33f27c08257e405bfb065efafe893b08767b5d93
bitcoin  : bc1t37twq5p82esa2wztv6vserxyrfksnc5axxddf
cosmos   : cosmos1t37twq5p82esa2wztv6vserxyrfksnc549e4un
polkadot : 5DGznzfSZXYLiAfgUPcmkvdRd4jHooaXw3qo62kzd8MgfTaE
xidentity: NZwxMu88Kc13JmA6xkQekXFoyGDzwWMtlpPkYpk/DAA=

```
