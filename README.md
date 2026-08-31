# OpenMetrics

Dashboard de métricas do sistema em tempo real, construído com Rust e Axum.

Coleta CPU, memória, disco e rede — exibe em um dashboard web com gráficos e tabelas de processos.

## Stack

- **Axum** — web framework async
- **SQLite** — armazenamento histórico de métricas
- **Askama** — templates HTML
- **HTMX** — atualizações parciais sem reload
- **Chart.js** — gráficos de tendência

## Rodar

```bash
cargo run
```

O servidor inicia em `http://0.0.0.0:3000`.

Login padrão: `admin` / `admin`

## Configuração

Edite `config.toml`:

```toml
[server]
host = "0.0.0.0"
port = 3000

[metrics]
polling_interval_secs = 5

[database]
db_path = "data/metrics.db"
retention_days = 30

[auth]
username = "admin"
password_hash = "$2b$12$..."  # bcrypt hash
session_secret = "mude-para-um-segredo-aleatorio"
session_expiry_hours = 24
```

## API

| Endpoint | Método | Descrição |
|---|---|---|
| `/api/v1/metrics/current` | GET | Métricas atuais |
| `/api/v1/metrics/history` | GET | Histórico |
| `/api/v1/metrics/{type}` | GET | Métrica específica |
| `/api/v1/docs` | GET | Documentação |
