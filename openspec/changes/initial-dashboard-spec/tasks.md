## 1. Setup & Config

- [x] 1.1 Add all dependencies to Cargo.toml (axum, tokio, askama, sysinfo, rusqlite, tower-http, tower-sessions, serde, toml, chrono, tracing, tracing-subscriber)
- [x] 1.2 Create config.toml with default settings (port, polling_interval, db_path, credentials, retention_days)
- [x] 1.3 Implement config.rs - TOML config loading with defaults and CLI override (--config)
- [x] 1.4 Create module structure: main.rs, config.rs, auth/, metrics/, storage/, handlers/, templates/

## 2. Data Layer - Metrics Collection

- [x] 2.1 Implement metrics/types.rs - Data structures for all metric types (CpuMetrics, MemoryMetrics, DiskMetrics, NetworkMetrics, ProcessMetrics, SystemInfo)
- [x] 2.2 Implement metrics/collector.rs - sysinfo-based collection for CPU (per-core), RAM, swap, disk, network, processes, load, uptime, temperature
- [x] 2.3 Implement metrics/mod.rs - Background Tokio task that collects metrics at configurable interval and broadcasts via channel

## 3. Data Layer - Storage

- [x] 3.1 Implement storage/models.rs - SQLite table schemas and row types
- [x] 3.2 Implement storage/mod.rs - SQLite connection management with WAL mode
- [x] 3.3 Implement storage/queries.rs - Insert snapshots, query by time range, aggregation queries, retention pruning
- [x] 3.4 Add database initialization and migration logic (create tables on first run)

## 4. Auth Layer

- [x] 4.1 Implement auth/mod.rs - Session middleware with tower-sessions and signed cookies
- [x] 4.2 Implement auth/handlers.rs - Login form rendering, credential validation, logout
- [x] 4.3 Add auth guard middleware to protect dashboard and API routes

## 5. Web Framework Setup

- [x] 5.1 Implement main.rs - Axum server setup with routes, static files, session layer
- [x] 5.2 Configure tower-http for serving static/ directory (CSS, JS)
- [x] 5.3 Add request logging with tracing-subscriber

## 6. Dashboard UI - Templates

- [x] 6.1 Create templates/base.html - Base layout with HTMX, Chart.js CDN, dark mode CSS
- [x] 6.2 Create templates/login.html - Login form
- [x] 6.3 Create templates/dashboard.html - Main dashboard with grid layout for metric cards
- [x] 6.4 Create partials: cpu_card.html, memory_card.html, disk_card.html, network_card.html, process_table.html
- [x] 6.5 Create partials: charts.html (CPU line chart, memory area chart, network throughput chart)

## 7. Dashboard UI - Handlers & HTMX

- [x] 7.1 Implement handlers/dashboard.rs - Dashboard page handler, metric card partial handlers (for HTMX polling)
- [x] 7.2 Wire HTMX hx-get endpoints for each metric card partial
- [x] 7.3 Implement static/js/charts.js - Chart.js initialization and update logic
- [x] 7.4 Implement static/css/style.css - Dark mode default, responsive grid, card styling

## 8. REST API

- [x] 8.1 Implement handlers/api.rs - GET /api/v1/metrics/current (latest snapshot JSON)
- [x] 8.2 Implement GET /api/v1/metrics/history?range=1h|6h|24h|7d|30d
- [x] 8.3 Implement GET /api/v1/metrics/{type}?range=X (cpu, memory, disk, network)
- [x] 8.4 Implement GET /api/v1/docs - API documentation endpoint
- [x] 8.5 Add consistent JSON response format: {"data": ..., "timestamp": ..., "meta": {...}}

## 9. Integration & Testing

- [ ] 9.1 Integration test: Config loading with defaults and custom values
- [ ] 9.2 Integration test: Metrics collection produces valid data
- [ ] 9.3 Integration test: SQLite storage round-trip (insert + query)
- [ ] 9.4 Integration test: Auth flow (login → session → dashboard access → logout)
- [ ] 9.5 Integration test: API endpoints return correct JSON structure
- [ ] 9.6 End-to-end test: Start server, verify dashboard loads, metrics update, API responds

## 10. Polish & Deploy

- [ ] 10.1 Add README.md with build, run, and configuration instructions
- [ ] 10.2 Create example config.toml with documented options
- [ ] 10.3 Verify cargo clippy passes with no warnings
- [ ] 10.4 Verify cargo test passes
