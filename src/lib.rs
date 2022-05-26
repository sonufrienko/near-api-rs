//! NEAR API SDK

/// NEAR API SDK
pub struct Near {
    /// `node_url` is RPC endpoint
    pub node_url: String
}

impl Near {
    /// # Near constructor
    /// - mainnet `https://rpc.mainnet.near.org`
    /// - testnet <https://rpc.testnet.near.org>
    /// - betanet <https://rpc.betanet.near.org>
    /// - localnet <http://localhost:3030>
    /// ```
    /// let testnet = Near::new(String::from("https://rpc.testnet.near.org"));
    /// ```
    pub fn new(node_url: String) -> Near {
        Near { node_url }
    }
}

#[cfg(test)]
mod tests {
    use crate::Near;

    #[test]
    fn constructor() {
        let api = Near::new(String::from("https://rpc.testnet.near.org"));
        assert_eq!(api.node_url, String::from("https://rpc.testnet.near.org"));
    }
}
