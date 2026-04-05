//! Harness Iteration 1 — P2 xrpl-tests
//!
//! Orientation: exercise the Transaction deserializer against every
//! TransactionType string listed in the XRPL protocol reference.
//! https://xrpl.org/docs/references/protocol/transactions/types
//!
//! Run: cargo run --example harness_iter1 -p xrpl_api
//!
//! Expected outcome (pre-fix): known variants succeed, all others fail
//! with `unknown variant`.

use xrpl_api::Transaction;

/// Minimal JSON for any transaction type: only the fields required by
/// TransactionCommon (Account, Fee, Sequence) plus the tag.
fn tx_json(transaction_type: &str) -> String {
    format!(
        r#"{{
  "TransactionType": "{transaction_type}",
  "Account": "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
  "Fee": "12",
  "Sequence": 1
}}"#
    )
}

/// Payment requires extra mandatory fields.
fn payment_json() -> String {
    r#"{
  "TransactionType": "Payment",
  "Account": "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
  "Destination": "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX",
  "Amount": "1000000",
  "Fee": "12",
  "Sequence": 1
}"#
    .to_owned()
}

fn probe(label: &str, json: &str) {
    match serde_json::from_str::<Transaction>(json) {
        Ok(_) => println!("  OK   {label}"),
        Err(e) => println!("  ERR  {label}: {e}"),
    }
}

fn main() {
    println!("=== Harness Iteration 1: Transaction deserializer coverage ===\n");

    // ── variants present in the enum (should all succeed) ──────────────────
    println!("-- Currently supported variants --");
    probe("AccountDelete", &tx_json("AccountDelete"));
    probe("AccountSet", &tx_json("AccountSet"));
    probe("CheckCancel", &tx_json("CheckCancel"));
    probe("CheckCash", &tx_json("CheckCash"));
    probe("CheckCreate", &tx_json("CheckCreate"));
    probe("DepositPreauth", &tx_json("DepositPreauth"));
    probe("EscrowCancel", &tx_json("EscrowCancel"));
    probe("EscrowCreate", &tx_json("EscrowCreate"));
    probe("EscrowFinish", &tx_json("EscrowFinish"));
    probe("NFTokenAcceptOffer", &tx_json("NFTokenAcceptOffer"));
    probe("NFTokenBurn", &tx_json("NFTokenBurn"));
    probe("NFTokenCancelOffer", &tx_json("NFTokenCancelOffer"));
    probe("NFTokenCreateOffer", &tx_json("NFTokenCreateOffer"));
    probe("NFTokenMint", &tx_json("NFTokenMint"));
    probe("OfferCancel", &tx_json("OfferCancel"));
    probe("OfferCreate", &tx_json("OfferCreate"));
    probe("Payment", &payment_json());
    probe("PaymentChannelClaim", &tx_json("PaymentChannelClaim"));
    probe("PaymentChannelCreate", &tx_json("PaymentChannelCreate"));
    probe("PaymentChannelFund", &tx_json("PaymentChannelFund"));
    probe("SetRegularKey", &tx_json("SetRegularKey"));
    probe("SignerListSet", &tx_json("SignerListSet"));
    probe("TicketCreate", &tx_json("TicketCreate"));
    probe("TrustSet", &tx_json("TrustSet"));

    println!();

    // ── AMM family (added in Ledger v1.12) ─────────────────────────────────
    println!("-- AMM variants (missing) --");
    probe("AMMBid", &tx_json("AMMBid"));
    probe("AMMCreate", &tx_json("AMMCreate"));
    probe("AMMDelete", &tx_json("AMMDelete"));
    probe("AMMDeposit", &tx_json("AMMDeposit"));
    probe("AMMVote", &tx_json("AMMVote"));
    probe("AMMWithdraw", &tx_json("AMMWithdraw"));

    println!();

    // ── Clawback ────────────────────────────────────────────────────────────
    println!("-- Clawback variant (missing) --");
    probe("Clawback", &tx_json("Clawback"));

    println!();

    // ── DID family (added in Ledger v1.12) ─────────────────────────────────
    println!("-- DID variants (missing) --");
    probe("DIDDelete", &tx_json("DIDDelete"));
    probe("DIDSet", &tx_json("DIDSet"));

    println!();

    // ── Price Oracle family (added in Ledger v2.2) ─────────────────────────
    println!("-- Oracle variants (missing — triggered issue #41) --");
    probe("OracleDelete", &tx_json("OracleDelete"));
    probe("OracleSet", &tx_json("OracleSet"));

    println!();

    // ── XChain family (added in Ledger v2.0) ───────────────────────────────
    println!("-- XChain variants (missing) --");
    probe(
        "XChainAccountCreateCommit",
        &tx_json("XChainAccountCreateCommit"),
    );
    probe(
        "XChainAddAccountCreateAttestation",
        &tx_json("XChainAddAccountCreateAttestation"),
    );
    probe(
        "XChainAddClaimAttestation",
        &tx_json("XChainAddClaimAttestation"),
    );
    probe("XChainClaim", &tx_json("XChainClaim"));
    probe("XChainCommit", &tx_json("XChainCommit"));
    probe("XChainCreateBridge", &tx_json("XChainCreateBridge"));
    probe("XChainCreateClaimID", &tx_json("XChainCreateClaimID"));
    probe("XChainModifyBridge", &tx_json("XChainModifyBridge"));

    println!();

    // ── Pseudo-transactions ─────────────────────────────────────────────────
    println!("-- Pseudo-transaction variants (missing) --");
    probe("EnableAmendment", &tx_json("EnableAmendment"));
    probe("SetFee", &tx_json("SetFee"));
    probe("UNLModify", &tx_json("UNLModify"));

    println!();
    println!("=== Done ===");
}
