## Purpose

Fornece autenticação básica com login e sessão para proteger o acesso ao dashboard de métricas.

## ADDED Requirements

### Requirement: Login page
The system SHALL provide a login page with username and password fields.

#### Scenario: Unauthenticated access
- **WHEN** a user navigates to the dashboard without a valid session
- **THEN** the system redirects to the login page

#### Scenario: Login page renders
- **WHEN** the user is on the login page
- **THEN** the system displays a form with username, password, and submit button

### Requirement: Credential validation
The system SHALL validate credentials against configured values (from config file).

#### Scenario: Valid credentials
- **WHEN** the user submits correct username and password
- **THEN** the system creates a session and redirects to the dashboard

#### Scenario: Invalid credentials
- **WHEN** the user submits incorrect credentials
- **THEN** the system displays an error message and remains on the login page

### Requirement: Session management
The system SHALL manage user sessions using signed cookies.

#### Scenario: Session created on login
- **WHEN** the user successfully authenticates
- **THEN** the system sets a signed session cookie with configurable expiry

#### Scenario: Session expired
- **WHEN** the session cookie has expired
- **THEN** the system redirects to the login page

### Requirement: Logout
The system SHALL provide a logout mechanism that clears the session.

#### Scenario: User logs out
- **WHEN** the user clicks the logout button
- **THEN** the system clears the session cookie and redirects to the login page

### Requirement: Protected routes
The system SHALL protect all dashboard and API routes behind authentication.

#### Scenario: Direct URL access
- **WHEN** an unauthenticated user accesses a dashboard URL directly
- **THEN** the system redirects to the login page instead of showing the content
