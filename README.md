# 🔐 Access Registry Smart Contract (Soroban / Stellar)

## 📖 Project Description

This project implements a permissioned access control smart contract using Soroban, the smart contract platform on the Stellar network.

The contract maintains a registry of approved addresses, allowing only authorized participants to interact with protected systems. It is particularly suited for fraud prevention, compliance enforcement, and controlled-access decentralized applications.

---

## ⚙️ What It Does

The contract introduces a simple whitelist-based access model:

- Initializes with a single admin account
- Allows the admin to approve trusted addresses
- Allows the admin to revoke access at any time
- Provides public verification of whether an address is approved
- Supports batch checking of multiple addresses

---

## ✨ Features

### 🔑 Secure Admin Control
- Contract is initialized with a single administrator
- All sensitive operations require admin authentication (`require_auth`)

### 📋 Address Whitelisting
- Store and manage a registry of approved addresses
- Efficient lookup using contract storage

### 🔍 Approval Verification
- Check if a specific address is approved
- Batch verification for multiple addresses in one call

### 🚫 Fraud Prevention Ready
- Restrict access to only vetted participants
- Easily revoke compromised or suspicious accounts

### ⚡ Lightweight & Efficient
- Minimal storage footprint
- Optimized for Soroban execution model
- No expensive on-chain iteration

---

## 🧱 Contract Structure

### Storage Keys

```rust
DataKey::Admin              // Stores admin address
DataKey::Approved(Address) // Maps address → approval status




Wallet address:GCWV5277AJCLZIPCBJG5VAAUMOS5ZM5H2JGPMCZ6Q6X3F4LN7VUFJEZ7

Contract address:CA7KSBLKQAHTY423BG6BSM7TPJUKIPEWDCEL2NFLYZMVZ6ORNPWI7LWM

https://stellar.expert/explorer/testnet/contract/CA7KSBLKQAHTY423BG6BSM7TPJUKIPEWDCEL2NFLYZMVZ6ORNPWI7LWM

<img width="1591" height="769" alt="image" src="https://github.com/user-attachments/assets/01f4df63-8494-444b-a376-f0452ec2cdf8" />
