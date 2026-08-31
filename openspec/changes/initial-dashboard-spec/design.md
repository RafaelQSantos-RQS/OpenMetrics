## Context

Projeto novo com Cargo.toml vazio (sem dependências) e um `main.rs` com Hello World. O objetivo é construir um dashboard de métricas Linux completo usando Rust + HTMX, inspirado no btop/htop. O projeto roda em um servidor único, servindo tanto a UI web quanto uma REST API.

## Goals / Non-Goals

**Goals:**
- Dashboard web responsivo com métricas em tempo real (polling HTMX)
- Histórico de métricas em SQLite para gráficos de tendência
- Auth básica para proteger o dashboard
- REST API para consultas externas
- Binary standalone sem dependências de runtime (exceto SQLite)
- Configuração flexível via TOML

**Non-Goals:**
- Multi-node aggregation (fase futura)
- Alertas/notificações (fase futura)
- WebSocket real-time (HTMX polling é suficiente para 5s intervals)
- Containerização (Docker) neste estágio
- Mobile-first design (desktop/tablet prioridade)

## Decisions

### Decision 1: Axum over Actix-web
**Choice**: Axum
**Rationale**: Axum é Tokio-native, tem API type-safe com extractors, e a comunidade está mais ativa. Para SSE/polling com HTMX, Axum tem suporte nativo melhor. Actix-web é mais maduro mas a API é mais verbosa.
**Alternatives considered**: Actix-web (mais maduro, maior overhead de boilerplate), Poem (menor comunidade)

### Decision 2: Askama over Tera
**Choice**: Askama
**Rationale**: Compile-time templates com type safety. Se o template quebrar, compila. Tera é runtime (flexível mas sem verificação). Para HTMX onde os templates são estáticos com partials, compile-time é ideal.
**Alternatives considered**: Tera (runtime, Jinja2-like), Maud (macro Rust, sintaxe diferente), Silly (menor comunidade)

### Decision 3: rusqlite over sqlx
**Choice**: rusqlite
**Rationale**: Para um projeto single-node com SQLite local, rusqlite é mais simples e não precisa de compile-time migrations. sqlx é melhor para projetos que precisam de async DB ou múltiplos bancos.
**Alternatives considered**: sqlx (async, compile-time queries), sled (embedded, não SQL)

### Decision 4: Chart.js over D3/Plotly
**Choice**: Chart.js via CDN
**Rationale**: Leve (~60KB), simples de integrar com HTMX (atualizar via JavaScript), bom para dashboards. D3 é mais poderoso mas complexo. Plotly é pesado (~3MB).
**Alternatives considered**: D3.js (muito complexo para este caso), Plotly (pesado), uPlot (menos popular)

### Decision 5: Cookie-based sessions over JWT
**Choice**: Signed cookies (tower-sessions)
**Rationale**: Para um dashboard web, cookies são mais simples e seguros. JWT é melhor para APIs stateless mas não é necessário aqui. O dashboard é server-rendered, não SPA.
**Alternatives considered**: JWT tokens (mais complexo, desnecessário para server-rendered)

### Decision 6: sysinfo crate
**Choice**: sysinfo
**Rationale**: Crate padrão para informações do sistema em Rust. Cross-platform, bem mantido, fornece tudo que precisamos (CPU, RAM, disco, rede, processos).
**Alternatives considered**: /proc parsing direto (mais controle mas mais código), psutil (não existe em Rust)

## Architecture

```
openmetrics/
├── src/
│   ├── main.rs              # Entry point, server setup
│   ├── config.rs            # TOML config loading
│   ├── auth/
│   │   ├── mod.rs           # Auth middleware, session management
│   │   └── handlers.rs      # Login/logout handlers
│   ├── metrics/
│   │   ├── mod.rs           # Metrics collection orchestrator
│   │   ├── collector.rs     # sysinfo-based metric collection
│   │   └── types.rs         # Metric data structures
│   ├── storage/
│   │   ├── mod.rs           # SQLite connection management
│   │   ├── models.rs        # Database models
│   │   └── queries.rs       # SQL queries for metrics
│   ├── handlers/
│   │   ├── mod.rs           # Route handlers
│   │   ├── dashboard.rs     # Dashboard page handlers
│   │   └── api.rs           # REST API handlers
│   └── templates/
│       ├── mod.rs           # Askama template structs
│       ├── base.html        # Base layout
│       ├── login.html       # Login page
│       ├── dashboard.html   # Main dashboard
│       └── partials/
│           ├── cpu_card.html
│           ├── memory_card.html
│           ├── disk_card.html
│           ├── network_card.html
│           ├── process_table.html
│           └── charts.html
├── static/
│   ├── css/
│   │   └── style.css        # Custom styles
│   └── js/
│       └── charts.js        # Chart.js initialization
├── config.toml              # Default configuration
└── Cargo.toml
```

## Data Flow

1. **Background Task**: Tokio spawn coleta métricas via sysinfo a cada X segundos
2. **Storage**: Snapshot completo é salvo no SQLite
3. **HTMX Polling**: Dashboard faz GET para endpoints parciais (HTML fragments)
4. **API**: Endpoints REST retornam JSON para consultas externas
5. **Charts**: JavaScript busca dados via API e renderiza com Chart.js

## Key Dependencies

```toml
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
askama = "0.12"
sysinfo = "0.34"
rusqlite = { version = "0.32", features = ["bundled"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["fs", "trace"] }
tower-sessions = "0.13"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Risks / Trade-offs

- **[HTMX polling overhead]** → Mitigation: Use partial HTML fragments, not full pages. 5s interval is reasonable for server metrics.
- **[SQLite write contention]** → Mitigation: Single writer pattern (background task owns writes, dashboard reads). WAL mode for concurrent reads.
- **[sysinfo accuracy]** → Mitigation: sysinfo is well-tested but some metrics (temperature) may not be available on all systems. Handle gracefully.
- **[Chart.js CDN dependency]** → Mitigation: Could vendor Chart.js locally in future. For now, CDN is fine for a single-server dashboard.
- **[Session security]** → Mitigation: Use signed cookies with configurable secret. HTTPS via Caddy in production.

## Migration Plan

1. Implement core (config + metrics collection + storage) first
2. Add auth layer
3. Build dashboard UI with HTMX
4. Add REST API endpoints
5. Deploy with Caddy reverse proxy

## Open Questions

- None at this time. All major decisions have been resolved.
