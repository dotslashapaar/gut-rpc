use solana_compute_budget::compute_budget::ComputeBudget;

#[cfg(feature = "frozen-abi")]
impl ::solana_frozen_abi::abi_example::AbiExample for RuntimeConfig {
    fn example() -> Self {
        // RuntimeConfig is not Serialize so just rely on Default.
        RuntimeConfig::default()
    }
}

/// Encapsulates flags that can be used to tweak the runtime behavior.
#[derive(Debug, Default, Clone)]
pub struct RuntimeConfig {
    pub compute_budget: Option<ComputeBudget>,
    pub log_messages_bytes_limit: Option<usize>,
    pub transaction_account_lock_limit: Option<usize>,
    /// When true, this node is a non-voting RPC node.
    /// Skips consensus-only validation on the replay write path.
    pub is_rpc_mode: bool,
}
