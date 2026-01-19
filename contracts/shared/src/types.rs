use soroban_sdk::{contracttype, Address, String};

/// Common timestamp type
pub type Timestamp = u64;

/// Common amount type
pub type Amount = i128;

/// Common percentage type (in basis points, 10000 = 100%)
pub type BasisPoints = u32;

/// Platform fee configuration
#[contracttype]
#[derive(Clone)]
pub struct FeeConfig {
    pub platform_fee: BasisPoints,  // Platform fee in basis points
    pub creator_fee: BasisPoints,    // Creator fee in basis points
    pub fee_recipient: Address,      // Address to receive fees
}

/// Token information
#[contracttype]
#[derive(Clone)]
pub struct TokenInfo {
    pub address: Address,
    pub symbol: String,
    pub decimals: u32,
}

/// User profile
#[contracttype]
#[derive(Clone)]
pub struct UserProfile {
    pub address: Address,
    pub reputation_score: i128,
    pub projects_created: u32,
    pub projects_funded: u32,
    pub total_contributed: Amount,
    pub verified: bool,
}
