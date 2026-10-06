//! Selected wallet and proving crates.
//!
//! Exactly one crate feature selects the concrete dependency family. This
//! module re-exports that family's crates under stable internal names so the
//! rest of the implementation remains backend-agnostic.

pub use voting_crypto_deps::{
    halo2_gadgets, halo2_proofs, incrementalmerkletree, pasta_curves, zip32,
};
pub use zakura_wallet_lib::{
    client_backend as zcash_client_backend, client_sqlite as zcash_client_sqlite,
    keys as zcash_keys, orchard, pczt, primitives as zcash_primitives,
};

#[cfg(feature = "lrz")]
pub use ::lrz_zcash_protocol as zcash_protocol;
#[cfg(feature = "zakura")]
pub use ::zcash_protocol;

/// Encodes `ufvk` as a ZIP 316 unified full viewing key string for `network`.
///
/// # Errors
///
/// Returns [`VotingError::InvalidInput`](crate::types::VotingError::InvalidInput)
/// if the key has no unified encoding.
pub(crate) fn encode_ufvk<P: zcash_protocol::consensus::Parameters>(
    ufvk: &zcash_keys::keys::UnifiedFullViewingKey,
    network: &P,
) -> Result<String, crate::types::VotingError> {
    #[cfg(feature = "lrz")]
    {
        ufvk.encode(network)
            .map_err(|e| crate::types::VotingError::InvalidInput {
                message: format!("cannot encode the unified full viewing key: {e}"),
            })
    }
    #[cfg(feature = "zakura")]
    {
        Ok(ufvk.encode(network))
    }
}
