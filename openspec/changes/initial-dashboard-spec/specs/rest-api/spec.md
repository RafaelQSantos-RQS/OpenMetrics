## Purpose

Expõe endpoints REST JSON para consultas externas de métricas, permitindo integração com outros sistemas e ferramentas de monitoramento.

## ADDED Requirements

### Requirement: Current metrics endpoint
The system SHALL provide a GET endpoint that returns the latest metric snapshot as JSON.

#### Scenario: Get current metrics
- **WHEN** a client sends GET /api/v1/metrics/current
- **THEN** the system returns a JSON object with all current metrics (CPU, RAM, disk, network, processes, system info)

#### Scenario: Unauthenticated API access
- **WHEN** a client sends GET /api/v1/metrics/current without a valid session
- **THEN** the system returns HTTP 401 Unauthorized

### Requirement: Historical metrics endpoint
The system SHALL provide a GET endpoint that returns historical metric data for a given time range.

#### Scenario: Get historical metrics
- **WHEN** a client sends GET /api/v1/metrics/history?range=1h
- **THEN** the system returns a JSON array of metric snapshots from the last hour

#### Scenario: Invalid time range
- **WHEN** a client sends an invalid range parameter
- **THEN** the system returns HTTP 400 Bad Request with a descriptive error message

### Requirement: Supported time ranges
The system SHALL support the following time range query parameters: `1h`, `6h`, `24h`, `7d`, `30d`.

#### Scenario: Valid range values
- **WHEN** a client sends range=24h
- **THEN** the system returns data from the last 24 hours (aggregated if needed)

### Requirement: Specific metric endpoint
The system SHALL provide a GET endpoint to query a single metric type (cpu, memory, disk, network).

#### Scenario: Get CPU history
- **WHEN** a client sends GET /api/v1/metrics/cpu?range=1h
- **THEN** the system returns only CPU-related metrics for the last hour

#### Scenario: Unknown metric type
- **WHEN** a client sends an invalid metric type
- **THEN** the system returns HTTP 404 Not Found with available metric types listed

### Requirement: API response format
The system SHALL return all API responses in JSON with consistent structure: `{"data": ..., "timestamp": ..., "meta": {...}}`.

#### Scenario: Successful response
- **WHEN** a client requests valid metrics data
- **THEN** the response contains `data` (the metrics), `timestamp` (ISO 8601), and `meta` (query parameters used)

### Requirement: API documentation
The system SHALL provide a /api/v1/docs endpoint that returns basic API documentation.

#### Scenario: API docs
- **WHEN** a client sends GET /api/v1/docs
- **THEN** the system returns JSON describing available endpoints, parameters, and response formats
