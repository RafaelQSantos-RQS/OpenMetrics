## Why

O servidor atual não tem visibilidade sobre seu estado de recursos. Quando algo dá errado (CPU alto, disco cheio, memória vazando), o diagnóstico é manual e demorado. Precisamos de um dashboard leve, inspirado no btop/htop, que mostre métricas em tempo real e preserve histórico para análise de tendências.

## What Changes

- **Dashboard web completo**: Overview com cards de métricas (CPU, RAM, disco, rede, processos, temperatura, load average, uptime) e gráficos históricos via Chart.js
- **Coleta de métricas**: Background task que usa `sysinfo` para ler métricas do sistema a cada X segundos (configurável, default 5s)
- **Armazenamento temporal**: SQLite para persistir histórico de métricas, permitindo gráficos de tendência
- **Autenticação**: Login básico com session/cookie para proteger o dashboard
- **REST API**: Endpoints JSON para consultas externas de métricas (futuro: integração com outros sistemas)
- **Configuração**: Arquivo de configuração para porta, intervalo de polling, credenciais, etc.
- **Deploy**: Binary standalone + Caddy reverse proxy em produção

## Capabilities

### New Capabilities

- `metrics-collection`: Coleta de métricas do sistema Linux usando sysinfo (CPU, RAM, disco, rede, processos, temperatura, load, uptime)
- `metrics-storage`: Armazenamento temporal de métricas em SQLite com retenção configurável
- `dashboard-ui`: Interface web com HTMX + Askama, cards de métricas e gráficos Chart.js
- `auth`: Autenticação básica com login/sessão para acesso ao dashboard
- `config`: Sistema de configuração (arquivo TOML/YAML) para porta, polling interval, credenciais
- `rest-api`: API REST para consultas externas de métricas (endpoints JSON)

### Modified Capabilities

- Nenhuma (projeto novo)

## Impact

- **Novas dependências Rust**: axum, askama, sysinfo, rusqlite/sqlx, tower-http, serde, toml/yaml
- **Novos arquivos**: src/ (módulos de coleta, storage, handlers, templates), templates/ (Askama), config files
- **Infraestrutura**: Requer SQLite no host, Caddy reverse proxy em produção
- **Browser**: Requer JavaScript habilitado para Chart.js e HTMX
