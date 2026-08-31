## Purpose

Gerencia configuração do aplicativo via arquivo TOML, permitindo customizar porta, intervalo de polling, credenciais e outros parâmetros.

## ADDED Requirements

### Requirement: Configuration file
The system SHALL read configuration from a TOML file (default: `config.toml` in the working directory).

#### Scenario: Config file exists
- **WHEN** the application starts and `config.toml` exists
- **THEN** the system loads all settings from the file

#### Scenario: Config file missing
- **WHEN** the application starts and no config file exists
- **THEN** the system uses built-in defaults for all settings

### Requirement: Server configuration
The system SHALL support configuring the listen address and port (default: 0.0.0.0:3000).

#### Scenario: Custom port
- **WHEN** config specifies `port = 8080`
- **THEN** the server listens on port 8080

#### Scenario: Default port
- **WHEN** no port is configured
- **THEN** the server listens on port 3000

### Requirement: Polling interval configuration
The system SHALL support configuring the metrics collection interval in seconds (default: 5).

#### Scenario: Custom polling interval
- **WHEN** config specifies `polling_interval_secs = 10`
- **THEN** the system collects metrics every 10 seconds

### Requirement: Database configuration
The system SHALL support configuring the SQLite database path (default: `data/metrics.db`).

#### Scenario: Custom database path
- **WHEN** config specifies `db_path = "/var/lib/openmetrics/metrics.db"`
- **THEN** the system creates/opens the database at that path

### Requirement: Authentication configuration
The system SHALL support configuring username, password hash, and session secret in the config file.

#### Scenario: Credentials configured
- **WHEN** config contains `username` and `password_hash`
- **THEN** the system uses those credentials for authentication

### Requirement: Data retention configuration
The system SHALL support configuring how long to retain metric data in days (default: 30).

#### Scenario: Custom retention
- **WHEN** config specifies `retention_days = 7`
- **THEN** the system prunes data older than 7 days

### Requirement: CLI argument override
The system SHALL support overriding the config file path via CLI argument (`--config <path>`).

#### Scenario: Custom config path
- **WHEN** the user runs `openmetrics --config /etc/openmetrics/config.toml`
- **THEN** the system loads configuration from the specified path
