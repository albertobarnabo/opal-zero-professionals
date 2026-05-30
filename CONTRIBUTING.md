# Contributing to OpalZero Professionals

Professionals are how OpalZero gains new abilities. Each one is a small Rust crate that compiles to WebAssembly and is loaded by the kernel **at runtime, without touching the core engine**. That makes this the easiest, highest-leverage place to contribute: **add the tool you wish OpalZero had, and it ships to every install.**

## 👋 New here? Start with a good first issue

Browse issues labeled [`good first issue`](https://github.com/albertobarnabo/opal-zero-professionals/issues?q=is%3Aopen+label%3A%22good+first+issue%22) — each is small, self-contained, and has acceptance criteria. **Comment to claim one** and the maintainer will help you through it.

## Add a Professional in 4 steps

1. **Implement the `Professional` trait** — a `name()` and an async `run(ctx)`:
   ```rust
   use opalzero_sdk::{Professional, Context, Output};

   #[derive(Default)]
   pub struct CurrencyConverter;

   impl Professional for CurrencyConverter {
       fn name(&self) -> &'static str { "currency_convert" }

       async fn run(&self, ctx: Context) -> Result<Output, opalzero_sdk::Error> {
           let amount: f64 = ctx.get("amount")?;
           let rate:   f64 = ctx.get("rate")?;
           Ok(Output::text(format!("{:.2}", amount * rate)))
       }
   }

   opalzero_sdk::export_professional!(CurrencyConverter);
   ```
2. **Compile to WASM:** `wasm-pack build --target bundler --out-dir pkg`
3. **Drop it in the registry:** `cp pkg/*_bg.wasm /path/to/opalzero/registry/`
4. **Open a PR** with a short README for your tool (its intent, inputs, and outputs).

See the existing tools in this repo for patterns: copy **`calculator`** for a pure/no-network tool, or **`web_search`** for one that calls an external API.

## Guidelines

- **One Professional per PR** — keeps reviews fast.
- **Include a test** (`cargo test`).
- **No secrets in code** — read API keys from the `Context`, never hard-code them.
- **Keys-optional first** — prefer free / no-key APIs for new tools where possible.
- Follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Dev setup

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo test --workspace
```

## Questions?

Open a [Discussion](https://github.com/albertobarnabo/opal-zero-engine/discussions) or email **albertobarnabo@gmail.com**. First PR? Don't worry about getting it perfect — open it early and we'll iterate together.
