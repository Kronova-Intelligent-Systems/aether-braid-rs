# **aether-braid-rs**

**QIC B-Gate (Quantum Information Correction Braiding Gate) Simulation**

*Author: Robert Mourey Jr. | Kronova AetherNet* *ORCID: 0000-0002-4641-2743*

⚠️ **Note:** This is a temporary README. The full repository is currently being transitioned into a formal Rust Cargo crate (aether-braid-rs), complete with an extensive unit testing suite and modularized complex math engine. This update will be pushed shortly.

## **Overview**

This repository contains a zero-dependency, high-performance Rust implementation modeling the physics described in my forthcoming arXiv paper: **"The Fibonacci Architecture of Quantum Error Correction: Golden Ratio Dynamics in Topological Quantum Computing."**

It specifically simulates a 3-qubit localized array, rigorously enforcing the **Golden Chain Hamiltonian** constraints. By mathematically forbidding "00" adjacencies (the trivial vacuum fusion channels), the algorithm successfully applies the Quasicrystal Inflation Code (QIC) B-Gate formulation:

![image1][image1]

## How It Works**

Rather than relying on active, software-driven syndrome measurements (which require massive qubit overhead), this code demonstrates **topological error correction**.

1. The simulation initializes an 8-dimensional Hilbert space (3 qubits).  
2. It introduces a high-intensity simulated environmental noise/decoherence factor.  
3. It applies the QIC algorithm, which utilizes the Golden Ratio ![image2][image2] to mathematically filter out non-topological errors and safely acquire the braided topological phases for valid permutations.

## **Running the Simulation**

While the Cargo crate is being finalized, you can compile and run the standalone simulation directly using rustc:

rustc qic-bgate-sim.rs  
./qic-bgate-sim.rs

## **References**

This code acts as the functional, algorithmic proof for the theories discussed in my research paper. A direct link to the published quant-ph arXiv preprint will be added here once the endorsement and publication process is complete.

*Built for Kronova AetherNet*

[image1]: https://quantumone.b-cdn.net/kronova/Shape1_light.svg

[image2]: https://quantumone.b-cdn.net/kronova/Shape2_light.svg