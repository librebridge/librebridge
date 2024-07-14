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
    pub fn new(consensus: C, headers: Vec<Header>) -> Result<Self> {
        let mut consensus = consensus;

        let mut hs = BTreeMap::new();
        let mut ps = BTreeMap::new();
        let mut latest_height = 0;
        let mut latest_hash = FixedBytes::default();

        let mut begin_height = u64::MAX;

        for h in headers {
            let hash = h.hash_slow();

            if h.number > latest_height {
                latest_height = h.number;
                latest_hash = hash;
            }

            if h.number < begin_height {
                begin_height = h.number;
            }

            let param = consensus.params(&h)?;
            ps.insert(hash, param);
            hs.insert(hash, h);
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

            log::info!("Height: {current_height}, Hash: {current_hash} verified");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{BlockVerifier, ConsensusVerifier};

    struct MockConensusVerifier;

    impl ConsensusVerifier for MockConensusVerifier {
        type ConsensusParams = u64;

        fn params(&mut self, _header: &alloy::consensus::Header) -> Result<Self::ConsensusParams> {
            Ok(0)
        }

        fn verify(
            &self,
            _params: &Self::ConsensusParams,
            _next_header: &alloy::consensus::Header,
        ) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_verify_block() {
        let _ = env_logger::builder().is_test(true).try_init();

        let s = include_str!("../testdata/blocks.jsons");

        let splited = s.split('\n');

        let mut headers = Vec::new();

        for s in splited {
            if !s.is_empty() {
                let h: alloy::rpc::types::Header = serde_json::from_str(s).unwrap();
                let header: alloy::consensus::Header = h.try_into().unwrap();

                headers.push(header);
            }
        }

        let verifier = BlockVerifier::new(MockConensusVerifier, headers).unwrap();
        verifier.verify().unwrap()
    }
}
