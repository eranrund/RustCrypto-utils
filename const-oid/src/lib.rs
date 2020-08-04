//! Object Identifier (OID) constants with heapless `no_std` support.
//!
//! This crate supports creating [`ObjectIdentifier`] constants with
//! compile-time checks on validity.
//!
//! It has full `no_std` support with no dependencies on a heap/liballoc and
//! stores OID values as static data.

#![no_std]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo_small.png",
    html_root_url = "https://docs.rs/const-oid/0.1.0"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

#[cfg(test)]
extern crate std;

use core::fmt;

/// Object identifier (OID)
pub struct ObjectIdentifier {
    /// Nodes in this OID
    nodes: &'static [u32],

    // TODO(tarcieri): replace this with const panic when OIDs are invalid
    // See: https://github.com/rust-lang/rust/issues/51999
    is_valid: bool,
}

impl ObjectIdentifier {
    /// Create a new OID
    pub const fn new(nodes: &'static [u32]) -> Self {
        // TODO(tarcieri): replace this with const panic when OIDs are invalid
        let mut is_valid = nodes.len() > 2;

        match nodes[0] {
            0..=2 => (),
            _ => is_valid = false,
        }

        match nodes[1] {
            0..=39 => {}
            _ => is_valid = false,
        }

        Self { nodes, is_valid }
    }
}

impl AsRef<[u32]> for ObjectIdentifier {
    fn as_ref(&self) -> &[u32] {
        // TODO(tarcieri): move this check into OID parsing when const panic is stable
        if self.is_valid {
            self.nodes
        } else {
            panic!("invalid OID: {:?}", &self.nodes);
        }
    }
}

impl fmt::Display for ObjectIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, node) in self.as_ref().iter().enumerate() {
            write!(f, "{}", node)?;

            if i < self.as_ref().len() - 1 {
                write!(f, ".")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ObjectIdentifier;
    use std::string::ToString;

    const EXAMPLE_OID: ObjectIdentifier = ObjectIdentifier::new(&[1, 2, 840, 10045, 3, 1, 7]);

    #[test]
    fn display_test() {
        let oid = EXAMPLE_OID.to_string();
        assert_eq!(oid, "1.2.840.10045.3.1.7");
    }
}
