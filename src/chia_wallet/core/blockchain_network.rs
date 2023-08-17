use serde::Deserialize; // Import serde's Deserialize trait

#[derive(Debug, Deserialize)] // Derive the Deserialize trait for serialization
pub struct BlockchainNetwork {
    name: String,
    unit: Option<String>,          // Use Option<String> for the nullable field
    ticker: Option<String>,        // Use Option<String> for the nullable field
    address_prefix: String,        // Use snake_case for field names
    agg_sig_me_extra_data: String, // Use snake_case for field names
    precision: Option<i32>,        // Use Option<i32> for the nullable field
    fee: Option<i32>,              // Use Option<i32> for the nullable field
    network_config: serde_json::Value, // Use serde_json::Value for dynamic data
}
