```bash
# Check version
# ./mscikdf version
$ ./mscikdf version
MSCIKDF Version: MSCIKDF v1.0-2025.11.10

# Generate a wallet
# $ ./mscikdf generate <passphrase>
$ ./mscikdf  generate abc
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

$ ./mscikdf  export abc cherry awake human fatigue tag major pause bike morning south breeze gravity plug orange shield glimpse own oppose foot orange trip absent absurd pretty
mnemonic : cherry awake human fatigue tag major pause bike morning south breeze gravity plug orange shield glimpse own oppose foot orange trip absent absurd pretty
solana   : 61kuJUR3mz8HdF9iUJsPpWF2p7az4ivUYhENbA8pc1RMjwtbwHqmJTgfiXNq94PFPbkx2F3acHN8CvfiDEjGamn7
evm      : 0x856322be064d131cd4ea267f1fbecd9b85b36c653762504abfb11eb9471d945c
bitcoin  : L1gzrQi4Sd8DLbWkKGbyxRcTRe2SRyJpc49MRuc3KhMiFDAiSDp6
cosmos   : 856322be064d131cd4ea267f1fbecd9b85b36c653762504abfb11eb9471d945c
polkadot : fabcdc5077edf6880f743f7f50bcb45fbc925e9f3bb6a141d0c38553419b9c58
xidentity: NZwxMu88Kc13JmA6xkQekXFoyGDzwWMtlpPkYpk/DAA=

$ ./mscikdf export cde drink grunt pitch royal weird opera economy penalty village undo toy smooth quote auto swing ecology goat advance baby wheel picture dinner manual banana
mnemonic : drink grunt pitch royal weird opera economy penalty village undo toy smooth quote auto swing ecology goat advance baby wheel picture dinner manual banana
solana   : 61kuJUR3mz8HdF9iUJsPpWF2p7az4ivUYhENbA8pc1RMjwtbwHqmJTgfiXNq94PFPbkx2F3acHN8CvfiDEjGamn7
evm      : 0x856322be064d131cd4ea267f1fbecd9b85b36c653762504abfb11eb9471d945c
bitcoin  : L1gzrQi4Sd8DLbWkKGbyxRcTRe2SRyJpc49MRuc3KhMiFDAiSDp6
cosmos   : 856322be064d131cd4ea267f1fbecd9b85b36c653762504abfb11eb9471d945c
polkadot : fabcdc5077edf6880f743f7f50bcb45fbc925e9f3bb6a141d0c38553419b9c58
xidentity: NZwxMu88Kc13JmA6xkQekXFoyGDzwWMtlpPkYpk/DAA=

```
