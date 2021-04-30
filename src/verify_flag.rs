use ::{height_to_flags, VERIFY_ALL};

struct VerifyFlag(u32);

// Enforce SCRIPT_VERIFY_P2SH and SCRIPT_VERIFY_WITNESS from genesis
// https://github.com/bitcoin/bitcoin/pull/11739



impl VerifyFlag {
    fn from_height(network: Network, height: u32) -> Self {
        VerifyFlag(match network {
            Network::Bitcoin => height_to_flags(height),
            Network::Testnet => {

            }
            Netowrk::Signet => VERIFY_ALL,
            Netowrk::Regtest => VERIFY_ALL,

        })

    }
}

impl From<VerifyFlag> for u32 {
    fn from(flag: VerifyFlag) -> Self {
        flag.0
    }
}