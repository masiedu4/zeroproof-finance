use serde::{Deserialize, Serialize};
use risc0_zkvm::sha::Digest;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FinancialData {
    pub bank_balance: u64,
    pub account_number: String,
    pub social_security: String,
    pub credit_score: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProofOutput {
    pub meets_requirements: bool,
    pub json_hash: Digest,
    pub timestamp: u64,
}