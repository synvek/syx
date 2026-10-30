# SYX VS Code Extension

Language support for `.sy` files: syntax highlighting, LSP (completion, hover, go-to-definition), formatting, and debugging.

## Prerequisites

From the repository root:

```bash
cargo build -p syx-cli
cd editors/vscode-syx && npm install
```

The extension launches the language server at `<workspace>/target/debug/syx lsp` (or `syx.exe` on Windows). Override with `syx.languageServerPath` — set either the binary, or the SDK `bin` directory (e.g. `C:/tools/syx/bin`).

## Development

1. Open this folder or the repo root in VS Code.
2. Run **Extension Development Host** (F5) with the SYX extension loaded.
3. Open a `.sy` file and type `.` after a variable (e.g. `xs.` on an `Array`) to see member completion.
4. Type `import` or `import ` to see importable packages (`IO`, `Net`, `Crypto`, …).

Enable LSP tracing via `syx.trace.server`: `messages` or `verbose` (output channel **SYX LSP Trace**).

## Member completion

After building the CLI, dot-completion suggests methods for:

- `String`, `Array`, `Set`, `Dictionary` receivers
- Literal arrays (`[1, 2].`)
- Function call results (`getItems().`)
- Imported modules (`Concurrency.join` via `Concurrency.`)

## Troubleshooting

### Dot (`.`) does not show completion

1. **Build the CLI** (required once per clone):
   ```bash
   cargo build -p syx-cli
   ```
2. **Install extension dependencies** (if developing from source):
   ```bash
   cd editors/vscode-syx && npm install
   ```
3. **Package without stripping dependencies** — use `npm run package`, **not** `vsce package --no-dependencies` (that omits `vscode-languageclient` and LSP will not start).
4. Open the **synswx workspace root** (so the extension finds `target/debug/syx`).
5. Run command **SYX: Check Language Server** — status bar shows `SYX LSP` when running.
6. Set `syx.trace.server` to `messages` and check **Output → SYX LSP Trace** if still broken.
7. Reload window after building (`Developer: Reload Window`).

If completion does not appear, verify `target/debug/syx` exists and reload the window.

## Actor routing graph webview spike (Phase 27)

Phase 26 delivered LSP hover hints for Actor routing; Phase 27 adds a **VS Code webview panel** (DZ3) visualizing process-local and cross-process Actor edges.

### Goals

- Read-only graph: nodes = Actor mailboxes / dist transport endpoints; edges = `ActorSend` / `dist_actor_send_process` / UDS/TLS routes.
- Command palette: **SYX: Show Actor Routing Graph** (`syx.showActorRoutingGraph`).
- Data source MVP: LSP custom request `syx/actorRoutingGraph` (v19) returning JSON `{ nodes, edges }`; fallback to workspace symbol scan.

### Spike architecture

```
extension.ts
  └─ ActorRoutingGraphPanel (WebviewPanel)
       └─ media/graph.html + graph.js (force-directed layout)
            ← LSP request syx/actorRoutingGraph
            ← optional dist_transport metadata (UDS path / TLS endpoint)
```

### Development notes

1. Webview uses `localResourceRoots` under `media/`; CSP allows inline script for spike only.
2. Cross-process edges styled dashed when `dist_actor_uds_enabled()`; TLS edges labeled with endpoint from `dist_actor_tls_endpoint()`.
3. Phase 27 spike is **read-only**; Phase 28 adds **LSP live** click-to-navigate via `syx/actorRoutes`.
4. Package as **2.1.0** with Test Explorer `p28_*` filter alongside routing graph live mode.

### Phase 28 — LSP live routing graph (EE3)

Phase 28 replaces mock JSON with **LSP semantic graph feed**:

- Custom request **`syx/actorRoutes`** (LSP v20) returns live `{ nodes, edges }` from workspace analysis
- **Click-to-navigate**: edge click opens source file at Actor send site
- Fallback: Phase 27 mock graph when LSP unavailable
- Package as **2.1.0** with Test Explorer `p28_*` filter

### Related

- Runtime: `crates/syx-async/src/dist_transport.rs`
- LSP v19 hover/completion for dist Actor/TLS
- ADR/docs: Phase 27 DZ theme in `docs/README.md` changelog

## Actor Routing Graph (Phase 27 → Phase 28)

Command **SYX: Show Actor Routing Graph** (`syx.showActorRoutingGraph`) opens a webview panel beside the editor with distributed actor routes:

- **Phase 27**: mock routing graph (main/worker processes, Counter/Worker actors, UDS/TLS edges)
- **Phase 28**: **LSP live mode** via `syx/actorRoutes` — click-to-navigate to send sites

Read-only preview in Phase 27; Phase 28 populates routes from LSP semantic analysis with dist-actor telemetry fallback.

## Phase 28 routing LSP live mode

Phase 28 replaces the mock-only webview with **LSP-fed routing graphs** and click-to-navigate on edge rows.

### Data flow

```
extension.js
  └─ syx.showActorRoutingGraph / syx.fetchActorRoutes
       └─ workspace/executeCommand { command: "syx.actorRoutes", arguments: [uri] }
            ← syx-lsp actor_routes_for_source(source, path) → { nodes, edges, live: true }
       └─ fallback: MOCK_ACTOR_ROUTING_GRAPH when LSP unavailable
```

### Commands

| Command | ID | Purpose |
|---------|-----|---------|
| Show Actor Routing Graph | `syx.showActorRoutingGraph` | Opens webview with LSP live graph or mock fallback |
| Fetch Actor Routes | `syx.fetchActorRoutes` | Returns `{ nodes, edges }` from LSP; shows node/edge counts (calls LSP execute command `syx.actorRoutes`) |

### LSP API

`actor_routes_for_source(source, path)` in `crates/syx-lsp/src/lib.rs` returns:

- **nodes**: process + `actor` / `global-actor` declarations with `line`/`character` for navigation
- **edges**: in-process, cross-actor, and cross-process (`UDS prod` / `TLS real`) routes with `targetLine`/`targetCharacter`
- **live**: `true` when served from LSP (vs mock fallback in the extension)

Registered as LSP execute command **`syx.actorRoutes`** (`executeCommandProvider` in initialize).

### Click-to-navigate

Edge rows in the webview post `navigateEdge` messages; the extension opens the target file and reveals the actor declaration line.

### Test Explorer

Default fixture filter is **`p28_*`** (Phase 28 integration matrix). Override via `syx.testExplorer.include`.

### Version

Extension **2.2.0** — LSP v21 (≥285 tests), DAP v21 (≥65 tests), SYX 13.5 Backend LTS GA IDE theme EJ.

## Phase 29 — bidirectional routing sync (2.2.0)

Phase 29 adds **bidirectional sync** between DAP breakpoints and the actor routing graph webview.

### Bidirectional sync

- **Edge → source**: click an edge row in the routing graph webview; the extension navigates to the edge's `targetLine` / `targetCharacter` on the target actor declaration.
- **Breakpoint → route**: place the cursor on a line with a route edge (e.g. a `dist_send` call) and run **SYX: Sync Breakpoint to Route Target** (`syx.syncBreakpointToRoute`); the extension finds the matching edge by breakpoint line and jumps to the route target.

### LSP graph metadata

`actor_routes_for_source` returns `phase: 29`, `bidirectional: true`, and edges include both send-site (`line`) and target (`targetLine`) coordinates. `actor_routes_edge_for_breakpoint(source, path, line)` maps a breakpoint line to the nearest edge for DAP/IDE sync.

### Data flow

```
extension.js
  └─ syx.showActorRoutingGraph / syx.syncBreakpointToRoute
       └─ workspace/executeCommand { command: "syx.actorRoutes", arguments: [uri] }
            ← syx-lsp actor_routes_for_source → { nodes, edges, phase: 29, bidirectional: true }
       └─ edge click or syncBreakpointToRoute → navigateToEdgeTarget(targetLine)
       └─ fallback: MOCK_ACTOR_ROUTING_GRAPH (phase 29, bidirectional)
```

### Test Explorer

Default fixture filter is **`p29_*`** (Phase 29 integration matrix). Override via `syx.testExplorer.include`.

### Version

Extension **2.2.0** — LSP v21 (≥285 tests), DAP v21 (≥65 tests), SYX 13.5 Backend LTS GA IDE theme EJ.

## Phase 30 — multiprocess routing live feed (2.3.0)

Phase 30 adds **multiprocess live feed** for the actor routing graph and **persistent breakpoint↔edge session mapping** for DAP sync.

### Multiprocess live feed

- **Show graph**: **SYX: Show Actor Routing Graph** fetches `syx.actorRoutesMultiprocess` from LSP and opens a multiprocess live feed panel (phase 30, `multiProcess: true`, `sessionId`).
- **Sync all routes**: **SYX: Sync Multiprocess Routes** (`syx.syncMultiprocessRoutes`) persists every edge's breakpoint line → target mapping under the route `sessionId`.

### Persistent breakpoint↔edge mapping

- Each cross-process edge carries a stable `sessionId` from `actor_routes_multiprocess_for_source`.
- **Breakpoint → route**: **SYX: Sync Breakpoint to Route Target** uses multiprocess routes first; mappings are stored in `breakpointEdgeSessionMap` keyed by `sessionId`.
- Subsequent syncs on the same session can resolve targets from the persistent map when the live edge list changes.

### LSP graph metadata

`actor_routes_multiprocess_for_source` returns `phase: 30`, `multiProcess: true`, `sessionId`, `persistentMapping: true`, and cross-process edges include `sessionId` + `multiProcess`. Base `actor_routes_for_source` returns `phase: 30`.

### Data flow

```
extension.js
  └─ syx.showActorRoutingGraph / syx.syncMultiprocessRoutes / syx.syncBreakpointToRoute
       └─ workspace/executeCommand { command: "syx.actorRoutesMultiprocess", arguments: [uri] }
            ← syx-lsp actor_routes_multiprocess_for_source → { nodes, edges, phase: 30, sessionId }
       └─ breakpointEdgeSessionMap[sessionId][breakpointLine] → targetLine navigation
       └─ fallback: MOCK_ACTOR_ROUTING_GRAPH (phase 30, multiprocess)
```

### Test Explorer

Default fixture filter is **`p30_*`** (Phase 30 integration matrix). Override via `syx.testExplorer.include`.

### Version

Extension **2.3.0** — LSP v22 (≥300 tests), DAP v22 (≥72 tests), SYX 14.0 Platform LTS GA IDE theme EO.

## Phase 31 — federated routing live feed (2.4.0)

Phase 31 adds **cross-workspace federated feed** for the actor routing graph and **DAP session persistence v2** for breakpoint↔edge mapping.

### Federated cross-workspace feed

- **Show graph**: **SYX: Show Federated Routing Graph** (`syx.showFederatedRoutingGraph`) fetches `syx.actorRoutesFederated` from LSP with all workspace folder roots and opens a federated live feed panel (phase 31, `federated: true`, `crossWorkspace: true`, `sessionPersistence: "v2"`).
- **Show graph (alias)**: **SYX: Show Actor Routing Graph** also uses federated feed in 2.4.0.
- **Sync all routes**: **SYX: Sync Federated Routes** (`syx.syncFederatedRoutes`) persists every edge's breakpoint line → target mapping under the federated `sessionId` with v2 metadata.

### DAP session persistence v2

- Federated sessions use `syx-fed-*` session IDs stable across workspace roots.
- `dapSessionPersistenceV2` stores `{ version: "v2", workspaceRoots, updatedAt }` per session.
- **Breakpoint → route**: **SYX: Sync Breakpoint to Route Target** uses federated routes first; mappings include `sessionPersistence: "v2"` and `crossWorkspace` flags.

### LSP graph metadata

`actor_routes_federated_for_source` returns `phase: 31`, `federated: true`, `crossWorkspace: true`, `sessionPersistence: "v2"`, `workspaceRoots`, and cross-workspace edges with `kind: "cross-workspace"`. Base `actor_routes_for_source` and `actor_routes_multiprocess_for_source` return `phase: 31`.

### LSP v23 / DAP v23 highlights

- **LSP v23** (≥315 tests): GC intrinsics AOT hover, generational GC, GlobalActor v11 failover, LLVM Phase 6, borrow formal verify, ADR-038 perf, federated routing cross-workspace.
- **DAP v23** (≥77 tests): generational GC breakpoints, rustls E2E attach, cross-workspace routing, Phase 6 AOT attach.

### Data flow

```
extension.js
  └─ syx.showFederatedRoutingGraph / syx.syncFederatedRoutes / syx.syncBreakpointToRoute
       └─ workspace/executeCommand { command: "syx.actorRoutesFederated", arguments: [uri, workspaceRoots] }
            ← syx-lsp actor_routes_federated_for_source → { nodes, edges, phase: 31, sessionId, sessionPersistence: "v2" }
       └─ breakpointEdgeSessionMap[sessionId][breakpointLine] + dapSessionPersistenceV2[sessionId] → targetLine navigation
       └─ fallback: MOCK_ACTOR_ROUTING_GRAPH (phase 31, federated)
```

### Test Explorer

Default fixture filter is **`p31_*`** (Phase 31 integration matrix). Override via `syx.testExplorer.include`.

### Version

Extension **2.4.0** — LSP v23 (≥315 tests), DAP v23 (≥77 tests), SYX 14.5 Generational GC LTS + LLVM Performance GA IDE theme ET.

## Phase 32 — federated routing live feed v3 (2.5.0)

Phase 32 adds **multi-repo federated feed v3** for the actor routing graph and **DAP session persistence v3** for breakpoint↔edge mapping across workspace roots.

### Federated multi-repo feed v3

- **Show graph**: **SYX: Show Federated Routing Graph** (`syx.showFederatedRoutingGraph`) fetches `syx.actorRoutesFederated` from LSP with all workspace folder roots and opens a federated live feed panel (phase 32, `federated: true`, `crossWorkspace: true`, `multiRepo: true`, `sessionPersistence: "v3"`).
- **Show graph (alias)**: **SYX: Show Actor Routing Graph** also uses federated feed v3 in 2.5.0.
- **Sync all routes**: **SYX: Sync Federated Routes** (`syx.syncFederatedRoutes`) persists every edge's breakpoint line → target mapping under the federated `sessionId` with v3 metadata including `multiRepo`.

### DAP session persistence v3

- Federated sessions use `syx-fed-*` session IDs stable across workspace roots and repos.
- `dapSessionPersistenceV3` stores `{ version: "v3", workspaceRoots, multiRepo, updatedAt }` per session.
- **Breakpoint → route**: **SYX: Sync Breakpoint to Route Target** uses federated routes first; mappings include `sessionPersistence: "v3"`, `crossWorkspace`, and `multiRepo` flags.

### LSP graph metadata

`actor_routes_federated_for_source` returns `phase: 32`, `federated: true`, `crossWorkspace: true`, `multiRepo: true`, `sessionPersistence: "v3"`, `workspaceRoots`, cross-workspace edges with `kind: "cross-workspace"`, and cross-repo edges with `kind: "cross-repo"` when ≥3 roots. Base `actor_routes_for_source` and `actor_routes_multiprocess_for_source` return `phase: 32`.

### LSP v24 / DAP v24 highlights

- **LSP v24** (≥330 tests): remembered set GC hover, ADR-040 release blocker GA, GlobalActor v12, LLVM Phase 7, borrow machine proof harness, federated routing v3 multi-repo.
- **DAP v24** (≥82 tests): remembered set breakpoints, rustls prod crate attach, multi-repo routing, Phase 7 AOT attach.

### Data flow

```
extension.js
  └─ syx.showFederatedRoutingGraph / syx.syncFederatedRoutes / syx.syncBreakpointToRoute
       └─ workspace/executeCommand { command: "syx.actorRoutesFederated", arguments: [uri, workspaceRoots] }
            ← syx-lsp actor_routes_federated_for_source → { nodes, edges, phase: 32, sessionId, sessionPersistence: "v3", multiRepo: true }
       └─ breakpointEdgeSessionMap[sessionId][breakpointLine] + dapSessionPersistenceV3[sessionId] → targetLine navigation
       └─ fallback: MOCK_ACTOR_ROUTING_GRAPH (phase 32, federated, multi-repo)
```

### Test Explorer

Default fixture filter is **`p32_*`** (Phase 32 integration matrix). Override via `syx.testExplorer.include`.

### Version

Extension **2.5.0** — LSP v24 (≥330 tests), DAP v24 (≥82 tests), SYX 15.0 LLVM Release Blocker GA + Full Generational GC LTS IDE theme EY.
