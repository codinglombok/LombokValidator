# Translation catalogs

| Language | Code | Status |
|---|---|---|
| English | `en` | ✅ source of truth |
| Bahasa Indonesia | `id` | ✅ written by the ecosystem owner |
| Español, Français, Deutsch, Português, Русский, 日本語, 한국어, 简体中文 | `es` `fr` `de` `pt` `ru` `ja` `ko` `zh-Hans` | 🟡 **machine-translation drafts — native-speaker review required** before they are considered final (MASTERPLAN_UTAMA §13) |

Rule: keys (`messageId`) and placeholders (`{field}`, `{min}`, …) must be identical to `en`; verified by `cargo test` (tests/catalogs.rs).
Not yet covered from Core-20: `ar` (needs RTL/BiDi), `hi`, `bn`, `tr`, `vi`, `th`, `it`, `pl`, `nl`, `uk`, plus the Nusantara languages (`jv`, `su`, `ms`).
