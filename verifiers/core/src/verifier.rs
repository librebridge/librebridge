use std::collections::BTreeMap;

use alloy::{consensus::Header, primitives::FixedBytes};
use anyhow::{anyhow, Result};

use crate::ConsensusVerifier;

pub struct BlockVerifier<C>
where
    C: ConsensusVerifier,
{
    consensus: C,

    headers: BTreeMap<FixedBytes<32>, Header>,
    params: BTreeMap<FixedBytes<32>, C::ConsensusParams>,

    latest_hash: FixedBytes<32>,
    latest_height: u64,

    begin_height: u64,
}

impl<C> BlockVerifier<C>
where
    C: ConsensusVerifier,
{
    pub fn new(consensus: C, headers: &[Header]) -> Result<Self> {
        let mut hs = BTreeMap::new();
        let mut ps = BTreeMap::new();
        let mut latest_height = 0;
        let mut latest_hash = FixedBytes::default();

        let mut begin_height = u64::MAX;

        for h in headers {
            let hash = h.hash_slow();

            hs.insert(hash, h.clone());

            if h.number > latest_height {
                latest_height = h.number;
                latest_hash = hash;
            }

            if h.number < begin_height {
                begin_height = h.number;
            }

            let param = consensus.params(h)?;
            ps.insert(hash, param);
        }

        Ok(Self {
            headers: hs,
            consensus,
            latest_hash,
            latest_height,
            begin_height,
            params: ps,
        })
    }

    pub fn verify(&self) -> Result<()> {
        let mut current_height = self.latest_height;
        let mut current_hash = self.latest_hash;

        while current_height > self.begin_height {
            let h = self.headers.get(&current_hash).ok_or(anyhow!(
                "Failed to get header by hash, it means chain of blockheader are broken"
            ))?;

            if h.number != current_height {
                return Err(anyhow!("Wrong number of header"));
            }

            current_height -= 1;
            current_hash = h.parent_hash;

            let consensus_params = self.params.get(&h.parent_hash).ok_or(anyhow!(
                "Failed to get consensus params by hash, it means chain of blockheader are broken",
            ))?;

            self.consensus.verify(consensus_params, h)?;
        }

        Ok(())
    }
}
