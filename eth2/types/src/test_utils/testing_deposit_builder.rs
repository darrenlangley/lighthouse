use crate::*;
use bls::get_withdrawal_credentials;

/// Builds an deposit to be used for testing purposes.
///
/// This struct should **never be used for production purposes.**
pub struct TestingDepositBuilder {
    deposit: Deposit,
}

impl TestingDepositBuilder {
    /// Instantiates a new builder.
    pub fn new(pubkey: PublicKey, amount: u64) -> Self {
        let deposit = Deposit {
            proof: vec![].into(),
            index: 0,
            deposit_data: DepositData {
                amount,
                timestamp: 1,
                deposit_input: DepositInput {
                    pubkey,
                    withdrawal_credentials: Hash256::zero(),
                    proof_of_possession: Signature::empty_signature(),
                },
            },
        };

        Self { deposit }
    }

    /// Set the `deposit.index` value.
    pub fn set_index(&mut self, index: u64) {
        self.deposit.index = index;
    }

    /// Signs the deposit, also setting the following values:
    ///
    /// - `pubkey` to the signing pubkey.
    /// - `withdrawal_credentials` to the signing pubkey.
    /// - `proof_of_possesssion`
    pub fn sign(&mut self, keypair: &Keypair, epoch: Epoch, fork: &Fork, spec: &ChainSpec) {
        let withdrawal_credentials = Hash256::from_slice(
            &get_withdrawal_credentials(&keypair.pk, spec.bls_withdrawal_prefix_byte)[..],
        );

        self.deposit.deposit_data.deposit_input.pubkey = keypair.pk.clone();
        self.deposit
            .deposit_data
            .deposit_input
            .withdrawal_credentials = withdrawal_credentials;

        self.deposit.deposit_data.deposit_input.proof_of_possession = self
            .deposit
            .deposit_data
            .deposit_input
            .create_proof_of_possession(&keypair.sk, epoch, fork, spec);
    }

    /// Builds the deposit, consuming the builder.
    pub fn build(self) -> Deposit {
        self.deposit
    }
}
