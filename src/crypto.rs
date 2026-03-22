// src/crypto.rs

/* Import modules. */
pub mod hashes;
pub mod math;

#[cfg(test)]
#[path = "crypto/math.test.rs"]
mod math_test;
