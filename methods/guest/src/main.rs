use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};
use zeroproof_core::{FinancialData, ProofOutput};
use serde_json;

// Financial requirements as constants
const MIN_BALANCE: u64 = 40_000;        // $40,000 minimum
const MIN_CREDIT_SCORE: u32 = 700;      // Minimum credit score

fn main() {
    // Read the JSON data and timestamp
    let json_data: String = env::read();
    let timestamp: u64 = env::read(); // Read timestamp from host
    
    // Calculate hash of original data
    let json_hash = *Impl::hash_bytes(&json_data.as_bytes());
    
    // Parse the financial data
    let financial_data: FinancialData = serde_json::from_str(&json_data)
        .expect("Failed to parse financial data");
    
    // Verify against hardcoded requirements
    let meets_requirements = financial_data.bank_balance >= MIN_BALANCE 
        && financial_data.credit_score >= MIN_CREDIT_SCORE;
    
    // Create verification result
    let result = ProofOutput {
        meets_requirements,
        json_hash,
        timestamp,
    };
    
    // Commit the result to the journal
    env::commit(&result);
}
