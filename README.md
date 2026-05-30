# opalzero-professionals

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)

**The WASM tool registry that gives OpalZero agents their superpowers.**

Apps delegate their AI to the [OpalZero kernel](https://github.com/albertobarnabo/opalzero-engine) — and **Professionals are how you widen what the kernel can do.** Each one is a Rust crate that compiles to WebAssembly and extends what OpalZero agents can do at runtime, without touching the core engine. Drop a `.wasm` binary into the registry directory and the kernel picks it up on next start. No recompilation, no config changes, no restarts beyond the initial load. Every Professional you add becomes a capability every delegating app inherits for free.

---

## 🔭 OpalGlimpse — the first product built on OpalZero *(coming soon)*

Autonomous monitoring powered by OpalZero: point it at markets, competitors, or any signal — it runs on a schedule and shows you **exactly what changed**, as structured diffs, not noise. *Watch the world change while you sleep.*

It launches as a hosted SaaS, and **we deploy it once there's enough interest.** Want early access?

- 👍 or comment on the **[OpalGlimpse early-access issue →](https://github.com/albertobarnabo/opal-zero-engine/issues/1)**
- or email **albertobarnabo@gmail.com**

---

## Built-in tools

| # | Tool | What it does |
|---|---|---|
| 🧮 | `calculator` | Safe arithmetic expression evaluator — agents use it instead of hallucinating math |
| 🗂️ | `memory` | Per-mission key/value store — read and write context within a single mission |
| 🧠 | `memory_persist` | Cross-mission persistent memory — writes to `memory/global.json`, survives restarts |
| 🔍 | `web_search` | Live web search via Tavily API — returns ranked results with source URLs |
| 👁️ | `vision` | Image analysis via GPT-4o vision — works on uploads and URLs |
| 🐍 | `python_interpreter` | Sandboxed Python 3 execution — runs agent-generated code safely |
| 🙋 | `feedback` | Human-in-the-loop pause/resume — agent asks a question, mission waits for your answer |
| 📄 | `generate_document` | Export mission findings as MD, CSV, or styled HTML |
| 📂 | `read_file` | Read uploaded data files (CSV, JSON, TXT) into agent context |

---

## Build your own in 3 steps

**1. Implement the `Professional` trait**

```rust
use opalzero_sdk::{Professional, Context, Output};

#[derive(Default)]
pub struct CurrencyConverter;

impl Professional for CurrencyConverter {
    fn name(&self) -> &'static str { "currency_convert" }

    async fn run(&self, ctx: Context) -> Result<Output> {
        let amount: f64 = ctx.get("amount")?;
        let rate:   f64 = ctx.get("rate")?;
        Ok(Output::text(format!("{:.2}", amount * rate)))
    }
}

opalzero_sdk::export_professional!(CurrencyConverter);
```

**2. Compile to WASM**

```bash
wasm-pack build --target bundler --out-dir pkg
```

**3. Drop it in the registry**

```bash
cp pkg/currency_converter_bg.wasm /path/to/opalzero/registry/
# The kernel loads it on next start — agents can call "currency_convert" immediately
```

---

## Contributing

**New contributors welcome — this is the easiest place to start.** Pick a [`good first issue`](https://github.com/albertobarnabo/opal-zero-professionals/issues?q=is%3Aopen+label%3A%22good+first+issue%22), comment to claim it, and follow **[CONTRIBUTING.md](./CONTRIBUTING.md)** — you can ship a new tool in an afternoon, without touching the core engine.

Full docs: [albertobarnabo.com/opal-zero/docs](https://albertobarnabo.com/opal-zero/docs).

---

## License

MIT

⭐ Built something useful? Open a PR — community Professionals are welcome.
