## Purpose

Coleta métricas do sistema Linux em tempo real usando o crate sysinfo, fornecendo dados atualizados para o dashboard e API.

## ADDED Requirements

### Requirement: CPU metrics collection
The system SHALL collect CPU usage percentage (per-core and aggregate), CPU frequency, and CPU model name.

#### Scenario: CPU metrics are collected
- **WHEN** the metrics collection task runs
- **THEN** the system provides current CPU usage % (total and per-core), frequency in MHz, and model name

#### Scenario: CPU per-core breakdown
- **WHEN** the system has multiple CPU cores
- **THEN** the system provides individual usage percentage for each core

### Requirement: Memory metrics collection
The system SHALL collect total RAM, used RAM, available RAM, swap total, swap used, and buffer/cache usage.

#### Scenario: Memory metrics are collected
- **WHEN** the metrics collection task runs
- **THEN** the system provides total, used, and available memory in bytes, plus swap and buffer/cache stats

### Requirement: Disk metrics collection
The system SHALL collect disk usage for all mounted filesystems including total space, used space, available space, and mount point.

#### Scenario: Disk metrics are collected
- **WHEN** the metrics collection task runs
- **THEN** the system provides disk usage for each mounted filesystem with total, used, available in bytes and mount point path

#### Scenario: Multiple disks
- **WHEN** the system has multiple disks or partitions
- **THEN** the system reports metrics for each filesystem separately

### Requirement: Network metrics collection
The system SHALL collect network interface names, bytes sent/received, packets sent/received, and current connection count.

#### Scenario: Network metrics are collected
- **WHEN** the metrics collection task runs
- **THEN** the system provides bytes and packets transferred per interface, plus total TCP/UDP connections

### Requirement: Process metrics collection
The system SHALL collect top processes sorted by CPU and memory usage, including PID, name, CPU %, memory %, and status.

#### Scenario: Top processes by CPU
- **WHEN** the metrics collection task runs
- **THEN** the system provides the top 10 processes sorted by CPU usage

#### Scenario: Top processes by memory
- **WHEN** the metrics collection task runs
- **THEN** the system provides the top 10 processes sorted by memory usage

### Requirement: System info collection
The system SHALL collect hostname, OS name/version, kernel version, uptime, load average (1/5/15 min), and system temperature (if available).

#### Scenario: System info is collected
- **WHEN** the metrics collection task runs
- **THEN** the system provides hostname, OS, kernel version, uptime in seconds, and load averages

#### Scenario: Temperature unavailable
- **WHEN** the system does not expose temperature sensors
- **THEN** the system reports temperature as null/None without error

### Requirement: Configurable collection interval
The system SHALL poll metrics at a configurable interval (default: 5 seconds).

#### Scenario: Custom interval
- **WHEN** the config specifies polling_interval_secs = 10
- **THEN** the system collects metrics every 10 seconds

#### Scenario: Default interval
- **WHEN** no polling interval is configured
- **THEN** the system uses 5 seconds as the default
