## Purpose

Fornece a interface web do dashboard usando HTMX + Askama para exibir métricas em tempo real com cards e gráficos Chart.js.

## ADDED Requirements

### Requirement: Dashboard overview page
The system SHALL provide a main dashboard page with an overview of all key metrics displayed as cards.

#### Scenario: Dashboard loads
- **WHEN** a logged-in user navigates to the dashboard
- **THEN** the system displays cards for CPU, RAM, disk, network, load average, uptime, and top processes

#### Scenario: Metrics are current
- **WHEN** the dashboard page is displayed
- **THEN** all metric cards show values from the most recent collection cycle

### Requirement: Auto-refresh via HTMX
The system SHALL use HTMX polling to automatically refresh metric cards without full page reloads.

#### Scenario: Periodic refresh
- **WHEN** the dashboard is open
- **THEN** metric cards update automatically at the configured polling interval via HTMX hx-trigger

#### Scenario: Partial page update
- **WHEN** a metric card is refreshed
- **THEN** only that card's HTML is replaced, not the entire page

### Requirement: CPU visualization with charts
The system SHALL display CPU usage as a time-series line chart using Chart.js.

#### Scenario: CPU chart renders
- **WHEN** the dashboard loads
- **THEN** a line chart shows CPU usage % over time (last hour by default)

#### Scenario: Chart updates
- **WHEN** new metric data arrives
- **THEN** the CPU chart appends the new data point and scrolls the time window

### Requirement: Memory visualization with charts
The system SHALL display memory usage as a stacked area chart showing used/buffer/cache/available over time.

#### Scenario: Memory chart renders
- **WHEN** the dashboard loads
- **THEN** a stacked area chart shows memory breakdown over time

### Requirement: Disk usage display
The system SHALL display disk usage as progress bars or gauge charts showing used/total per filesystem.

#### Scenario: Disk metrics display
- **WHEN** the dashboard loads
- **THEN** each filesystem shows a progress bar with used/total and percentage

### Requirement: Network throughput display
The system SHALL display network throughput as a chart showing bytes in/out over time per interface.

#### Scenario: Network chart renders
- **WHEN** the dashboard loads
- **THEN** a chart shows network throughput (KB/s or MB/s) over time

### Requirement: Process list
The system SHALL display top processes as a sortable table with PID, name, CPU %, memory %, and status.

#### Scenario: Process table renders
- **WHEN** the dashboard loads
- **THEN** a table shows top 10 processes by CPU usage with columns for PID, name, CPU %, memory %, and status

### Requirement: Responsive layout
The system SHALL provide a responsive grid layout that works on desktop and tablet screens.

#### Scenario: Desktop layout
- **WHEN** viewed on a 1920px+ screen
- **THEN** the dashboard displays a multi-column grid of metric cards

#### Scenario: Tablet layout
- **WHEN** viewed on a 768px-1024px screen
- **THEN** the dashboard reflows to fewer columns

### Requirement: Dark mode support
The system SHALL support dark and light themes, defaulting to dark mode.

#### Scenario: Dark mode default
- **WHEN** no theme preference is set
- **THEN** the dashboard renders in dark mode

#### Scenario: Theme toggle
- **WHEN** the user clicks the theme toggle
- **THEN** the dashboard switches between dark and light mode and persists the preference
