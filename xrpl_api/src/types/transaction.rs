mod common;
mod variants;

use serde::{Serialize, Deserialize};

pub use common::*;

pub use variants::account_delete::*;
pub use variants::account_set::*;
pub use variants::offer_cancel::*;
pub use variants::offer_create::*;
pub use variants::payment::*;
pub use variants::trust_set::*;

/// Ledger transaction. See <https://xrpl.org/transaction-formats.html>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "TransactionType")]
pub enum Transaction {
    AccountDelete(AccountDeleteTransaction),
    AccountSet(AccountSetTransaction),
    // TODO add model for remaining transactions
    CheckCancel(TransactionCommon),
    CheckCash(TransactionCommon),
    CheckCreate(TransactionCommon),
    DepositPreauth(TransactionCommon),
    EscrowCancel(TransactionCommon),
    EscrowCreate(TransactionCommon),
    EscrowFinish(TransactionCommon),
    NFTokenAcceptOffer(TransactionCommon),
    NFTokenBurn(TransactionCommon),
    NFTokenCancelOffer(TransactionCommon),
    NFTokenCreateOffer(TransactionCommon),
    NFTokenMint(TransactionCommon),
    OfferCancel(OfferCancelTransaction),
    OfferCreate(OfferCreateTransaction),
    Payment(PaymentTransaction),
    PaymentChannelClaim(TransactionCommon),
    PaymentChannelCreate(TransactionCommon),
    PaymentChannelFund(TransactionCommon),
    SetRegularKey(TransactionCommon),
    SignerListSet(TransactionCommon),
    TicketCreate(TransactionCommon),
    TrustSet(TrustSetTransaction),
}

impl Transaction {
    pub fn common(&self) -> &TransactionCommon {
        match self {
            Transaction::AccountDelete(t) => &t.common,
            Transaction::AccountSet(t) => &t.common,
            Transaction::OfferCancel(t) => &t.common,
            Transaction::OfferCreate(t) => &t.common,
            Transaction::Payment(t) => &t.common,
            Transaction::TrustSet(t) => &t.common,
            Transaction::CheckCancel(t) => t,
            Transaction::CheckCash(t) => t,
            Transaction::CheckCreate(t) => t,
            Transaction::DepositPreauth(t) => t,
            Transaction::EscrowCancel(t) => t,
            Transaction::EscrowCreate(t) => t,
            Transaction::EscrowFinish(t) => t,
            Transaction::NFTokenAcceptOffer(t) => t,
            Transaction::NFTokenBurn(t) => t,
            Transaction::NFTokenCancelOffer(t) => t,
            Transaction::NFTokenCreateOffer(t) => t,
            Transaction::NFTokenMint(t) => t,
            Transaction::PaymentChannelClaim(t) => t,
            Transaction::PaymentChannelCreate(t) => t,
            Transaction::PaymentChannelFund(t) => t,
            Transaction::SetRegularKey(t) => t,
            Transaction::SignerListSet(t) => t,
            Transaction::TicketCreate(t) => t,
        }
    }
}

impl Transaction {
    pub fn common_mut(&mut self) -> &mut TransactionCommon {
        match self {
            Transaction::AccountDelete(t) => &mut t.common,
            Transaction::AccountSet(t) => &mut t.common,
            Transaction::OfferCancel(t) => &mut t.common,
            Transaction::OfferCreate(t) => &mut t.common,
            Transaction::Payment(t) => &mut t.common,
            Transaction::TrustSet(t) => &mut t.common,
            Transaction::CheckCancel(t) => t,
            Transaction::CheckCash(t) => t,
            Transaction::CheckCreate(t) => t,
            Transaction::DepositPreauth(t) => t,
            Transaction::EscrowCancel(t) => t,
            Transaction::EscrowCreate(t) => t,
            Transaction::EscrowFinish(t) => t,
            Transaction::NFTokenAcceptOffer(t) => t,
            Transaction::NFTokenBurn(t) => t,
            Transaction::NFTokenCancelOffer(t) => t,
            Transaction::NFTokenCreateOffer(t) => t,
            Transaction::NFTokenMint(t) => t,
            Transaction::PaymentChannelClaim(t) => t,
            Transaction::PaymentChannelCreate(t) => t,
            Transaction::PaymentChannelFund(t) => t,
            Transaction::SetRegularKey(t) => t,
            Transaction::SignerListSet(t) => t,
            Transaction::TicketCreate(t) => t,
        }
    }
}

// BASELINE: tests for missing TransactionType variants — see issue #41
#[cfg(test)]
mod tests {
    use super::Transaction;

    // --- AMM family ---

    #[test]
    fn test_deserialize_amm_bid() {
        let json = r#"{"TransactionType":"AMMBid","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_amm_create() {
        let json = r#"{"TransactionType":"AMMCreate","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_amm_delete() {
        let json = r#"{"TransactionType":"AMMDelete","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_amm_deposit() {
        let json = r#"{"TransactionType":"AMMDeposit","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_amm_vote() {
        let json = r#"{"TransactionType":"AMMVote","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_amm_withdraw() {
        let json = r#"{"TransactionType":"AMMWithdraw","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    // --- Clawback ---

    #[test]
    fn test_deserialize_clawback() {
        let json = r#"{"TransactionType":"Clawback","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    // --- DID family ---

    #[test]
    fn test_deserialize_did_delete() {
        let json = r#"{"TransactionType":"DIDDelete","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_did_set() {
        let json = r#"{"TransactionType":"DIDSet","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    // --- Oracle family ---

    #[test]
    fn test_deserialize_oracle_delete() {
        let json = r#"{"TransactionType":"OracleDelete","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_oracle_set() {
        let json = r#"{"TransactionType":"OracleSet","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    // --- XChain family ---

    #[test]
    fn test_deserialize_x_chain_account_create_commit() {
        let json = r#"{"TransactionType":"XChainAccountCreateCommit","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_add_account_create_attestation() {
        let json = r#"{"TransactionType":"XChainAddAccountCreateAttestation","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_add_claim_attestation() {
        let json = r#"{"TransactionType":"XChainAddClaimAttestation","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_claim() {
        let json = r#"{"TransactionType":"XChainClaim","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_commit() {
        let json = r#"{"TransactionType":"XChainCommit","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_create_bridge() {
        let json = r#"{"TransactionType":"XChainCreateBridge","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_create_claim_id() {
        let json = r#"{"TransactionType":"XChainCreateClaimID","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_x_chain_modify_bridge() {
        let json = r#"{"TransactionType":"XChainModifyBridge","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    // --- Pseudo-transactions ---

    #[test]
    fn test_deserialize_enable_amendment() {
        let json = r#"{"TransactionType":"EnableAmendment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_set_fee() {
        let json = r#"{"TransactionType":"SetFee","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    #[test]
    fn test_deserialize_unl_modify() {
        let json = r#"{"TransactionType":"UNLModify","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        serde_json::from_str::<Transaction>(json).unwrap();
    }

    // --- Unknown catchall (currently FAILS — no catchall variant exists) ---

    #[test]
    fn test_deserialize_unknown_variant_does_not_panic() {
        // Currently FAILS — no catchall variant exists
        let json = r#"{"TransactionType":"SomeFutureVariant","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#;
        let tx: Transaction = serde_json::from_str(json).unwrap();
        // After fix: should deserialize to Transaction::Unknown
        // common() on Unknown is not intended — do not call it here
        let _ = tx; // silence unused warning
    }

    // --- Regression: existing variants must survive the fix ---

    #[test]
    fn test_deserialize_existing_variants_still_work() {
        // These must continue to pass after the fix
        let variants = vec![
            r#"{"TransactionType":"CheckCancel","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#,
            r#"{"TransactionType":"EscrowCreate","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#,
            r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1}"#,
        ];
        for json in variants {
            serde_json::from_str::<Transaction>(json).unwrap();
        }
    }
}
