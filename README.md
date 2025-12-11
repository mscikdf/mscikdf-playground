# MSCIKDF – Behavior Verification Test Suite  
*A fully offline, air-gapped, stateless cryptographic behavior testing toolkit.*

## ✨ MSCIKDF Capabilities Overview

| MSCIKDF Capability | Technical Description |
| :--- | :--- |
| **Seed-Storage-Free Identity** | The root entropy (master seed) is **never stored** or exported. Identity is recovered **ephemerally** from a sealed artifact and a credential. |
| **Credential-Sealed Entropy** | The master seed is protected by a **built-in, entropy-hardened credential mechanism**, eliminating the exposure risks of a bare key asset. |
| **Atomic Credential Rotation** | Supports **unlimited, stateless passphrase/mnemonic rotation** without requiring rekeying or recalculation of existing derived addresses/keys. |
| **Unilateral Stateless Revocation** | The credential binding can be **revoked immediately and unilaterally**, enabling robust and agile identity invalidation strategies. |
| **PQC Migration Path Readiness** | Provides a seamless, root-level path to migrate identity to **Post-Quantum Cryptography (PQC)** by leveraging the context-isolation mechanism. |
| **Legacy Asset Assimilation** | Existing cryptographic material (e.g., private keys, symmetric keys, or legacy seeds) from Key Management Systems (KMS), Hardware Security Modules (HSM), or traditional wallets can be securely assimilated into the MSCIKDF Crypto Entity model, leveraging its next-generation security features. |

This test suite provides a controlled, reproducible environment for verifying a set of wallet behaviors using **only observable effects**.  

No internal implementation details are exposed.  

All operations can be performed on a **fully offline (air-gapped)** machine, with **no network access**, **no file storage**, and **no external state**.

We also recommend performing each operation on **two independent air-gapped machines** to ensure complete statelessness. For example:

- Generate a wallet on Machine A, then restore it on Machine B.  
- Rotate a passphrase on Machine A, then restore the resulting mnemonic with the new passphrase on Machine B.
- Repeat these steps on both machines as many times as desired.
- Export same private keys with different mnemonics-passphrase pairs

**⚠️ Note on Input Validation**

This CLI playground assumes well-formed UTF-8 input from trusted users
and intentionally performs no input normalization or malicious-input filtering,
because its purpose is to demonstrate the behavioral guarantees of MSCIKDF.

The production MSCIKDF-Core library will enforce:

- Unicode NFC normalization
- Forbidden codepoint detection (zero-width, control, bidi, surrogate)
- Whitespace canonicalization
- UTF-8 validation

to ensure deterministic, misuse-resistant cryptographic behavior across platforms.

> ⚠️ **Important Notice (Dec 11, 2025)**  
> The prebuilt binaries (`*.so`, `*.a`, WASM artifacts) in this repository  
> were compiled from an **older** version of the MSCIKDF implementation and  
> do **NOT** reflect the algorithms, constants, sealing scheme, or  
> domain-separated HKDF structure described in the **latest MSCIKDF paper**.  
> Updated, paper-compliant builds will be published after full verification.

**⚠️ Important Safety Notice**

This toolkit is intended **for testing and research purposes only**.  

The CLI includes a private-key export subcommand, but only for behavior-verification purposes.

While we have made every effort to ensure correctness and security, **we cannot guarantee the absence of bugs, flaws, or unexpected behavior**.  

**Do NOT use real funds**, production keys, or any sensitive material with this test suite.

Use at your own risk.

This further confirms that all behaviors are fully deterministic and do not rely on any hidden state, caches, or device-specific artifacts.

---

## 🔄 Recent Updates (2025-12-04)

The MSCIKDF project has recently added several major improvements:

### 1. Enterprise-grade Revocation Layer (New)
A new revocation primitive has been added to MSCIKDF Core.  
This enables verifiable, deterministic identity invalidation and  
enterprise-level recovery schemes.  
(⚠️ Not included in the CLI.)

### 2. Seamless Migration of Existing Private Keys (New)
MSCIKDF can now wrap and assimilate existing legacy private keys  
(ECDSA / Ed25519 / secp256k1 / etc.) into the unified Crypto Entity model.  
This allows smooth migration from traditional seed-based wallets.  
(⚠️ Not included in the CLI.)

### 3. Added the ePrint Paper
The original MSCIKDF / ADI design paper is now included in this repository  
for research and citation purposes.

### 4. Added Test Case Code
A full set of deterministic, stateless behavior test cases has been added  
to extend this playground’s coverage.

---

# 1. Purpose of This Test Suite

This test suite allows anyone to verify the following behaviors:

- Passphrase rotation without changing any derived addresses  
- Restore of the same multi-chain identity from the same mnemonic + passphrase pair  
- Restore using any previous passphrase version  
- Stateless, offline operation (no database, no cache, no persistent files)  
- Deterministic generation of multiple cryptographic identities from one mnemonic  
- No exposure of seeds, private keys, or derivation paths

The focus is on **observable, reproducible behavior**, not implementation details.

---

# 2. Preparing a Safe and Neutral Test Environment

The following steps ensure that all behavior is verified transparently and without any external influence.

---

## 2.1 Use an Air-Gapped Machine

Before running any tests:

- Disconnect network cables  
- Disable Wi-Fi  
- Disable Bluetooth  
- Preferably use a fresh virtual machine or a spare offline computer  

All functions work entirely offline.

---

## 2.2 Monitor System Calls (Verify No File or Network I/O)

You may track system calls to confirm that no files are read/written and no network communication is attempted.

### macOS
```bash
sudo dtruss -f ./mscikdf generate "test123"

### Linux
```bash
strace -f -e trace=file,network ./mscikdf generate "test123"
```

Expected observations:

- No open(), read(), write()
- No stat()
- No socket() or connect()
- No file or network operations of any kind  

All functions work entirely offline and the library operates purely in memory.

## 2.3 Verify Binary Integrity (Ensure No Self-Modification)

Compute the file hash before and after any operation:

```bash
shasum libmscikdf_core.so
# or
md5 libmscikdf_core.dylib
```

## 2.4 (Optional) Inspect Memory Layout

macOS

```bash
vmmap <pid>
```

Linux

```bash
pmap <pid>
```

No file-backed memory regions should appear.

# 3. Behavior Verification Scenarios

Each scenario is designed as a reproducible experiment.

## 3.1 Scenario 1 — Generate and restore the Same Wallet from a Mnemonic

### Step 1 — Generate

All tests use the command-line tool provided by this test suite.

```bash
./mscikdf generate "pass123"
```

This outputs:

- mnemonic

- Addresses for multiple chains

- Public keys for supported identity types

### Step 2 — Power off, reboot, stay offline or start a fresh VM

### Step 3 — restore

```bash
./mscikdf restore  "pass123" "<mnemonic>"
```

**Expected** :
All addresses and public identities match exactly.

No private keys or seeds are shown at any time.

## 3.2 Scenario 2 — Change the Passphrase While Keeping All Addresses the Same

### Step 1

```bash
./mscikdf restore "pass" "<mnemonic>" 
```

### Step 2 — Rotate passphrase

```bash
./mscikdf rekey "oldpass" "newpass" "<mnemonic>" 
```

This produces a new mnemonic corresponding to the new passphrase.

### Step 3 — restore with the new passphrase

```bash
./mscikdf restore "newpass" "<new_mnemonic>" 
```

**Expected**:

All derived addresses remain unchanged.

### 3.3 Scenario 3 — Multi-Version Passphrase restore

**Sequence example:**

1. Generate with passphrase **P1**  
2. Rekey to **P2**  
3. Rekey again to **P3**

You will receive:

- `mnemonic_1` (paired with **P1**)  
- `mnemonic_2` (paired with **P2**)  
- `mnemonic_3` (paired with **P3**)  

Now verify:

```bash
./mscikdf restore "P1" "<mnemonic_1>" 
./mscikdf restore "P2" "<mnemonic_2>" 
./mscikdf restore "P3" "<mnemonic_3>" 
```
`
**Expected:**

- All three restores produce the **same multi-chain identity**.  
- No state is written to disk at any point.

---

### 3.4 Scenario 4 — Stateless restore

Demonstrates complete independence from external files or stored state.

**Step 1 — Remove all project files except the CLI and library**
```bash
rm -rf ./data ./cache ./config
```

**Step 2 — Reboot (stay offline)**

**Step 3 — restore**
```bash
./mscikdf restore "<passphrase>" "<mnemonic>" 
```

**Expected:**  
The same identity is reproduced despite having **no stored data** of any kind.

---

### 3.5 Additional Optional Tests

#### (a) Cross-curve independence  
Confirm that all key types (signing keys, encryption keys, chain-specific keys)  
are derived without leaking information across curves.

#### (b) X25519 identity verification  
Check that the same mnemonic + passphrase pair always yields the same encryption identity.

#### (c) Passphrase as a replaceable unlock factor  
Observe that changing the passphrase does not alter any derived identity.

---

## 4. Disclaimers

- This test suite provides **behavior-level evaluation tools only**.  
- No internal cryptographic implementation is disclosed.  
- Distributed binaries must **not** be reverse-engineered, modified, or decompiled.  
- Use is limited to **research, evaluation, and educational purposes**.  
- Not intended for production or commercial deployment.  
- No warranties are provided; **use at your own risk**.  

For questions or collaboration inquiries, please contact the author.

## 5. License

The MSCIKDF Behavior Verification Playground is released under MIT License.
