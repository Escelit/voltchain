use log::{info, warn};

/// Polls Soroban RPC for `TRADE` events emitted by the energy-trade contract
/// and syncs them into the local Postgres database.
///
/// TODO: Replace polling with a persistent cursor using `getEvents` + `startLedger`.
/// TODO: Parse event topics/data and upsert into `trades` table.
/// TODO: Run this as a background task via `tokio::spawn` in `main`.
pub async fn sync_trade_events(
    contract_id: &str,
    soroban_rpc_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if contract_id.is_empty() {
        warn!("sync_trade_events: CONTRACT_ID not set, skipping sync");
        return Ok(());
    }

    info!(
        "sync_trade_events: polling {} for TRADE events on contract {}",
        soroban_rpc_url, contract_id
    );

    // TODO: implement using soroban-client or raw RPC call:
    //
    // POST /getEvents
    // {
    //   "startLedger": <last_synced_ledger>,
    //   "filters": [{ "type": "contract", "contractIds": [contract_id],
    //                 "topics": [["TRADE"]] }]
    // }
    //
    // For each event: parse (prosumer, consumer, amount_kwh, price_per_kwh)
    // and upsert into the trades table.

    Ok(())
}
