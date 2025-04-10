use methods::{FINANCIAL_VERIFY_ELF, FINANCIAL_VERIFY_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::Serialize;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use zeroproof_core:: ProofOutput;

#[derive(Serialize)]
struct ProofData {
    input: String,  // Hex string containing receipt and image_id
}

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // Read the financial data
    let json_data = fs::read_to_string("res/financial.json")
        .expect("Failed to read financial data");
    
    
    // Get current timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    
    // Create the execution environment
    let env = ExecutorEnv::builder()
        .write(&json_data)
        .unwrap()
        .write(&timestamp)  // Pass timestamp to guest
        .unwrap()
        .build()
        .unwrap();

    // Generate proof
    println!("Generating financial verification proof...");
    let receipt = default_prover()
        .prove(env, FINANCIAL_VERIFY_ELF)
        .unwrap()
        .receipt;

    // Extract verification result
    let result: ProofOutput = receipt.journal.decode().unwrap();
    
    // Prepare proof data
    let receipt_bytes = bincode::serialize(&receipt).unwrap();
    

    let image_id_bytes: Vec<u8> = FINANCIAL_VERIFY_ID
        .iter()
        .flat_map(|&id| id.to_le_bytes().to_vec())
        .collect();

    // Combine receipt and image_id
    let mut combined_bytes = Vec::new();
    combined_bytes.extend_from_slice(&receipt_bytes);
    combined_bytes.extend_from_slice(&image_id_bytes);

    // Create proof data with hex encoding
    let proof_data = ProofData {
        input: hex::encode(&combined_bytes),
    };

    // Display verification results
    println!("\n=== VERIFICATION RESULTS ===");
    println!("Requirements met: {}", result.meets_requirements);
    println!("Verification timestamp: {}", result.timestamp);
    println!("Data hash: {:?}", result.json_hash);

    // Log the image ID
    println!("\n=== FINANCIAL VERIFY IMAGE ID ===");
    println!("const FINANCIAL_VERIFY_ID: [u32; 8] = [");
    for (i, &id) in FINANCIAL_VERIFY_ID.iter().enumerate() {
        if i < FINANCIAL_VERIFY_ID.len() - 1 {
            println!("    0x{:08x},", id);
        } else {
            println!("    0x{:08x}", id);
        }
    }
    println!("];");

    // Save proof to file
    fs::write(
        "proof_input.json",
        serde_json::to_string_pretty(&proof_data).unwrap(),
    )
    .unwrap();

    println!("\n=== PROOF STATS ===");
    println!("Proof size: {} bytes", combined_bytes.len());
    println!("Hex string length: {} chars", proof_data.input.len());
}
