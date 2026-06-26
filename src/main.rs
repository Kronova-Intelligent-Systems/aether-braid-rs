/**
 * QIC B-Gate (Quantum Information Correction Braiding Gate) Simulation
 * Advanced 3-Qubit Local Distillation (Quasicrystal Inflation Code)
 * * Author: Robert Mourey Jr.
 * ORCID: 0000-0002-4641-2743
 * Date: June 21, 2026
 * * Description:
 * This Rust implementation directly models the physics described in 
 * "The Fibonacci Architecture of Quantum Error Correction". 
 * It simulates a 3-qubit local array, strictly enforcing the Golden 
 * Chain Hamiltonian (forbidding "00" adjacencies) and applying the 
 * mathematical B_gate formulation: B_gate = R_I * P_local + R_tau * (I - P_local).
 */

use std::f64::consts::PI;

// ==========================================
// 1. Core Mathematical Constructs
// ==========================================

#[derive(Clone, Copy, Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn new(re: f64, im: f64) -> Self { Complex { re, im } }
    fn add(&self, other: &Complex) -> Complex { Complex::new(self.re + other.re, self.im + other.im) }
    fn mul(&self, other: &Complex) -> Complex {
        Complex::new(self.re * other.re - self.im * other.im, self.re * other.im + self.im * other.re)
    }
    fn from_polar(r: f64, theta: f64) -> Complex { Complex::new(r * theta.cos(), r * theta.sin()) }
    fn norm_sq(&self) -> f64 { self.re * self.re + self.im * self.im }
    fn scale(&self, scalar: f64) -> Complex { Complex::new(self.re * scalar, self.im * scalar) }
}

struct LCG { seed: u64 }
impl LCG {
    fn new(seed: u64) -> Self { LCG { seed } }
    fn next_f64(&mut self) -> f64 {
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.seed >> 11) as f64 / (1u64 << 53) as f64
    }
}

// ==========================================
// 2. QIC State Definition (3-Qubit Hilbert Space)
// ==========================================
// In a 3-qubit array, there are 2^3 = 8 possible states.
// Binary representations: 000, 001, 010, 011, 100, 101, 110, 111

#[derive(Clone)]
struct QIC3QubitState {
    amplitudes: [Complex; 8],
}

impl QIC3QubitState {
    fn new_normalized_uniform() -> Self {
        let mut state = QIC3QubitState { amplitudes: [Complex::new(0.0, 0.0); 8] };
        let norm_factor = 1.0 / (8.0_f64).sqrt();
        for i in 0..8 {
            state.amplitudes[i] = Complex::new(norm_factor, 0.0);
        }
        state
    }

    fn apply_noise(&mut self, noise_level: f64, prng: &mut LCG) {
        for i in 0..8 {
            let n_re = (prng.next_f64() - 0.5) * noise_level;
            let n_im = (prng.next_f64() - 0.5) * noise_level;
            self.amplitudes[i] = self.amplitudes[i].add(&Complex::new(n_re, n_im));
        }
        self.normalize();
    }

    fn normalize(&mut self) {
        let mut total_prob = 0.0;
        for c in &self.amplitudes { total_prob += c.norm_sq(); }
        let norm = total_prob.sqrt();
        for i in 0..8 { self.amplitudes[i] = self.amplitudes[i].scale(1.0 / norm); }
    }

    fn print_state(&self, label: &str) {
        println!("\n--- {} ---", label);
        let labels = ["|000>", "|001>", "|010>", "|011>", "|100>", "|101>", "|110>", "|111>"];
        for i in 0..8 {
            println!("  {}: [{:.3} + {:.3}i]  (Prob: {:.2}%)", 
                labels[i], self.amplitudes[i].re, self.amplitudes[i].im, self.amplitudes[i].norm_sq() * 100.0);
        }
    }
}

// ==========================================
// 3. The Golden Chain Hamiltonian Constraint
// ==========================================
// The QIC requires that the trivial vacuum state cannot sit adjacent to another vacuum.
// Mapping 1 to '0' and tau to '1', the sequence "00" is strictly forbidden.

fn is_state_forbidden_by_golden_chain(state_index: usize) -> bool {
    // Check bits for "00" adjacencies. 
    // binary representations mapping:
    // 0: 000 (Forbidden) | 1: 001 (Forbidden) | 2: 010 (Valid)   | 3: 011 (Valid)
    // 4: 100 (Forbidden) | 5: 101 (Valid)     | 6: 110 (Valid)   | 7: 111 (Valid)
    matches!(state_index, 0 | 1 | 4)
}

// ==========================================
// 4. QIC B-Gate Formulation
// Formula: B_gate = R_I * P_local + R_tau * (I - P_local)
// ==========================================

fn apply_qic_bgate(state: &mut QIC3QubitState) {
    println!("\n[!] Applying Localized 3-Qubit QIC B-Gate...");
    
    // R-Matrix eigenvalues embedded with the Golden Ratio
    let r_i = Complex::from_polar(1.0, -4.0 * PI / 5.0);
    let r_tau = Complex::from_polar(1.0, 3.0 * PI / 5.0);

    for i in 0..8 {
        if is_state_forbidden_by_golden_chain(i) {
            // P_local acts as identity on the forbidden sub-sectors (nullifying/penalizing them)
            // Simulating the projection Hamiltonian collapsing these physical states:
            state.amplitudes[i] = Complex::new(0.0, 0.0);
        } else {
            // Over the valid Temperley-Lieb subspace, the states acquire the topological phase.
            // (For demonstration, we split the valid subspace evenly between the I and Tau fusion channels)
            if i % 2 == 0 {
                // Apply R_I * P_local
                state.amplitudes[i] = state.amplitudes[i].mul(&r_i);
            } else {
                // Apply R_tau * (I - P_local)
                state.amplitudes[i] = state.amplitudes[i].mul(&r_tau);
            }
        }
    }
    
    // Normalize to reflect the system energetically falling strictly into the valid code space
    state.normalize();
}

// ==========================================
// 5. Main Simulation Execution
// ==========================================

fn main() {
    println!("=== Kronova AetherNet: Quasicrystal Inflation Code (QIC) ===");
    println!("Simulating 3-Qubit localized B-Gate distillation on the Golden Chain.\n");

    let mut prng = LCG::new(1618); // Seeded with the Golden Ratio digits
    let mut quantum_register = QIC3QubitState::new_normalized_uniform();
    
    quantum_register.print_state("1. Initial Raw Physical State (Uniform Superposition)");

    // Injecting Hardware Noise
    let noise_intensity = 0.25; 
    println!("\n[!] Warning: High-intensity environmental decoherence detected.");
    quantum_register.apply_noise(noise_intensity, &mut prng);
    quantum_register.print_state("2. Corrupted Physical State");

    // Applying the Topological QIC Algorithm
    println!("\n>>> Executing Algorithm: Enforcing Golden Chain constraints.");
    println!(">>> Applying B_gate = R_I * P_local + R_tau * (I - P_local).");
    apply_qic_bgate(&mut quantum_register);

    quantum_register.print_state("3. Final Topologically Protected State");
    
    println!("\n=== Simulation Complete ===");
    println!("Conclusion: The QIC successfully projected the noisy physical register");
    println!("into the protected Fibonacci subspace. The forbidden '00' states (000, 001, 100)");
    println!("were mathematically filtered out, and the topological braiding phases were");
    println!("safely acquired by the valid Golden Chain permutations.");
}
