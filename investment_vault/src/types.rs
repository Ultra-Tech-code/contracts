use soroban_sdk::{contracttype, Address};

#[contracttype]
pub enum VaultKey {
    Admin,
    UsdcSac,
    Registry,
    TotalShares,
    TotalInvestments,
    Balance(Address),
    Allowance(Address, Address),
    AllowanceExpiry(Address, Address),
    ProjectInvestment(u32),
}
