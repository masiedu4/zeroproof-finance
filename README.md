# ZeroProof Finance

A zero-knowledge financial verification system that combines RISC Zero's zkVM with Cartesi Rollups for secure, private financial requirement verification. The system allows users to prove they meet specific financial criteria without revealing sensitive financial data.

## Technical Overview

### Architecture
1. **Proof Generation (RISC Zero)**
   - Uses RISC Zero's zkVM to generate zero-knowledge proofs of financial requirements
   - Processes raw financial data locally, keeping sensitive information private
   - Outputs a verifiable proof and a commitment to the data

2. **Verification Layer (Cartesi)**
   - Implements proof verification using Cartesi Rollups
   - Provides scalable, on-chain verification of RISC Zero proofs
   - Maintains an immutable record of verified proofs

### Key Components
- `host/`: Proof generation service
- `methods/`: RISC Zero guest code for ZK proof computation
- `financial-verify-rollup/`: Cartesi Rollup for on-chain verification
- `core/`: Shared types and utilities

## Prerequisites

1. Install [Rust](https://rustup.rs/) and required components:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install [Docker](https://docs.docker.com/get-docker/) for Cartesi environment

3. Install Cartesi CLI:
```bash
# Using Homebrew
brew install cartesi/tap/cartesi

# Or using NPM
npm install -g @cartesi/cli
```

4. Install Risc Zero zKVM
```
curl -L https://risczero.com/install | bash
rzup install
```

## Building and Running

1. **Generate Financial Proof**
```bash
# Clone and build the project
git clone https://github.com/masiedu4/zeroproof-finance
cd zeroproof-finance

# Generate a proof
cargo run
```

2. **Start Cartesi Verification Node**
```bash
cd financial-verify-rollup
cartesi build && cartesi run
```

3. **Submit Proof for Verification**
```bash
# In project root
./send_proof.sh
```

## System Components

### 1. Proof Generation (RISC Zero)
- Located in `host/` and `methods/`
- Processes financial data locally
- Generates ZK proofs using RISC Zero's zkVM
- Outputs:
  - Zero-knowledge proof of financial requirements
  - Commitment to financial data
  - Verification timestamp

### 2. Verification Layer (Cartesi)
- Located in `financial-verify-rollup/`
- Implements on-chain verification of RISC Zero proofs
- Maintains proof registry
- Handles state transitions and rollup execution

### 3. Core Library
- Located in `core/`
- Proof output format


## Security Considerations

1. **Zero-Knowledge Proofs**
   - Financial data never leaves the user's machine
   - Only proof of requirements is shared
   - Proofs are non-interactive,self-contained and verifiable

2. **On-Chain Verification**
   - Proof verification is done within Cartesi's secure environment
   - Results are committed to blockchain
   - Immutable audit trail of verifications

3. **Data Privacy**
   - Raw financial data remains private
   - Only binary verification results are public
   - Temporal validity through embedded timestamps


## Other Resources


