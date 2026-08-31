use rusqlite::params;
use crate::metrics::MetricSnapshot;
use super::SharedDb;

pub fn insert_snapshot(db: &SharedDb, snapshot: &MetricSnapshot) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let data = serde_json::to_string(snapshot).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO metric_snapshots (timestamp, data) VALUES (?1, ?2)",
        params![snapshot.timestamp.to_rfc3339(), data],
    ).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn get_recent_snapshots(db: &SharedDb, seconds: u64) -> Result<Vec<MetricSnapshot>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT data FROM metric_snapshots WHERE timestamp >= datetime('now', ?1) ORDER BY timestamp ASC"
    ).map_err(|e| e.to_string())?;

    let offset = format!("-{} seconds", seconds);
    let snapshots = stmt.query_map(params![offset], |row| {
        let data: String = row.get(0)?;
        Ok(data)
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .filter_map(|data| serde_json::from_str(&data).ok())
    .collect();

    Ok(snapshots)
}

pub fn prune_old_data(db: &SharedDb, retention_days: u64) -> Result<usize, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let deleted = conn.execute(
        "DELETE FROM metric_snapshots WHERE timestamp < datetime('now', ?1)",
        params![format!("-{} days", retention_days)],
    ).map_err(|e| e.to_string())?;
    Ok(deleted)
}
