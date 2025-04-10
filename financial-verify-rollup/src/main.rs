use hex;
use json::{object, JsonValue};
use risc0_zkvm::Receipt;
use serde::Deserialize;
use serde_json;
use std::env;
use zeroproof_core::ProofOutput;

// This is the image ID of the RISC Zero program we want to verify
// You should replace this with your own image ID
const FINANCIAL_VERIFY_ID: [u32; 8] = [
    0x8a6f7f28,
    0x572522d0,
    0x07b3694d,
    0x47ba3cac,
    0x28ac4051,
    0xa5673ade,
    0xaf65f1d1,
    0x3a0dd777
];

#[derive(Deserialize)]
struct ProofData {
    input: String, // o match host program output
}

async fn verify_financial_proof(payload: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Remove '0x' prefix if present
    let clean_payload = payload.trim_start_matches("0x");

    // Decode the hex-encoded payload
    let combined_bytes = hex::decode(clean_payload)?;

    // Debug output
    println!("Received payload length: {} bytes", combined_bytes.len());
    
    // The image ID is 8 u32 values (32 bytes)
    const IMAGE_ID_SIZE: usize = 32;
    
    // Make sure we have enough data
    if combined_bytes.len() <= IMAGE_ID_SIZE {
        return Err("Payload too small to contain receipt and image ID".into());
    }

    // Split the payload into receipt and image ID
    let receipt_bytes = &combined_bytes[..combined_bytes.len() - IMAGE_ID_SIZE];
    let image_id_bytes = &combined_bytes[combined_bytes.len() - IMAGE_ID_SIZE..];

    println!("Receipt length: {} bytes", receipt_bytes.len());
    println!("Image ID length: {} bytes", image_id_bytes.len());
    

    let receipt: Receipt = match bincode::deserialize(receipt_bytes) {
        Ok(r) => r,
        Err(e) => {
            println!("Deserialization error: {}", e);
            return Err(format!("Failed to deserialize receipt: {}", e).into());
        }
    };

    // Verify the receipt against the expected image ID
    match receipt.verify(FINANCIAL_VERIFY_ID) {
        Ok(_) => {
            // Extract and log the result from the journal
            let result: ProofOutput = receipt.journal.decode()?;
            println!("\n=== Verification Results ===");
            println!("Requirements met: {}", result.meets_requirements);
            println!("Data hash: {:?}", result.json_hash);
            println!("Timestamp: {}", result.timestamp);
            Ok(())
        },
        Err(e) => {
            println!("Receipt verification failed: {}", e);
            
            // Try to extract the image ID from the payload and compare
            let extracted_id = if image_id_bytes.len() == 32 {
                let mut id = [0u32; 8];
                for i in 0..8 {
                    let bytes = &image_id_bytes[i*4..(i+1)*4];
                    id[i] = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                }
                
                println!("Extracted image ID: {:?}", id);
                println!("Expected image ID: {:?}", FINANCIAL_VERIFY_ID);
                
                if id != FINANCIAL_VERIFY_ID {
                    return Err("Image ID mismatch".into());
                }
            };
            
            Err(format!("Receipt verification failed: {}", e).into())
        }
    }
}

pub async fn handle_advance(
    _client: &hyper::Client<hyper::client::HttpConnector>,
    _server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received advance request data {}", &request);
    let payload = request["data"]["payload"]
        .as_str()
        .ok_or("Missing payload")?;

    match verify_financial_proof(payload).await {
        Ok(()) => {
            println!("Proof verified successfully!");
            Ok("accept")
        }
        Err(e) => {
            println!("Proof verification failed: {}", e);
            Ok("reject")
        }
    }
}

pub async fn handle_inspect(
    _client: &hyper::Client<hyper::client::HttpConnector>,
    _server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received inspect request data {}", &request);
    let _payload = request["data"]["payload"]
        .as_str()
        .ok_or("Missing payload")?;
    // TODO: add application logic here
    Ok("accept")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = hyper::Client::new();
    let server_addr = env::var("ROLLUP_HTTP_SERVER_URL")?;

    let mut status = "accept";
    loop {
        println!("Sending finish");
        let response = object! {"status" => status};
        let request = hyper::Request::builder()
            .method(hyper::Method::POST)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .uri(format!("{}/finish", &server_addr))
            .body(hyper::Body::from(response.dump()))?;
        let response = client.request(request).await?;
        println!("Received finish status {}", response.status());

        if response.status() == hyper::StatusCode::ACCEPTED {
            println!("No pending rollup request, trying again");
        } else {
            let body = hyper::body::to_bytes(response).await?;
            let utf = std::str::from_utf8(&body)?;
            let req = json::parse(utf)?;

            let request_type = req["request_type"]
                .as_str()
                .ok_or("request_type is not a string")?;
            status = match request_type {
                "advance_state" => handle_advance(&client, &server_addr[..], req).await?,
                "inspect_state" => handle_inspect(&client, &server_addr[..], req).await?,
                &_ => {
                    eprintln!("Unknown request type");
                    "reject"
                }
            };
        }
    }
}