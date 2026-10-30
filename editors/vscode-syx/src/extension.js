const fs = require("fs");
const path = require("path");
const vscode = require("vscode");

/** @type {import("vscode-languageclient/node").LanguageClient | undefined} */
let client;

/** @type {vscode.StatusBarItem | undefined} */
let statusBarItem;

/**
 * On Windows, `fs.existsSync("syx")` is false even when `syx.exe` exists.
 * @param {string} filePath
 * @returns {string[]}
 */
function executableCandidates(filePath) {
  if (process.platform !== "win32") {
    return [filePath];
  }
  const lower = filePath.toLowerCase();
  if (
    lower.endsWith(".exe") ||
    lower.endsWith(".cmd") ||
    lower.endsWith(".bat") ||
    lower.endsWith(".com")
  ) {
    return [filePath];
  }
  return [`${filePath}.exe`, filePath];
}

/**
 * @param {string} candidate
 * @returns {string | undefined}
 */
function existingExecutable(candidate) {
  for (const p of executableCandidates(candidate)) {
    try {
      if (fs.existsSync(p) && fs.statSync(p).isFile()) {
        return p;
      }
    } catch {
      // ignore stat races
    }
  }
  return undefined;
}

/**
 * Resolve a configured path that may be a binary, a directory containing syx,
 * or a bare name for PATH lookup.
 * @param {string} configured
 * @returns {string | undefined} absolute/resolved file path when found on disk
 */
function resolveConfiguredBinary(configured) {
  try {
    if (fs.existsSync(configured) && fs.statSync(configured).isDirectory()) {
      return existingExecutable(path.join(configured, "syx"));
    }
  } catch {
    // ignore
  }
  return existingExecutable(configured);
}

/**
 * Preferred path to show in errors when the binary is missing.
 * @param {string} configured
 * @returns {string}
 */
function preferredMissingPath(configured) {
  try {
    if (fs.existsSync(configured) && fs.statSync(configured).isDirectory()) {
      return process.platform === "win32"
        ? path.join(configured, "syx.exe")
        : path.join(configured, "syx");
    }
  } catch {
    // ignore
  }
  if (
    process.platform === "win32" &&
    path.isAbsolute(configured) &&
    !configured.toLowerCase().endsWith(".exe")
  ) {
    return `${configured}.exe`;
  }
  return configured;
}

/**
 * @param {vscode.WorkspaceFolder | undefined} folder
 */
function resolveSyxPath(folder) {
  const cfg = vscode.workspace.getConfiguration("syx");
  const configured = String(cfg.get("languageServerPath", "syx") || "syx").trim();

  if (path.isAbsolute(configured)) {
    return resolveConfiguredBinary(configured) || preferredMissingPath(configured);
  }

  const root =
    folder?.uri.fsPath || vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  if (root) {
    if (configured === "syx") {
      for (const rel of ["target/debug/syx", "target/release/syx"]) {
        const found = existingExecutable(path.join(root, rel));
        if (found) {
          return found;
        }
      }
    } else {
      const asRelative = resolveConfiguredBinary(path.join(root, configured));
      if (asRelative) {
        return asRelative;
      }
      const legacy = existingExecutable(
        path.join(root, "target", "debug", configured)
      );
      if (legacy) {
        return legacy;
      }
    }
  }
  // Bare name: let the OS PATH / PATHEXT resolve at spawn time.
  return configured;
}

/**
 * @param {string} syxPath
 * @returns {string | undefined}
 */
function missingBinaryMessage(syxPath) {
  // Bare `syx` may still be found via PATH when spawning; only hard-fail absolute paths.
  if (!path.isAbsolute(syxPath)) {
    return undefined;
  }
  if (existingExecutable(syxPath)) {
    return undefined;
  }
  const root = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  if (root) {
    return (
      `SYX language server not found at ${syxPath}. ` +
      "Set `syx.languageServerPath` to your SDK `bin` folder or `syx.exe`, " +
      "or run `cargo build -p syx-cli` in the workspace root, then reload."
    );
  }
  return (
    `SYX language server not found at ${syxPath}. ` +
    "Set `syx.languageServerPath` to your SDK `bin` folder (e.g. `C:/tools/syx/bin`) " +
    "or the full path to `syx.exe`."
  );
}

/**
 * @param {"starting" | "running" | "failed"} state
 * @param {string} [detail]
 */
function updateStatusBar(state, detail) {
  if (!statusBarItem) {
    return;
  }
  if (state === "running") {
    statusBarItem.text = "$(check) SYX LSP";
    statusBarItem.tooltip = detail || "SYX language server is running";
    statusBarItem.backgroundColor = undefined;
  } else if (state === "starting") {
    statusBarItem.text = "$(sync~spin) SYX LSP";
    statusBarItem.tooltip = "Starting SYX language server…";
    statusBarItem.backgroundColor = undefined;
  } else {
    statusBarItem.text = "$(warning) SYX LSP";
    statusBarItem.tooltip = detail || "SYX language server failed to start";
    statusBarItem.backgroundColor = new vscode.ThemeColor(
      "statusBarItem.warningBackground"
    );
  }
  statusBarItem.show();
}

/**
 * @param {string[]} args
 * @param {string | undefined} filePath
 */
function substituteFileInArgs(args, filePath) {
  if (!filePath) {
    return args;
  }
  return args.map((arg) => arg.replace(/\$\{file\}/g, filePath));
}

/**
 * Register DAP support first so debugging works even if LSP fails to start.
 * @param {vscode.ExtensionContext} context
 */
function registerDebugSupport(context) {
  const debugProvider = {
    /**
     * @param {vscode.WorkspaceFolder | undefined} folder
     * @param {vscode.DebugConfiguration | undefined} config
     */
    resolveDebugConfiguration(folder, config) {
      if (!config?.type && !config?.request && !config?.name) {
        config = {
          type: "syx",
          request: "launch",
          name: "SYX: Debug File",
          program: resolveSyxPath(folder),
          args: ["debug", "--dap", "${file}"],
          cwd: folder?.uri.fsPath,
        };
      }
      if (config?.type === "syx") {
        const activeFile = vscode.window.activeTextEditor?.document;
        if (!config.program) {
          config.program = resolveSyxPath(folder);
        }
        if (!config.cwd) {
          config.cwd = folder?.uri.fsPath;
        }
        if (activeFile?.languageId === "syx") {
          config.args = substituteFileInArgs(
            config.args || ["debug", "--dap", "${file}"],
            activeFile.uri.fsPath
          );
        }
      }
      return config;
    },
  };
  context.subscriptions.push(
    vscode.debug.registerDebugConfigurationProvider("syx", debugProvider)
  );

  const factory = {
    /**
     * @param {vscode.DebugSession} session
     */
    createDebugAdapterDescriptor(session) {
      const folder = session.configuration.cwd
        ? vscode.workspace.getWorkspaceFolder(
            vscode.Uri.file(session.configuration.cwd)
          )
        : undefined;
      const program =
        session.configuration.program || resolveSyxPath(folder);
      const activeFile = vscode.window.activeTextEditor?.document;
      const args = substituteFileInArgs(
        session.configuration.args || ["debug", "--dap", "${file}"],
        activeFile?.languageId === "syx" ? activeFile.uri.fsPath : undefined
      );
      const cwd =
        session.configuration.cwd ||
        folder?.uri.fsPath ||
        vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
      return new vscode.DebugAdapterExecutable(program, args, { cwd });
    },
  };
  context.subscriptions.push(
    vscode.debug.registerDebugAdapterDescriptorFactory("syx", factory)
  );
}

/** @type {vscode.ExtensionContext | undefined} */
let extensionContext;

/**
 * @param {import("vscode-languageclient/node").LanguageClient | undefined} oldClient
 */
async function disposeLanguageClient(oldClient) {
  if (!oldClient) {
    return;
  }
  try {
    if (oldClient.isRunning()) {
      await oldClient.stop();
    }
  } catch {
    // startFailed / stopped clients cannot be stopped; discard the instance.
  }
}

/**
 * @param {vscode.ExtensionContext} context
 */
async function startLanguageClient(context) {
  extensionContext = context;
  await disposeLanguageClient(client);
  client = undefined;

  const {
    LanguageClient,
    TransportKind,
  } = require("vscode-languageclient/node");

  const cfg = vscode.workspace.getConfiguration("syx");
  const syxPath = resolveSyxPath();
  const trace = cfg.get("trace.server", "off");
  const missing = missingBinaryMessage(syxPath);
  if (missing) {
    updateStatusBar("failed", missing);
    await vscode.window.showErrorMessage(missing, "Open Settings").then((choice) => {
      if (choice === "Open Settings") {
        void vscode.commands.executeCommand(
          "workbench.action.openSettings",
          "syx.languageServerPath"
        );
      }
    });
    return;
  }

  updateStatusBar("starting");

  const workspaceRoot =
    vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  const outputChannel = vscode.window.createOutputChannel("SYX Language Server");

  const serverOptions = {
    command: syxPath,
    args: ["lsp"],
    transport: TransportKind.stdio,
    options: workspaceRoot ? { cwd: workspaceRoot } : undefined,
  };

  const clientOptions = {
    documentSelector: [{ scheme: "file", language: "syx" }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher("**/*.sy"),
    },
    outputChannel,
    traceOutputChannel: vscode.window.createOutputChannel("SYX LSP Trace"),
  };

  client = new LanguageClient(
    "syxLanguageServer",
    "SYX Language Server",
    serverOptions,
    clientOptions
  );
  if (trace !== "off") {
    client.setTrace(trace === "verbose" ? 2 : 1);
  }
  client.onDidChangeState((event) => {
    if (event.newState === 2) {
      updateStatusBar("running", `SYX LSP: ${syxPath}`);
    } else if (event.newState === 1) {
      updateStatusBar("failed", event.oldState === 2 ? "SYX LSP stopped" : undefined);
    }
  });

  try {
    await client.start();
    updateStatusBar("running", `SYX LSP: ${syxPath}`);
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    const detail =
      `SYX LSP failed to start (${message}). Expected binary at ${syxPath}. ` +
      "Check Output → SYX Language Server for details.";
    updateStatusBar("failed", detail);
    console.error("SYX LSP failed to start:", err);
    await vscode.window.showErrorMessage(detail);
    await disposeLanguageClient(client);
    client = undefined;
  }
}

/** Mock dist-actor routing graph fallback when LSP is unavailable. */
const MOCK_ACTOR_ROUTING_GRAPH = {
  nodes: [
    { id: "proc-main", label: "main process", kind: "process" },
    { id: "actor-counter", label: "Counter", kind: "actor", process: "proc-main", line: 0, character: 6 },
    { id: "proc-worker", label: "worker process", kind: "process" },
    { id: "actor-worker", label: "Worker", kind: "actor", process: "proc-worker", line: 1, character: 6 },
  ],
  edges: [
    {
      from: "actor-counter",
      to: "actor-counter",
      kind: "in-process",
      label: "take()",
      path: "mock.sy",
      line: 2,
      character: 40,
      targetPath: "mock.sy",
      targetLine: 0,
      targetCharacter: 6,
      sessionId: "syx-mp-mock-fallback",
    },
    {
      from: "actor-counter",
      to: "actor-worker",
      kind: "cross-process",
      label: "dist_send (UDS/TLS)",
      path: "mock.sy",
      line: 2,
      character: 50,
      targetPath: "mock.sy",
      targetLine: 1,
      targetCharacter: 6,
      multiProcess: true,
      sessionId: "syx-mp-mock-fallback",
    },
  ],
  live: false,
  phase: 34,
  bidirectional: true,
  multiProcess: true,
  federated: true,
  crossWorkspace: true,
  multiRepo: true,
  multiOrg: true,
  multiRegion: true,
  sessionPersistence: "v5",
  sessionId: "syx-fed-mock-fallback",
  persistentMapping: true,
  workspaceRoots: ["region/us/mock-ws1", "region/eu/mock-ws2", "region/apac/mock-ws3"],
};

/** Persistent breakpoint line → edge session mapping keyed by DAP route sessionId (v5). */
const breakpointEdgeSessionMap = new Map();

/** @type {Map<string, { version: string; workspaceRoots: string[]; multiRepo: boolean; multiOrg: boolean; multiRegion: boolean; updatedAt: number }>} */
const dapSessionPersistenceV5 = new Map();

/**
 * @param {string} sessionId
 * @param {number} breakpointLine
 * @param {{ targetPath?: string; targetLine?: number; targetCharacter?: number; path?: string; line?: number; character?: number }} edge
 */
function persistBreakpointEdgeMapping(sessionId, breakpointLine, edge) {
  if (!sessionId) {
    return;
  }
  let map = breakpointEdgeSessionMap.get(sessionId);
  if (!map) {
    map = new Map();
    breakpointEdgeSessionMap.set(sessionId, map);
  }
  map.set(breakpointLine, {
    targetPath: edge.targetPath || edge.path || "",
    targetLine: edge.targetLine ?? edge.line ?? 0,
    targetCharacter: edge.targetCharacter ?? edge.character ?? 0,
    edgeLine: edge.line ?? breakpointLine,
    sessionPersistence: edge.sessionPersistence || "v5",
    federated: edge.federated ?? false,
    crossWorkspace: edge.crossWorkspace ?? false,
    multiRepo: edge.multiRepo ?? false,
    multiOrg: edge.multiOrg ?? false,
    multiRegion: edge.multiRegion ?? false,
  });
}

/**
 * @param {string} root
 * @returns {string}
 */
function workspaceRegionKey(root) {
  const parts = root.replace(/^\/+/, "").split("/");
  if (parts.length >= 2 && parts[0] === "region") {
    return parts[1];
  }
  if (parts.length >= 4 && parts[0] === "org" && parts[2] === "region") {
    return parts[3];
  }
  return parts[0] || root;
}

/**
 * @param {string} sessionId
 * @param {string[]} workspaceRoots
 * @param {boolean} [multiRepo]
 * @param {boolean} [multiOrg]
 * @param {boolean} [multiRegion]
 */
function persistDapSessionV5(
  sessionId,
  workspaceRoots,
  multiRepo = workspaceRoots.length >= 2,
  multiOrg = false,
  multiRegion = false
) {
  if (!sessionId) {
    return;
  }
  const orgKeys = new Set(
    workspaceRoots.map((root) => {
      const parts = root.replace(/^\/+/, "").split("/");
      if (parts[0] === "org" && parts.length >= 2) {
        return parts[1];
      }
      return parts[0] || root;
    })
  );
  const regionKeys = new Set(workspaceRoots.map(workspaceRegionKey));
  dapSessionPersistenceV5.set(sessionId, {
    version: "v5",
    workspaceRoots,
    multiRepo,
    multiOrg: multiOrg || orgKeys.size >= 2,
    multiRegion: multiRegion || regionKeys.size >= 2,
    updatedAt: Date.now(),
  });
}

/**
 * @param {string} sessionId
 * @param {number} breakpointLine
 * @returns {{ targetPath: string; targetLine: number; targetCharacter: number } | undefined}
 */
function lookupBreakpointEdgeMapping(sessionId, breakpointLine) {
  const map = breakpointEdgeSessionMap.get(sessionId);
  if (!map) {
    return undefined;
  }
  return map.get(breakpointLine);
}

/**
 * @param {string | undefined} documentUri
 * @returns {Promise<{ nodes: unknown[]; edges: unknown[]; live?: boolean; phase?: number; bidirectional?: boolean; multiProcess?: boolean; sessionId?: string; persistentMapping?: boolean } | null>}
 */
async function fetchActorRoutesFromLsp(documentUri) {
  if (!client?.isRunning()) {
    return null;
  }
  const uri =
    documentUri ||
    vscode.window.activeTextEditor?.document.uri.toString() ||
    "";
  if (!uri) {
    return null;
  }
  try {
    const result = await client.sendRequest("workspace/executeCommand", {
      command: "syx.actorRoutes",
      arguments: [uri],
    });
    if (
      result &&
      typeof result === "object" &&
      Array.isArray(result.nodes) &&
      Array.isArray(result.edges)
    ) {
      return result;
    }
  } catch (err) {
    console.warn("syx.actorRoutes LSP request failed:", err);
  }
  return null;
}

/**
 * @param {string | undefined} documentUri
 * @returns {Promise<{ nodes: unknown[]; edges: unknown[]; live?: boolean; phase?: number; bidirectional?: boolean; multiProcess?: boolean; sessionId?: string; persistentMapping?: boolean } | null>}
 */
async function fetchActorRoutesMultiprocessFromLsp(documentUri) {
  if (!client?.isRunning()) {
    return null;
  }
  const uri =
    documentUri ||
    vscode.window.activeTextEditor?.document.uri.toString() ||
    "";
  if (!uri) {
    return null;
  }
  try {
    const result = await client.sendRequest("workspace/executeCommand", {
      command: "syx.actorRoutesMultiprocess",
      arguments: [uri],
    });
    if (
      result &&
      typeof result === "object" &&
      Array.isArray(result.nodes) &&
      Array.isArray(result.edges)
    ) {
      return result;
    }
  } catch (err) {
    console.warn("syx.actorRoutesMultiprocess LSP request failed:", err);
  }
  return null;
}

/**
 * @param {string | undefined} documentUri
 * @returns {Promise<{ nodes: unknown[]; edges: unknown[]; live?: boolean; phase?: number; bidirectional?: boolean; multiProcess?: boolean; federated?: boolean; crossWorkspace?: boolean; multiRepo?: boolean; multiOrg?: boolean; sessionPersistence?: string; sessionId?: string; persistentMapping?: boolean; workspaceRoots?: string[] } | null>}
 */
async function fetchActorRoutesFederatedFromLsp(documentUri) {
  if (!client?.isRunning()) {
    return null;
  }
  const uri =
    documentUri ||
    vscode.window.activeTextEditor?.document.uri.toString() ||
    "";
  if (!uri) {
    return null;
  }
  const workspaceRoots = (vscode.workspace.workspaceFolders || []).map(
    (f) => f.uri.fsPath
  );
  if (workspaceRoots.length === 0) {
    const root = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (root) {
      workspaceRoots.push(root);
    }
  }
  try {
    const result = await client.sendRequest("workspace/executeCommand", {
      command: "syx.actorRoutesFederated",
      arguments: [uri, workspaceRoots],
    });
    if (
      result &&
      typeof result === "object" &&
      Array.isArray(result.nodes) &&
      Array.isArray(result.edges)
    ) {
      return result;
    }
  } catch (err) {
    console.warn("syx.actorRoutesFederated LSP request failed:", err);
  }
  return null;
}

/**
 * @param {{ nodes: Array<{ id: string; label: string; kind: string; process?: string }>; edges: Array<{ from: string; to: string; kind: string; label: string; targetPath?: string; targetLine?: number; targetCharacter?: number; path?: string; line?: number; character?: number; multiProcess?: boolean; federated?: boolean; crossWorkspace?: boolean; multiOrg?: boolean; multiRegion?: boolean; sessionId?: string; sessionPersistence?: string }>; live?: boolean; phase?: number; bidirectional?: boolean; multiProcess?: boolean; federated?: boolean; crossWorkspace?: boolean; multiOrg?: boolean; multiRegion?: boolean; sessionPersistence?: string; sessionId?: string; persistentMapping?: boolean; workspaceRoots?: string[] }} graph
 * @param {boolean} [federated]
 */
function getActorRoutingGraphHtml(graph, federated = false) {
  const modeLabel = graph.live
    ? federated
      ? "LSP federated multi-region feed"
      : "LSP live multiprocess feed"
    : "mock fallback";
  const phaseLabel = graph.phase ?? 34;
  const biLabel = graph.bidirectional ? "bidirectional" : "one-way";
  const mpLabel = graph.multiProcess ? "multi-process" : "single-process";
  const fedLabel = graph.federated ? "federated" : "local";
  const repoLabel = graph.multiRepo ? "multi-repo" : "single-repo";
  const orgLabel = graph.multiOrg ? "multi-org" : "single-org";
  const regionLabel = graph.multiRegion ? "multi-region" : "single-region";
  const sessionLabel = graph.sessionId || "—";
  const persistLabel = graph.sessionPersistence || "v1";
  const wsRoots = (graph.workspaceRoots || []).join(", ") || "—";
  const nodeRows = graph.nodes
    .map(
      (n) =>
        `<tr><td>${n.id}</td><td>${n.label}</td><td>${n.kind}</td><td>${n.process || "—"}</td></tr>`
    )
    .join("");
  const edgeRows = graph.edges
    .map((e, idx) => {
      const targetPath = e.targetPath || e.path || "";
      const targetLine = e.targetLine ?? e.line ?? 0;
      const targetCharacter = e.targetCharacter ?? e.character ?? 0;
      return `<tr class="edge-row" data-idx="${idx}" data-target-path="${targetPath}" data-target-line="${targetLine}" data-target-character="${targetCharacter}" title="Click to navigate">
        <td>${e.from}</td><td>${e.to}</td><td class="kind-${e.kind.replace(/[^a-z-]/g, "")}">${e.kind}</td><td>${e.label}</td>
      </tr>`;
    })
    .join("");
  const edgePayload = JSON.stringify(
    graph.edges.map((e) => ({
      targetPath: e.targetPath || e.path || "",
      targetLine: e.targetLine ?? e.line ?? 0,
      targetCharacter: e.targetCharacter ?? e.character ?? 0,
    }))
  );
  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <style>
    body { font-family: var(--vscode-font-family); color: var(--vscode-foreground); background: var(--vscode-editor-background); padding: 12px; }
    h2 { margin: 0 0 8px; font-size: 1.1em; }
    table { border-collapse: collapse; width: 100%; margin-bottom: 16px; }
    th, td { border: 1px solid var(--vscode-panel-border); padding: 6px 8px; text-align: left; }
    th { background: var(--vscode-editor-inactiveSelectionBackground); }
    tr.edge-row { cursor: pointer; }
    tr.edge-row:hover { background: var(--vscode-list-hoverBackground); }
    .kind-in-process { color: var(--vscode-charts-green); }
    .kind-cross-process, .kind-cross-actor, .kind-cross-workspace, .kind-cross-org, .kind-cross-region { color: var(--vscode-charts-orange); }
    p.note { font-size: 0.9em; opacity: 0.85; }
  </style>
</head>
<body>
  <h2>SYX Actor Routing Graph (${modeLabel}, phase ${phaseLabel}, ${biLabel}, ${mpLabel}, ${fedLabel}, ${repoLabel}, ${orgLabel}, ${regionLabel})</h2>
  <p class="note">Phase 34: ${federated ? "federated multi-region CDN feed from LSP <code>syx.actorRoutesFederated</code>" : "multiprocess live feed from LSP <code>syx.actorRoutesMultiprocess</code>"}; session <code>${sessionLabel}</code> (persistence ${persistLabel}); workspaces: <code>${wsRoots}</code>; persistent breakpoint↔edge mapping via <strong>SYX: Sync Federated Routes</strong>.</p>
  <h2>Processes &amp; Actors</h2>
  <table>
    <thead><tr><th>ID</th><th>Label</th><th>Kind</th><th>Process</th></tr></thead>
    <tbody>${nodeRows}</tbody>
  </table>
  <h2>Routes</h2>
  <table>
    <thead><tr><th>From</th><th>To</th><th>Kind</th><th>Label</th></tr></thead>
    <tbody>${edgeRows}</tbody>
  </table>
  <script>
    const vscode = acquireVsCodeApi();
    const edgeTargets = ${edgePayload};
    document.querySelectorAll("tr.edge-row").forEach((row) => {
      row.addEventListener("click", () => {
        const idx = Number(row.getAttribute("data-idx"));
        const target = edgeTargets[idx] || {
          targetPath: row.getAttribute("data-target-path") || "",
          targetLine: Number(row.getAttribute("data-target-line") || 0),
          targetCharacter: Number(row.getAttribute("data-target-character") || 0),
        };
        vscode.postMessage({ type: "navigateEdge", ...target });
      });
    });
  </script>
</body>
</html>`;
}

/**
 * @param {{ targetPath?: string; targetLine?: number; targetCharacter?: number }} target
 */
async function navigateToEdgeTarget(target) {
  const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  let filePath = target.targetPath || "";
  if (filePath && workspaceRoot && !path.isAbsolute(filePath)) {
    filePath = path.join(workspaceRoot, "crates/syx-driver/tests", filePath);
  }
  if (!filePath) {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== "syx") {
      return;
    }
    const pos = new vscode.Position(target.targetLine || 0, target.targetCharacter || 0);
    editor.revealRange(new vscode.Range(pos, pos), vscode.TextEditorRevealType.InCenter);
    editor.selection = new vscode.Selection(pos, pos);
    return;
  }
  try {
    const uri = vscode.Uri.file(filePath);
    const doc = await vscode.workspace.openTextDocument(uri);
    const editor = await vscode.window.showTextDocument(doc, vscode.ViewColumn.One);
    const pos = new vscode.Position(target.targetLine || 0, target.targetCharacter || 0);
    editor.revealRange(new vscode.Range(pos, pos), vscode.TextEditorRevealType.InCenter);
    editor.selection = new vscode.Selection(pos, pos);
  } catch (err) {
    void vscode.window.showWarningMessage(
      `Could not open route target: ${filePath} (${err instanceof Error ? err.message : String(err)})`
    );
  }
}

/**
 * @param {number} breakpointLine
 * @param {string | undefined} documentUri
 * @param {boolean} [multiprocess]
 */
async function syncBreakpointLineToRoute(breakpointLine, documentUri, multiprocess = false, federated = false) {
  const graph = federated
    ? (await fetchActorRoutesFederatedFromLsp(documentUri)) || MOCK_ACTOR_ROUTING_GRAPH
    : multiprocess
      ? (await fetchActorRoutesMultiprocessFromLsp(documentUri)) || MOCK_ACTOR_ROUTING_GRAPH
      : (await fetchActorRoutesFromLsp(documentUri)) || MOCK_ACTOR_ROUTING_GRAPH;
  const sessionId = graph.sessionId || "";
  const edge = graph.edges?.find(
    (e) => (e.line ?? -1) === breakpointLine
  );
  if (!edge) {
    if ((multiprocess || federated) && sessionId) {
      const mapped = lookupBreakpointEdgeMapping(sessionId, breakpointLine);
      if (mapped) {
        await navigateToEdgeTarget(mapped);
        return;
      }
    }
    await vscode.window.showInformationMessage(
      `No route edge at line ${breakpointLine + 1}.`
    );
    return;
  }
  if (sessionId) {
    persistBreakpointEdgeMapping(sessionId, breakpointLine, edge);
    if (federated && graph.workspaceRoots) {
      persistDapSessionV5(
        sessionId,
        graph.workspaceRoots,
        graph.multiRepo ?? graph.workspaceRoots.length >= 2,
        graph.multiOrg ?? false,
        graph.multiRegion ?? false
      );
    }
  }
  await navigateToEdgeTarget({
    targetPath: edge.targetPath || edge.path,
    targetLine: edge.targetLine ?? edge.line ?? 0,
    targetCharacter: edge.targetCharacter ?? edge.character ?? 0,
  });
}

/**
 * @param {string | undefined} documentUri
 * @param {boolean} [federated]
 */
async function syncMultiprocessRoutes(documentUri, federated = false) {
  const graph = federated
    ? (await fetchActorRoutesFederatedFromLsp(documentUri)) ||
      MOCK_ACTOR_ROUTING_GRAPH
    : (await fetchActorRoutesMultiprocessFromLsp(documentUri)) ||
      MOCK_ACTOR_ROUTING_GRAPH;
  const sessionId = graph.sessionId || "";
  const edgeCount = graph.edges?.length ?? 0;
  if (sessionId && graph.edges) {
    for (const edge of graph.edges) {
      const line = edge.line ?? -1;
      if (line >= 0) {
        persistBreakpointEdgeMapping(sessionId, line, edge);
      }
    }
    if (federated && graph.workspaceRoots) {
      persistDapSessionV5(
        sessionId,
        graph.workspaceRoots,
        graph.multiRepo ?? graph.workspaceRoots.length >= 2,
        graph.multiOrg ?? false,
        graph.multiRegion ?? false
      );
    }
  }
  const mapped = sessionId
    ? breakpointEdgeSessionMap.get(sessionId)?.size ?? 0
    : 0;
  const v5Meta = sessionId ? dapSessionPersistenceV5.get(sessionId) : undefined;
  const label = federated ? "Federated" : "Multiprocess";
  await vscode.window.showInformationMessage(
    `${label} routes synced (session ${sessionId || "—"}, persistence ${v5Meta?.version || "v1"}, multiRepo ${v5Meta?.multiRepo ?? false}, multiOrg ${v5Meta?.multiOrg ?? false}, multiRegion ${v5Meta?.multiRegion ?? false}): ${edgeCount} edges, ${mapped} persistent breakpoint mappings.`
  );
  return graph;
}

/**
 * @param {vscode.ExtensionContext} context
 */
function registerActorRoutingGraphCommand(context) {
  context.subscriptions.push(
    vscode.commands.registerCommand("syx.showFederatedRoutingGraph", async () => {
      const active = vscode.window.activeTextEditor?.document;
      const live = await fetchActorRoutesFederatedFromLsp(active?.uri.toString());
      const graph = live || MOCK_ACTOR_ROUTING_GRAPH;
      const panel = vscode.window.createWebviewPanel(
        "syxFederatedRoutingGraph",
        "SYX Federated Routing Graph (Multi-Region)",
        vscode.ViewColumn.Beside,
        { enableScripts: true, retainContextWhenHidden: true }
      );
      panel.webview.html = getActorRoutingGraphHtml(graph, true);
      panel.webview.onDidReceiveMessage((message) => {
        if (message?.type === "navigateEdge") {
          void navigateToEdgeTarget(message);
        }
      });
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("syx.showActorRoutingGraph", async () => {
      const active = vscode.window.activeTextEditor?.document;
      const live = await fetchActorRoutesFederatedFromLsp(active?.uri.toString());
      const graph = live || MOCK_ACTOR_ROUTING_GRAPH;
      const panel = vscode.window.createWebviewPanel(
        "syxActorRoutingGraph",
        "SYX Actor Routing Graph (Federated Multi-Region)",
        vscode.ViewColumn.Beside,
        { enableScripts: true, retainContextWhenHidden: true }
      );
      panel.webview.html = getActorRoutingGraphHtml(graph, true);
      panel.webview.onDidReceiveMessage((message) => {
        if (message?.type === "navigateEdge") {
          void navigateToEdgeTarget(message);
        }
      });
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("syx.syncBreakpointToRoute", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || editor.document.languageId !== "syx") {
        await vscode.window.showWarningMessage(
          "Open a .sy file to sync breakpoint to route target."
        );
        return;
      }
      const line = editor.selection.active.line;
      await syncBreakpointLineToRoute(line, editor.document.uri.toString(), true, true);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("syx.syncFederatedRoutes", async () => {
      const editor = vscode.window.activeTextEditor;
      const uri = editor?.document?.uri.toString();
      await syncMultiprocessRoutes(uri, true);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("syx.syncMultiprocessRoutes", async () => {
      const editor = vscode.window.activeTextEditor;
      const uri = editor?.document?.uri.toString();
      await syncMultiprocessRoutes(uri, true);
    })
  );

  // Palette command must not use `syx.actorRoutes` — LanguageClient registers that
  // ID from LSP executeCommandProvider (ExecuteCommandFeature).
  context.subscriptions.push(
    vscode.commands.registerCommand("syx.fetchActorRoutes", async () => {
      const active = vscode.window.activeTextEditor?.document;
      const graph =
        (await fetchActorRoutesFromLsp(active?.uri.toString())) ||
        MOCK_ACTOR_ROUTING_GRAPH;
      const edgeCount = graph.edges?.length ?? 0;
      const nodeCount = graph.nodes?.length ?? 0;
      const source = graph.live ? "LSP live" : "mock fallback";
      await vscode.window.showInformationMessage(
        `SYX actor routes (${source}): ${nodeCount} nodes, ${edgeCount} edges`
      );
      return graph;
    })
  );
}

/**
 * @param {vscode.ExtensionContext} context
 */
function registerLanguageServerCommands(context) {
  context.subscriptions.push(
    vscode.commands.registerCommand("syx.restartLanguageServer", async () => {
      if (!extensionContext) {
        extensionContext = context;
      }
      await startLanguageClient(extensionContext);
    })
  );
  context.subscriptions.push(
    vscode.commands.registerCommand("syx.checkLanguageServer", async () => {
      const syxPath = resolveSyxPath();
      const missing = missingBinaryMessage(syxPath);
      if (missing) {
        await vscode.window.showErrorMessage(missing);
        return;
      }
      if (client?.isRunning()) {
        await vscode.window.showInformationMessage(
          `SYX LSP is running (${syxPath}). Type \`.\` after a variable to trigger member completion.`
        );
        return;
      }
      await vscode.window.showWarningMessage(
        `SYX binary found at ${syxPath}, but LSP is not running. Use "SYX: Restart Language Server".`
      );
    })
  );
}

/**
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
  statusBarItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    100
  );
  statusBarItem.command = "syx.checkLanguageServer";
  context.subscriptions.push(statusBarItem);

  registerDebugSupport(context);
  registerActorRoutingGraphCommand(context);
  registerLanguageServerCommands(context);

  try {
    void startLanguageClient(context);
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    const syxPath = resolveSyxPath();
    console.error("SYX LSP failed to start:", err);
    updateStatusBar(
      "failed",
      `SYX LSP failed to load (${message}). Is npm install done in editors/vscode-syx?`
    );
    void vscode.window.showErrorMessage(
      `SYX LSP failed to load (${message}). Expected binary at ${syxPath}. ` +
        "Run `cargo build -p syx-cli` and `npm install` in editors/vscode-syx."
    );
  }
}

function deactivate() {
  return disposeLanguageClient(client);
}

module.exports = { activate, deactivate };
