use alloy::consensus::Header;
use anyhow::Result;

pub trait ConsensusVerifier {
    type ConsensusParams;

    fn params(&self, header: &Header) -> Result<Self::ConsensusParams>;

    fn verify(&self, params: &Self::ConsensusParams, next_header: &Header) -> Result<()>;
}
