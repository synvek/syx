# soga — SYX LLM chat library

用 SYX 写的 LLM 聊天库与非交互范例。支持 **OpenAI Chat Completions** 与 **Anthropic Messages** 两套接口；DeepSeek / MiniMax（以及官方 OpenAI / Anthropic）均可作为后端。

当前**不做交互式 REPL**。演示是 one-shot + 脚本化多轮。

## 快速开始

```bash
# 只看将要发出的请求（不访问网络）
SOGA_DRY_RUN=1 SOGA_DEMO=deepseek syx run tools/soga
SOGA_DRY_RUN=1 SOGA_DEMO=minimax  syx run tools/soga
SOGA_DRY_RUN=1 SOGA_DEMO=both     syx run tools/soga
SOGA_DRY_RUN=1 SOGA_DEMO=anthropic-paths syx run tools/soga

# 真实调用（需 API Key）
export DEEPSEEK_API_KEY=sk-...
SOGA_DEMO=deepseek syx run tools/soga

export MINIMAX_API_KEY=...
SOGA_DEMO=minimax syx run tools/soga
```

开发机可用仓库内 CLI：

```bash
cargo run -p syx-cli -- run tools/soga
```

## 布局

同目录多 `.sy` 属于同一个 `syx.toml` 包：共享命名空间，**不要**再写 `import message` 这类 file module。

| 文件 | 作用 |
|------|------|
| `message.sy` | 消息 / 历史（`{ messages: [...] }`） |
| `providers.sy` | DeepSeek / MiniMax / OpenAI / Anthropic 预设 |
| `openai_compat.sy` | OpenAI 兼容 `POST …/chat/completions` |
| `anthropic_api.sy` | Anthropic 兼容 `POST …/v1/messages` |
| `response.sy` | 统一 ChatResult |
| `client.sy` | 按 `api_style` 分发 |
| `session.sy` | 非交互多轮会话 |
| `config.sy` | 环境变量 |
| `main.sy` | 范例入口 |
| `json_util.sy` | JSON / URL 小工具 |

## 环境变量

| 变量 | 说明 |
|------|------|
| `SOGA_DEMO` | `deepseek` / `minimax` / `both` / `anthropic-paths` / `provider` |
| `SOGA_PROVIDER` | `provider` 模式下的提供商名 |
| `SOGA_API_KEY` | 覆盖提供商专用 key |
| `SOGA_BASE_URL` / `SOGA_MODEL` | 覆盖 base / 模型 |
| `SOGA_MAX_TOKENS` / `SOGA_TIMEOUT_MS` | 生成长度 / 超时 |
| `SOGA_DRY_RUN` | `1` → 只打印请求 |
| `DEEPSEEK_API_KEY` / `MINIMAX_API_KEY` / `OPENAI_API_KEY` / `ANTHROPIC_API_KEY` | 各厂商 key |

## 预设

| 名称 | 风格 | 默认 base | 默认模型 |
|------|------|-----------|----------|
| `deepseek` | openai | `https://api.deepseek.com` | `deepseek-chat` |
| `deepseek-anthropic` | anthropic | `https://api.deepseek.com/anthropic` | `deepseek-chat` |
| `minimax` | openai | `https://api.minimax.io/v1` | `MiniMax-M2.5` |
| `minimax-anthropic` | anthropic | `https://api.minimax.io/anthropic` | `MiniMax-M2.5` |
| `openai` | openai | `https://api.openai.com/v1` | `gpt-4o-mini` |
| `anthropic` | anthropic | `https://api.anthropic.com` | `claude-sonnet-4-20250514` |

## 库用法

同包多文件共享命名空间（无需 `import message` 这类 file module）：

```syx
import Env

func main() {
    var cfg = deepseek()
    cfg = withApiKey(cfg: cfg, apiKey: Env.get("DEEPSEEK_API_KEY"))

    var history = newHistory()
    let user = makeMessage(role: "user", content: "Hello")
    history = historyAppend(history: history, msg: user)

    let result = chatComplete(cfg: cfg, history: history)
    printResult(r: result)
}
```

## SYX 实现注意（本范例踩过的坑）

1. **用 `.field` 读 Object**，不要用 `obj["key"]`（当前会返回整个对象）。
2. **String 按值传递会 move**；同一变量不要传两次，字面量可在每个调用点各写一份。
3. **单标签调用**如 `userMessage(content: x)` 可能被当成 `Type_init`；请用 `makeMessage(role:content:)`。
4. **拼 JSON 优先 `+` 拼接 + `JSON.encode`**，避免 `\(…)` 把已转义内容再次弄坏。
5. **Array 请 `JSON.encode(value: arr)`**，不要塞进只接受 `Object` 的 encode 封装。

流式 / 工具调用 / 交互式 REPL 可后续再加。
