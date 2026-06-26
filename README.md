# aether-braid-rs

**QIC B-Gate (Quantum Information Correction Braiding Gate) Simulation** *A Foundational Engine for the Decentralized Topological Quantum Network (DTQN)*

*Author: Robert Mourey Jr. | Kronova AetherNet* *ORCID: 0000-0002-4641-2743*

---

## Overview
`aether-braid-rs` is a zero-dependency, high-performance Rust Cargo crate that models the physics of fault-tolerant topological quantum computing. It serves as the algorithmic proof for the theories detailed in the arXiv preprint: **"The Fibonacci Architecture of Quantum Error Correction: Golden Ratio Dynamics in Topological Quantum Computing."**

The crate simulates a 3-qubit localized array, rigorously enforcing the **Golden Chain Hamiltonian** constraints. By mathematically forbidding "00" adjacencies (the trivial vacuum fusion channels), the algorithm applies the Quasicrystal Inflation Code (QIC) B-Gate formulation to filter out decoherence:

$$B_{gate} = R_I P_{local} + R_\tau (I - P_{local})$$

## The Vision: A Decentralized Topological Quantum Network (DTQN)
Maintaining a fragile, highly entangled quantum state across a distributed network of classical machines has historically been deemed impossible due to decoherence. `aether-braid-rs` bypasses this limitation by simulating the *topological logic* of quantum systems rather than physical qubits. 

By utilizing **Canton Network's** privacy-enabled blockchain architecture, the computational workload of this simulation can be sharded and executed across consumer-grade computing power. The Canton ledger ensures the mathematical state of the topological simulation remains consistent across all participating nodes, effectively creating a globally synchronized, fault-tolerant Decentralized Topological Quantum Network (DTQN) without the need for cryogenic supercomputers.

## Getting Started

Because this is a pure Rust implementation with zero external dependencies, running the simulation is incredibly fast and lightweight.

**Prerequisites:**
* [Rust & Cargo](https://rustup.rs/) installed.

**Build and Run:**
```bash
# Clone the repository
git clone git@github.com:kronova-intelligent-systems/aether-braid-rs.git
cd aether-braid-rs

# Build the crate
cargo build --release

# Run the 3-Qubit QIC Simulation
cargo run