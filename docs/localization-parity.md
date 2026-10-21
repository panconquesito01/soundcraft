# Localization parity

> **Last reviewed:** 2026-10-20 · **Last updated:** 2026-10-20 · **Change:** minor (stable Languages & Themes selector and translated theme controls) · **Target:** Avid Pro Tools Ultimate 2026.4.1

SoundCraft now has a typed initial string catalog and a persistent selector for English, Spanish,
Portuguese, French, German, Russian and Japanese. The migrated surface is deliberately small:
main menu roots, Dashboard, Languages & Themes and the Piano Roll. Unmigrated labels, dialogs and tooltips
fall back to their English source text. Pro Tools ships its UI in
**8 languages** (bundle `.lproj` folders: English, German, Spanish, French, Japanese, Korean,
Simplified Chinese, Traditional Chinese).

**Localization remains ~12 % overall**: English is still the only complete language and the other
six selectable languages cover only the initial core surface. Remaining ≈ 60–100 h: migrate the
full command/dialog catalog, add external catalogs, interpolation/pluralization and pseudo-locale,
broaden CJK/Devanagari/Arabic fallback, then RTL layout and complex-script shaping (egui has neither:
≈ 25–40 h, or ship Arabic/Hindi menus-only), then ≈ 3–5 h of agent translation per language, plus
native-speaker review (human).

The desktop UI loads Japanese/CJK fallback from installed OS fonts and never redistributes those
fonts. Web still needs a permissively licensed bundled fallback. Open work remains for full
French/Spanish coverage, broader multilingual fallbacks and issue #67 (Ukrainian).

| Language | Code | UI strings translated | Dialogs / tooltips / help | Script support | Native review | Status | Est. to full (h) | In Pro Tools |
|---|---|---|---|---|---|---|---:|---|
| English | en | all (source) | ✅ | Latin ✅ | — | **full** | 0 | ✅ |
| Simplified Chinese | zh-Hans | 0 (0 %) | ❌ | ❌ no CJK font fallback, no IME testing | no | none | 6–10 | ✅ |
| Spanish | es | initial core surface | partial | Latin ✅ | no | **partial** | 3–5 | ✅ |
| Hindi | hi | 0 (0 %) | ❌ | ❌ no Devanagari font or shaping | no | none | 8–15 | ❌ |
| Arabic | ar | 0 (0 %) | ❌ | ❌ no RTL layout, no shaping | no | none | 15–25 | ❌ |
| French | fr | initial core surface | partial | Latin ✅ | no | **partial** | 3–5 | ✅ |
| Portuguese | pt | initial core surface | partial | Latin ✅ | no | **partial** | 3–5 | ❌ |
| Indonesian | id | 0 (0 %) | ❌ | Latin ✅ | no | none | 3–5 | ❌ |
| Japanese | ja | initial core surface | partial | desktop OS fallback; web/IME unverified | no | **partial** | 6–10 | ✅ |
| German | de | initial core surface | partial | Latin ✅ | no | **partial** | 3–5 | ✅ |
| Russian | ru | initial core surface | partial | Cyrillic via desktop UI fonts | no | **partial** | 3–5 | ❌ |
| Korean | ko | 0 (0 %) | ❌ | ❌ no Hangul font fallback | no | none | 6–10 | ✅ |
| Vietnamese | vi | 0 (0 %) | ❌ | 🟡 Latin with stacked diacritics, untested | no | none | 3–5 | ❌ |

No other languages are selectable. Pro Tools also ships Traditional Chinese (`zh-Hant`); Russian
is additional to the target's eight-language set.

Per-language hours exclude the shared catalog and script work above.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-20 | minor | Renamed the selector to Languages & Themes, kept its modal position stable across translations, and localized the new theme controls |
| 2026-10-17 | minor | Added persistent seven-language selector, typed fallback catalog, initial core translations and desktop Japanese font fallback; full localization remains open |
| 2026-10-10 | major | Created: English only, no catalog; the twelve key languages at 0 %; Pro Tools' 8 languages listed from its bundle |
