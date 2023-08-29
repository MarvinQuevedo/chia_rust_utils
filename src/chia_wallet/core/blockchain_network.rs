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

impl Clone for BlockchainNetwork {
    fn clone(&self) -> Self {
        BlockchainNetwork {
            name: self.name.clone(),
            unit: self.unit.clone(),
            ticker: self.ticker.clone(),
            address_prefix: self.address_prefix.clone(),
            agg_sig_me_extra_data: self.agg_sig_me_extra_data.clone(),
            precision: self.precision.clone(),
            fee: self.fee.clone(),
            network_config: self.network_config.clone(),
        }
    }
}
