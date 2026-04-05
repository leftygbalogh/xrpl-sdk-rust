//! Harness Iteration 2b — Wrong type probing
//!
//! What happens when fields receive values of the wrong JSON type?
//!
//! Run: cargo run --example harness_iter2b -p xrpl_api

use xrpl_api::Transaction;

fn probe(label: &str, json: &str) {
    match serde_json::from_str::<Transaction>(json) {
        Ok(tx) => println!("  OK   {label} [account={}]", tx.common().account),
        Err(e) => println!("  ERR  {label}: {e}"),
    }
}

fn main() {
    println!("=== Harness Iteration 2b: Wrong-type field probing ===\n");

    // ── TransactionCommon fields with wrong types ───────────────────────────
    println!("-- TransactionCommon fields with wrong types --");
    probe(
        "Account is a number",
        r#"{"TransactionType":"CheckCancel","Account":42,"Fee":"12","Sequence":1}"#,
    );
    probe(
        "Account is null",
        r#"{"TransactionType":"CheckCancel","Account":null,"Fee":"12","Sequence":1}"#,
    );
    probe(
        "Account is an object",
        r#"{"TransactionType":"CheckCancel","Account":{"key":"value"},"Fee":"12","Sequence":1}"#,
    );
    probe(
        "Fee is a number (should be string per XRPL spec)",
        r#"{"TransactionType":"CheckCancel","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":12,"Sequence":1}"#,
    );
    probe(
        "Fee is null",
        r#"{"TransactionType":"CheckCancel","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":null,"Sequence":1}"#,
    );
    probe(
        "Sequence is a string",
        r#"{"TransactionType":"CheckCancel","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":"1"}"#,
    );
    probe(
        "Sequence is a float",
        r#"{"TransactionType":"CheckCancel","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1.5}"#,
    );
    probe(
        "Sequence is negative (u32 overflow)",
        r#"{"TransactionType":"CheckCancel","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":-1}"#,
    );
    probe(
        "Sequence is u32::MAX + 1 (overflow)",
        r#"{"TransactionType":"CheckCancel","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":4294967296}"#,
    );
    println!();

    // ── Payment-specific fields with wrong types ────────────────────────────
    println!("-- Payment variant: wrong types on variant-specific fields --");
    probe(
        "Amount is an object (IOU form) — should be OK",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":{"currency":"USD","value":"1","issuer":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn"},"Fee":"12","Sequence":1}"#,
    );
    probe(
        "Amount is null",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":null,"Fee":"12","Sequence":1}"#,
    );
    probe(
        "Amount is a number (should be string for XRP drops)",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":1000000,"Fee":"12","Sequence":1}"#,
    );
    probe(
        "Amount is an array",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":[],"Fee":"12","Sequence":1}"#,
    );
    probe(
        "Destination is a number",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":99,"Amount":"1000000","Fee":"12","Sequence":1}"#,
    );
    probe(
        "DestinationTag is a string (should be u32)",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1,"DestinationTag":"notanumber"}"#,
    );
    println!();

    // ── Optional TransactionCommon fields with wrong types ──────────────────
    println!("-- Optional TransactionCommon fields: wrong types --");
    probe(
        "SourceTag is a string (should be u32)",
        r#"{"TransactionType":"CheckCancel","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":1,"SourceTag":"abc"}"#,
    );
    probe(
        "Flags is a string (should be bitflag u64)",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1,"Flags":"2147483648"}"#,
    );
    probe(
        "Flags is null",
        r#"{"TransactionType":"Payment","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Destination":"ra5nK24KXen9AHvsdFTKHSANinZseWnPcX","Amount":"1000000","Fee":"12","Sequence":1,"Flags":null}"#,
    );
    println!();

    println!("=== Done ===");
}
