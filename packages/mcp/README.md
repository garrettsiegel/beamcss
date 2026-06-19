# @beamcss/mcp

MCP server for [Beam CSS](https://www.npmjs.com/package/beamcss). Exposes Beam syntax guidance, component scaffolding, and token inspection as MCP tools so AI coding agents can generate valid Beam class strings without shell access or docs lookup.

## Run

```sh
npx beamcss-mcp
```

Or install and run directly:

```sh
npm install -g @beamcss/mcp
beamcss-mcp
```

## MCP tools

### `beam_syntax_reference`

Returns Beam CSS syntax guidance for a requested topic.

```json
{ "topic": "all" | "variants" | "utilities" | "values" | "install" }
```

### `beam_scaffold_component`

Returns an HTML or JSX snippet using Beam classes for a common component shape.

```json
{ "kind": "button" | "card" | "dashboard-panel" | "form-row", "jsx": false }
```

### `beam_token_summary`

Summarizes token names from a Beam config, grouped by family (color, spacing, radius, etc.).

```json
{ "config": "<JSON string of a BeamConfig object>" }
```

## Configure with your agent

**Claude Desktop / Claude Code** (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "beamcss": {
      "command": "npx",
      "args": ["beamcss-mcp"]
    }
  }
}
```

**Cursor / Windsurf** (`.cursor/mcp.json` or `.windsurf/mcp.json`):

```json
{
  "mcpServers": {
    "beamcss": {
      "command": "npx",
      "args": ["beamcss-mcp"]
    }
  }
}
```

## Programmatic use

```ts
import { createBeamMcpServer, startBeamMcpServer } from '@beamcss/mcp'

// Start with stdio transport (default)
await startBeamMcpServer()

// Or create a server instance to attach a custom transport
const server = createBeamMcpServer({ name: 'beamcss', version: '0.1.0' })
```

## Links

- [beamcss (core)](https://www.npmjs.com/package/beamcss)
- [GitHub](https://github.com/garrettsiegel/beamcss)
- License: MIT
