use std::collections::BTreeMap;

use anyhow::Result;
use librebridge_verifier_core::ConsensusVerifier;

use crate::{BSCExtraData, Validator};

pub struct BSCConsensusVerifier {
    validators: BTreeMap<u64, Vec<Validator>>,
    block_per_epoch: u64,
}

pub struct BSCConsensusParams {
    epoch: u64,
}

impl ConsensusVerifier for BSCConsensusVerifier {
    type ConsensusParams = BSCConsensusParams;

    fn params(&mut self, header: &alloy::consensus::Header) -> Result<Self::ConsensusParams> {
        let is_epoch_start = header.number % self.block_per_epoch == 0;
        let epoch = header.number / self.block_per_epoch;

        let extradata = BSCExtraData::from_bytes(&header.extra_data, is_epoch_start)?;

        if header.number % self.block_per_epoch == 0 {
            self.validators.insert(epoch, extradata.validator_set);
        }

        Ok(BSCConsensusParams { epoch })
    }

    fn verify(
        &self,
        params: &Self::ConsensusParams,
        next_header: &alloy::consensus::Header,
    ) -> Result<()> {
        Ok(())
    }
}
