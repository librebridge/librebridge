use alloy::{
    primitives::{Bytes, FixedBytes},
    rlp::{RlpDecodable, RlpEncodable},
};
use alloy_rlp::Decodable;
use anyhow::{anyhow, Result};

pub const EXTAR_VANITY_LENGTH: usize = 32;
pub const SEAL_LENGTH: usize = 65;
pub const VALIDATOR_ADDR_LENGTH: usize = 20;
pub const VALIDATOR_PUBLIC_KEY_LENGTH: usize = 48;

#[derive(Debug)]
pub struct BSCExtraData {
    pub extra_vanity: FixedBytes<EXTAR_VANITY_LENGTH>,
    pub validator_set: Vec<Validator>,
    pub attestation: Attestation,
    pub seal: FixedBytes<SEAL_LENGTH>,
}

impl BSCExtraData {
    pub fn from_bytes(bytes: &[u8], is_begin_of_epoch: bool) -> Result<Self> {
        let bytes_len = bytes.len();

        if bytes_len < EXTAR_VANITY_LENGTH + SEAL_LENGTH {
            return Err(anyhow!("The length of bytes too short"));
        }

        let extra_vanity = FixedBytes::from_slice(&bytes[0..EXTAR_VANITY_LENGTH]);
        let seal = FixedBytes::from_slice(&bytes[bytes_len - SEAL_LENGTH..bytes_len]);

        let mut validator_set = Vec::new();

        let attestation_begin = if is_begin_of_epoch {
            let validator_size = bytes[EXTAR_VANITY_LENGTH] as usize;

            let begin_of_validator_bytes = EXTAR_VANITY_LENGTH + 1;

            for i in 0..validator_size {
                let addr_idx = begin_of_validator_bytes
                    + i * (VALIDATOR_ADDR_LENGTH + VALIDATOR_PUBLIC_KEY_LENGTH);
                let pk_idx = addr_idx + VALIDATOR_ADDR_LENGTH;
                let pk_end = pk_idx + VALIDATOR_PUBLIC_KEY_LENGTH;

                validator_set.push(Validator {
                    addr: FixedBytes::from_slice(&bytes[addr_idx..pk_idx]),
                    public_key: FixedBytes::from_slice(&bytes[pk_idx..pk_end]),
                })
            }

            EXTAR_VANITY_LENGTH
                + 1
                + validator_size * (VALIDATOR_ADDR_LENGTH + VALIDATOR_PUBLIC_KEY_LENGTH)
        } else {
            EXTAR_VANITY_LENGTH
        };

        let ab = &mut &bytes[attestation_begin..bytes_len - SEAL_LENGTH];
        log::debug!("attestation bytes: {}", hex::encode(&ab));

        let attestation = Attestation::decode(ab)?;

        Ok(Self {
            extra_vanity,
            validator_set,
            attestation,
            seal,
        })
    }
}

#[derive(Debug)]
pub struct Validator {
    pub addr: FixedBytes<VALIDATOR_ADDR_LENGTH>,
    pub public_key: FixedBytes<VALIDATOR_PUBLIC_KEY_LENGTH>,
}

#[derive(Debug, RlpDecodable, RlpEncodable)]
pub struct Attestation {
    pub validator_bitset: u64,
    pub bls_signature: FixedBytes<96>,
    pub vote_data: VoteData,
    // Reserved for future usage.
    pub extra: Bytes,
}

#[derive(Debug, RlpDecodable, RlpEncodable)]
pub struct VoteData {
    pub source_number: u64,
    pub source_hash: FixedBytes<32>,
    pub target_number: u64,
    pub target_hash: FixedBytes<32>,
}

#[cfg(test)]
mod tests {
    use crate::BSCExtraData;

    #[test]
    fn test_decode_with_vs() {
        let _ = env_logger::builder().is_test(true).try_init();

        // This data from extradata field of `cast block 39030400 --rpc-url https://bsc.drpc.org`
        let data = "d88301030d846765746888676f312e32322e32856c696e75780000007b6ff50b151cfdbd2dff70c6e2e30df5012726f87731f381648e9879d77f0f25c8f6348135cef7477c2455f516bec180921d4c669eae7258857327674ace724b598c0df3fd41068ef837e9627a91dd13e453246856d58797ad6583d762abd04e3688a7c071dbc7eb3d0ace1c06baf163fdc8ffc742fec16f09fa468d30778a3c533b944899d33ae3225a3aee0738944092685a336cb6b9ea58836436709a2adc89afc1c041d36ee43ed51b1cb17b9dff14068e594b79a3c401a0bcf9fae9fd86324822fc0bb768f0b7dae76927b23d39543f349bbafec1551819b8be1efea2fc46ca749aa184248a459464eec1a21e7fc7b71a053d9644e9bb8da4853b8f872cd7c1d6b324bf1922829830646ceadfb658d3de009a460a252b4feefa821d3351731220627d7b7d1f3db3e34a6e7967c4da80dd3e5227acb02c92f33a026bbce5e52c19b7d8746b7e55d3e29b9083de0bf334fdf8ac91bc1485482ba86399ab6dcbe54071f8d22258688b4509b18bcfeba8fcafdc6b6f9016d5a0dd08e4685a13bffd8c2087f66bf7ca2dace7fbbc40c40824e30a84d3fe62a2ddcd52175009317fd4f6f8feea9dae41e5f0a4737bb7a7d5b3a0de43e5a979f8d7a9ad04f8f3f102bdbb17ef0bf6ac9a8fce3f110f409d99828be80295da56c2c7a7becd3647ce40502aecfe253e6aa0e8d2a06e12438ffed0fe16a0b15df58914a6b751909f0558a9f9af8efca7d46e480fc24478d977dafe7daf5161b38c72e9e1ea4865c288ef5b8054ab58567f7a51a58708c8b40ec592a38ba64c0697de8d78def84b10ab93dbfff6980d1054a2bc561bcf0abf3daf6096849bf03744fd4a49392e4813dc2251d68f6f95f27ba775b851a27d7101438f45fce31816501193239a83ad9a5f4ae5ec7dd886b09a47021461ec6f1971b3558f31e622311e94714398c80573fd531e0e8b4c4c3b456c4a5b9bf67b501c7944185130dd4ad73293e8aa84effdcee7b7adb10448b8be5fc875af7df065a5ee57f2ace2cca77b37bbe2e30fd16481afe8a64fe4dd3aff03d14fb180a05bf6a07e1fdf03eb3ac35bf0256694d7fbe6b6d7b3e0c8a066981de27634c2d17a68333f6d9b0c8cd7e08882c397c6ad92d95f8c279d6ef9ea04a1e2f4c4cdfe7ea6015f367ccb8a239732871adc8829ea2f47e94087c5fbad47b6adc9ae11a5f0da15082a4ded8abaeb73338984c06e2c1af2eb24232e00511e95b24e89291d689f6bedb13d5a398af2ecb218c5d6af1f979ac42bc68d98a5a0d796c6ab01b659ad0fbd9f515893fdd740b29ba0772dbde9b4635921dd91bd2963a0fc855e31f6338f45b211c4e9dedb7f2eb09de7b4647b856cb9c3856d559c885bed8b43e0846a47a663982486c84b2f66d9391efe6875d30be1d907e55d9c4a5a224de92a5d8ff180cc4ebca44253fa5a9730cc89d61994bdcc079bbb23c1d9a6f36aa31309676c258abac7a2564fd6f7101c1fdb441018a24d25672826953caa03b6717e26e8af1c38507dd570bdedf1cb270c376630823f101577c2d534f079444e6e7ff9dabb3fd8a26c607932c88633993fd05c4b6293e6f2b8429a628cfa00ac7551eff4940eb56f1f480094b1eff8eb73a9f958f164e944c27e00565cca503a7ed99eca485da2e875aedf7758472c378c91dce99bbdc44ee9500ed1e5c864bc88ba518585c7e6de5e94d26ee216dd8a5e06c5d2fc740123976c9588787b54998cccb42a9b8d6c46468900527bc741938e78ab4577b1e211be938b9f77888cbdc7bd3787148a6d1653eb0e17603df69d67db30fa4857e11edec461e01e3e02cea2c7d2c5ddf8b99643fafc79d9404de68e48c4d49a3936f7878d68efc0951aa89bf89fd829dd021fac27712f0f046e70d2d33a8a1fc05554a16d26669a9adb33bb41aaff43df7d4da0f8de5e61322302b2c6e0a525cc842f10332811bf8e69853df9edb142b5d596f93bfa14253a733cb9d2d5a7ed1fc345e248a8cae7f23f438930123eebf61c98785d846a8bf8b5831fffefb86082a3f908739ff54c85dc70d2a48425dcd0867d141c82c6c6b1a4daabcbe877d6e50519cba671785db6a03726afeda9b819b0b42321353c8689e3b18e7daa48b9730d6f9b1d32b002c394365aca7e16dbd49cb5d3883ccc336bf7202aab143082f84c8402538e7ea09ea53e521c482c3487a711e41cdd40079c05b079ac5905d1fef46cf87ccf63f08402538e7fa09a73f596d357164d10962a4214496debe2eb644ea4066487f52cb382b9a1d34780aa3c6ec7247326a2048dce25353aea2ba10b20122e52936adcd59acafc8a2e35791df271d4eb79c040a93791db7fe6bb250b2ca38ebdde46e10c5584b9b453f701";
        let data = hex::decode(data).unwrap();

        let extradata = BSCExtraData::from_bytes(&data, true).unwrap();

        println!("{:?}", extradata);
    }

    #[test]
    fn test_decode_without_vs() {
        let _ = env_logger::builder().is_test(true).try_init();

        let data = "d883010407846765746888676f312e32312e38856c696e757800000048fa7b05f8b5831fffefb8608390f2ba33b841a95b6c1754f49d807d479eb06d72dbc1338e77a54758442f21de4d8efb1b0125b784c2ac7ac8a12a8c19f71d9c9396b3d90c0dd656f109be10587de9b890ac2e981ba1a271d0785e0bead38adfd7712d096d10a66e371b984df84c8402538e7fa09a73f596d357164d10962a4214496debe2eb644ea4066487f52cb382b9a1d3478402538e80a0eef9d0f6560595c4647950af9b90440f643799859e3c3886742d66bd1102f410809261bb30fcccf4cc29ae93d79242b4e906bcec58d2f2653c8f9747ba1440cdb80e8b4b04c2c7ea60926b64aff272861839b016b543fdedb1757b47bca6f1d97f00";
        let data = hex::decode(data).unwrap();

        let extradata = BSCExtraData::from_bytes(&data, false).unwrap();

        println!("{:?}", extradata);
    }
}
