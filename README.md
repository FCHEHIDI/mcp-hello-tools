# mcp-hello-tools

> **Projet 01 — Suite ModelContextProtocol**  
> Niveau : Fondation

Implémentation du premier serveur **MCP (Model Context Protocol)** en Rust avec `rmcp`.  
Objectif : comprendre le protocole dans ses fondamentaux — comment un LLM découvre les tools, les appelle et reçoit les résultats.

## Ce que fait ce serveur

| Tool | Description |
|------|-------------|
| `echo` | Retourne le message inchangé — vérifie que la connexion MCP fonctionne |
| `get_timestamp` | Heure actuelle avec paramètre timezone optionnel |
| `calculate` | Évalue une expression arithmétique simple |

## Stack

- **SDK** : `rmcp` (SDK officiel Rust pour MCP)
- **Async** : `tokio`
- **Transport** : `stdio` (Claude Desktop / VS Code Copilot Chat)
- **Sérialisation** : `serde_json` + `schemars` (JSON Schema auto-généré)
- **Erreurs** : `thiserror` + codes MCP standardisés

## Structure

    mcp-hello-tools/
    ├── Cargo.toml
    ├── claude_desktop_config.json
    └── src/
        ├── main.rs          ← setup transport stdio + run server
        ├── server.rs        ← impl ServerHandler (rmcp trait)
        └── tools/
            ├── mod.rs
            ├── echo.rs
            ├── timestamp.rs
            └── calculate.rs

## Démarrage rapide

    cargo build --release
    # Puis dans claude_desktop_config.json :
    # "command": "./target/release/mcp-hello-tools"

## Connexion à Claude Desktop

Copie `claude_desktop_config.json` dans :
- **Windows** : `%APPDATA%\Claude\`
- **macOS** : `~/Library/Application Support/Claude/`

---
*Suite ModelContextProtocol — [FCHEHIDI](https://github.com/FCHEHIDI)*
