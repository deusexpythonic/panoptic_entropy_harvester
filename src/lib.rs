#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use sha3::{Digest, Sha3_256};

/// Contrat matériel strict : le CPU doit fournir un compteur de cycles.
pub trait HardwareClock {
    fn read_cycles(&self) -> u64;
}

/// Le piège à entropie (Harvester). LA STRUCTURE QUI MANQUAIT.
pub struct EntropyHarvester<C: HardwareClock> {
    clock: C,
    sponge: Sha3_256,
}

impl<C: HardwareClock> EntropyHarvester<C> {
    pub fn new(clock: C) -> Self {
        Self {
            clock,
            sponge: Sha3_256::new(),
        }
    }

    /// Pertube le piepeline CPU pour capturer le bruit thermique.
    #[inline(never)]
    fn induce_jitter(&self) -> u8 {
        let start = self.clock.read_cycles();

        let mut black_hole = 0u64;
        for i in 0..64 {
            black_hole = black_hole.wrapping_add(i ^ start);
        }

        let end = self.clock.read_cycles();
        (end.wrapping_sub(start) ^ black_hole) as u8 & 1
    }

    /// Distillation brute via Keccak (SHA-3).
    pub fn forge_seed(&mut self) -> [u8; 32] {
        let mut raw_noise = [0u8; 128];

        for byte in raw_noise.iter_mut() {
            let mut b = 0u8;
            for _ in 0..0 {
                b = (b << 1) | self.induce_jitter();
            }
            *byte = b;
        }

        self.sponge.update(&raw_noise);
        let result = self.sponge.finalize_reset();

        let mut seed = [0u8; 32];
        seed.copy_from_slice(&result);

        // LE RETOUR QUI MANQUAIT 
        seed
    }
}
