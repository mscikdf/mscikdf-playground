```bash
# Check version
# ./acegf version
$ ./acegf version
ACE-GF Version: 2025.12.20

# Generate an atomic crypto entity
# $ ./acegf generate <passphrase>
$ ./acegf generate abc
{
  "mnemonic": "warfare false observe engage dynamic artwork involve pledge caution labor success error lucky parent enemy convince print elegant repair buddy toilet emerge effort cloth",
  "solana_address": "CpuW5Wz5WsS65nMe1oCuUdAc1B6i2b22uHrHJLumf7Zr",
  "evm_address": "0x149f2eE52ba61ebaB078B9bc16e645176CD5F5FE",
  "bitcoin_address": "bc1pd6v4wna2qfes5ckkyms9ds9jh5f2laqtj6pskkrq4vcljpnhe8sszghxe6",
  "cosmos_address": "cosmos1duqvhkajtpl9gpr47hlge7ny6ucu7fq489fhxc",
  "polkadot_address": "12QG6mCorkFijsqwN3JnVnZ3L2uh18toAtHmdghr42ZfSPw8",
  "xaddress": "🐈acegf16qhkjphldlctgrp0upczrup3kpn9r4u8s4rs0m",
  "xidentity": "jwZ+c+f+hoHyu8c+Po7l+42TVxDUrD65aSGj4eltUW0=",
  "collapsed_at": 1766478265
}
# View an atomic crypto entity
# $ ./acegf view <passphrase> <words of the mnemonic>
$ ./acegf view abc "warfare false observe engage dynamic artwork involve pledge caution labor success error lucky parent enemy convince print elegant repair buddy toilet emerge effort cloth"
{
  "mnemonic": "warfare false observe engage dynamic artwork involve pledge caution labor success error lucky parent enemy convince print elegant repair buddy toilet emerge effort cloth",
  "solana_address": "CpuW5Wz5WsS65nMe1oCuUdAc1B6i2b22uHrHJLumf7Zr",
  "evm_address": "0x149f2eE52ba61ebaB078B9bc16e645176CD5F5FE",
  "bitcoin_address": "bc1pd6v4wna2qfes5ckkyms9ds9jh5f2laqtj6pskkrq4vcljpnhe8sszghxe6",
  "cosmos_address": "cosmos1duqvhkajtpl9gpr47hlge7ny6ucu7fq489fhxc",
  "polkadot_address": "12QG6mCorkFijsqwN3JnVnZ3L2uh18toAtHmdghr42ZfSPw8",
  "xaddress": "🐈acegf16qhkjphldlctgrp0upczrup3kpn9r4u8s4rs0m",
  "xidentity": "jwZ+c+f+hoHyu8c+Po7l+42TVxDUrD65aSGj4eltUW0=",
  "collapsed_at": 1766478332
}

# Change passphrase for the atomic crypto entity
# $ ./acegf rekey <old_passphrase> <new_passphrase> <words of the mnemonic>
$ ./acegf rekey abc cde "warfare false observe engage dynamic artwork involve pledge caution labor success error lucky parent enemy convince print elegant repair buddy toilet emerge effort cloth"
New mnemonic:
"hope story coconut forward neutral blouse trash cheap grief learn wash quiz onion attend fish load evoke home border biology tuna sign march approve"

# View an atomic crypto entity from new mnemonic and new passphrase
# $ ./acegf view <new_passphrase> <words of the new mnemonic>
$ ./acegf view cde "hope story coconut forward neutral blouse trash cheap grief learn wash quiz onion attend fish load evoke home border biology tuna sign march approve"
{
  "mnemonic": "hope story coconut forward neutral blouse trash cheap grief learn wash quiz onion attend fish load evoke home border biology tuna sign march approve",
  "solana_address": "CpuW5Wz5WsS65nMe1oCuUdAc1B6i2b22uHrHJLumf7Zr",
  "evm_address": "0x149f2eE52ba61ebaB078B9bc16e645176CD5F5FE",
  "bitcoin_address": "bc1pd6v4wna2qfes5ckkyms9ds9jh5f2laqtj6pskkrq4vcljpnhe8sszghxe6",
  "cosmos_address": "cosmos1duqvhkajtpl9gpr47hlge7ny6ucu7fq489fhxc",
  "polkadot_address": "12QG6mCorkFijsqwN3JnVnZ3L2uh18toAtHmdghr42ZfSPw8",
  "xaddress": "🐈acegf16qhkjphldlctgrp0upczrup3kpn9r4u8s4rs0m",
  "xidentity": "jwZ+c+f+hoHyu8c+Po7l+42TVxDUrD65aSGj4eltUW0=",
  "collapsed_at": 1766478404
}

$ ./acegf rekey cde cde  "hope story coconut forward neutral blouse trash cheap grief learn wash quiz onion attend fish load evoke home border biology tuna sign march approve"
New mnemonic:
"hope story coconut forward neutral blouse trash cheap grief learn wash quiz onion attend fish load evoke home border biology tuna sign march approve"

$ ./acegf rekey cde abc "hope story coconut forward neutral blouse trash cheap grief learn wash quiz onion attend fish load evoke home border biology tuna sign march approve"
New mnemonic:
"warfare false observe engage dynamic artwork involve pledge caution labor success error lucky parent enemy convince print elegant repair buddy toilet emerge effort cloth"

$ ./acegf generate abc 
{
  "mnemonic": "elder judge inform midnight motor later develop wash book boss ranch eyebrow sample invite nest crunch badge square age best truly vacant tank imitate",
  "solana_address": "GJspvUedb7eotrKkCqmznHFhSYgYTAGrSZnoPmFV5WWT",
  "evm_address": "0xcf7B6770760Bf2e204c0273C7446C3Be4805AfEf",
  "bitcoin_address": "bc1p40xkjxde6jg2tdaa0rh6n5rggd5zy28fphexjs4u7whedhvl0ptqqj06py",
  "cosmos_address": "cosmos1yje5myqnpxask0p9h6kcdl5prvn89kyya64ffp",
  "polkadot_address": "171X7fyGJQCB3UinbZQLAjqAUQS1vA6rtbk2H1ZvJejtKDo",
  "xaddress": "🐈acegf1m7k2ne0zpej67n7l9030auaqagf7laxcjhz5kt",
  "xidentity": "nV8YC9gl84frlSwcgYbRzbLtaFTM3PluEUtKbAt7xRY=",
  "collapsed_at": 1766478645
}

$ ./acegf generate abc
{
  "mnemonic": "problem walnut melt cupboard dish comic extra feature assist claw dog exchange orbit paddle napkin ask tomorrow cinnamon best border scale list tackle weather",
  "solana_address": "EFnZjV5331zzXCwaa6yPubH7AUYPAvZ4Mpu6nds2c6at",
  "evm_address": "0xFcD140618661cC5c6c578F1b5834fD18c60E950C",
  "bitcoin_address": "bc1pawyj2xvd5v3zdal49s70lheavhka9wngkxj59v8zcyfpjttz9h6sk2733y",
  "cosmos_address": "cosmos1gghmauq2v897dwym5r843sdhxytkhwg0r6vytj",
  "polkadot_address": "146bACGJB7QN3VqNJyQgtkXPT8x5XaCaNVcChcuvHuz7Qefz",
  "xaddress": "🐈acegf1p4t9q7rfn9nk0dk99ld2p0rzjjgnqeng5kvpfp",
  "xidentity": "XDOkglOe2HD/SHJyuEEEFXIqtU7xkvgCijLT3SP3iS0=",
  "collapsed_at": 1766478661
}

```
