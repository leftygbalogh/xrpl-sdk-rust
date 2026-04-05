//! Harness Iteration 2 — P2 xrpl-tests
//!
//! Edge probing: boundary conditions, malformed inputs, ordering,
//! case sensitivity, and empty/minimal payloads.
//!
//! Run: cargo run --example harness_iter2 -p xrpl_api

use xrpl_api::Transaction;

fn probe(label: &str, json: &str) {
    match serde_json::from_str::<Transaction>(json) {
        Ok(tx) => println!("  OK   {label} [account={}]", tx.common().account),
        Err(e) => println!("  ERR  {label}: {e}"),
    }
}

fn main() {
    println!("=== Harness Iteration 2: Edge probing ===\n");

    // ── Case sensitivity ────────────────────────────────────────────────────
    println!("-- Case sensitivity on TransactionType tag --");
    probe(
        "lowercase 'payment'",
        r#"{"TransactionType":"payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1}"#,
    );
    probe(
        "UPPERCASE 'PAYMENT'",
        r#"{"TransactionType":"PAYMENT","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1}"#,
    );
    probe(
        "mixed case 'payMent'",
        r#"{"TransactionType":"payMent","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1}"#,
    );
    probe(
        "correct 'Payment' (control)",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1}"#,
    );
    println!();

    // ── Missing compulsory fields ───────────────────────────────────────────
    println!("-- Missing compulsory fields on known variants --");
    probe(
        "Payment missing Amount",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Fee":"12","Sequence":1}"#,
    );
    probe(
        "Payment missing Destination",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Amount":"1000000","Fee":"12","Sequence":1}"#,
    );
    probe(
        "Payment missing Account (TransactionCommon field)",
        r#"{"TransactionType":"Payment","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1}"#,
    );
    probe(
        "Payment missing Fee",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Sequence":1}"#,
    );
    probe(
        "Payment missing Sequence",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12"}"#,
    );
    probe(
        "TrustSet missing LimitAmount",
        r#"{"TransactionType":"TrustSet","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#,
    );
    probe(
        "OfferCreate missing TakerGets and TakerPays",
        r#"{"TransactionType":"OfferCreate","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#,
    );
    println!();

    // ── Field ordering ──────────────────────────────────────────────────────
    println!("-- Field order variations --");
    probe(
        "TransactionType last",
        r#"{"Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1,"TransactionType":"Payment"}"#,
    );
    probe(
        "TransactionType in middle",
        r#"{"Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","TransactionType":"Payment","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Sequence":1}"#,
    );
    probe(
        "Known variant (CheckCancel) — fields reversed",
        r#"{"Sequence":1,"Fee":"12","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","TransactionType":"CheckCancel"}"#,
    );
    println!();

    // ── Empty / degenerate payloads ─────────────────────────────────────────
    println!("-- Empty and degenerate payloads --");
    probe("Empty JSON object {}", r#"{}"#);
    probe("Empty string", r#""""#);
    probe("Null", r#"null"#);
    probe("JSON array instead of object", r#"[]"#);
    probe("Bare number", r#"42"#);
    probe(
        "TransactionType is null",
        r#"{"TransactionType":null,"Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#,
    );
    probe(
        "TransactionType is empty string",
        r#"{"TransactionType":"","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#,
    );
    probe(
        "TransactionType is a number",
        r#"{"TransactionType":7,"Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1}"#,
    );
    println!();

    // ── Extra / unknown fields ──────────────────────────────────────────────
    println!("-- Extra unknown fields alongside valid type --");
    probe(
        "Payment with unknown extra field",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1,"UnknownExtraField":"surprise"}"#,
    );
    probe(
        "Payment with nested unknown object",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1,"Meta":{"key":"value"}}"#,
    );
    probe(
        "Unknown type with extra fields",
        r#"{"TransactionType":"OracleSet","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1,"Provider":"deadbeef","AssetClass":"currency","LastUpdateTime":1000}"#,
    );
    println!();

    // ── Duplicate fields ────────────────────────────────────────────────────
    println!("-- Duplicate fields in JSON --");
    probe(
        "Duplicate TransactionType (last wins in serde_json?)",
        r#"{"TransactionType":"OracleSet","TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1}"#,
    );
    probe(
        "Duplicate Account field",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Account":"duplicate","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1}"#,
    );
    println!();

    println!("=== Done ===");
}
