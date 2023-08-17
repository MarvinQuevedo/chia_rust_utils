use serde::Deserialize; // Import serde's Deserialize trait

#[derive(Debug, Deserialize)] // Derive the Deserialize trait for serialization
pub struct BlockchainNetwork {
    pub name: String,
    pub unit: Option<String>,   // Use Option<String> for the nullable field
    pub ticker: Option<String>, // Use Option<String> for the nullable field
    pub address_prefix: String, // Use snake_case for field names
    pub agg_sig_me_extra_data: String, // Use snake_case for field names
    pub precision: Option<i32>, // Use Option<i32> for the nullable field
    pub fee: Option<i32>,       // Use Option<i32> for the nullable field
    pub network_config: serde_json::Value, // Use serde_json::Value for dynamic data
}
