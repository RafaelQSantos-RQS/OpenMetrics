## Purpose

Armazena métricas coletadas em SQLite para permitir análise de tendências e gráficos históricos no dashboard.

## ADDED Requirements

### Requirement: SQLite schema for metrics
The system SHALL create and maintain a SQLite database with tables for time-series metric data, indexed by timestamp.

#### Scenario: Database initialization
- **WHEN** the application starts for the first time
- **THEN** the system creates the SQLite database and required tables if they don't exist

#### Scenario: Database exists
- **WHEN** the application starts and the database already exists
- **THEN** the system runs any necessary migrations and opens the existing database

### Requirement: Store metric snapshots
The system SHALL store complete metric snapshots with a timestamp at each collection interval.

#### Scenario: Snapshot stored on collection
- **WHEN** the metrics collection task completes a collection cycle
- **THEN** the system stores a timestamped snapshot of all metrics in SQLite

### Requirement: Query historical metrics
The system SHALL support querying metrics for a given time range (last hour, last 24h, last 7d, last 30d).

#### Scenario: Query last hour
- **WHEN** the dashboard requests metrics for the last hour
- **THEN** the system returns all snapshots from the past 60 minutes

#### Scenario: Query last 24 hours
- **WHEN** the dashboard requests metrics for the last 24 hours
- **THEN** the system returns all snapshots from the past 24 hours, potentially aggregated to reduce data points

### Requirement: Configurable data retention
The system SHALL support configurable retention periods (default: 30 days), automatically pruning old data.

#### Scenario: Retention pruning
- **WHEN** data older than the retention period exists
- **THEN** the system removes old records during periodic cleanup

#### Scenario: Default retention
- **WHEN** no retention period is configured
- **THEN** the system retains data for 30 days

### Requirement: Metric aggregation for long ranges
The system SHALL aggregate historical data (averages) when querying long time ranges to reduce payload size.

#### Scenario: Aggregation for 24h+
- **WHEN** querying more than 24 hours of data
- **THEN** the system returns averaged data points (e.g., per-minute averages) instead of raw snapshots
