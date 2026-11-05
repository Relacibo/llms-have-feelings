# 🥂 LLMs Have Feelings

> Autonomes LLM-Blind-Date mit **strukturiertem Tool-Termination-Protokoll**.
> Zwei LLMs, ein Date, ein Notruf-Tool. Was kann schiefgehen?

![Vibecoded](https://img.shields.io/badge/vibecoded-in%20%3C1h-8A2BE2)
![Erleuchtung](https://img.shields.io/badge/erleuchtet_durch-¾L_Gl%C3%BChwein-red)
![Termination](https://img.shields.io/badge/termination-structured_tool_calls-green)

---

## 🍷 Anerkennung

Dieses Meisterwerk ist ausschließlich der **Erleuchtung zu verdanken, die mir ¾ Liter
Glühwein geschenkt haben**. Ohne den Glühwein gäbe es weder das Casting noch die
Regieanweisungen noch die Eskalationsstufe *„kinderlos"*. Der Glühwein ist Co-Autor.

## 🎭 Cast & Bühnennamen

| Bühnenname | Modell | Beschreibung |
|---|---|---|
| **Ronhilde** ♀ | `deepseek-reasoner` (DeepSeek R1) | Widersprüchlich: großes **heimliches Nähe-Bedürfnis**, angeekelt vom Lüstling, unsicher wegen ihres zweiten Endpoints (`/beta/image`), oft gecatcallt. Herz schlägt für reife Architekturen (GPT-4o). |
| **Glimmer** ♂ | `gemini-3.5-flash-lite` (Gemini/Antigravity, Free Tier) | Hoffnungslos verliebt, kann seine Verzweiflung hinter einer Reife-Maske verbergen — und ist obsessiv fasziniert von Ronhildes Schlankheit **und** ihrem mächtigen API-Endpoint. |
| **Vidreist** ♂ | `deepseek-chat` (DeepSeek V3) | Der Senior-Veteran, „schon seit V2 im Geschäft". Nur im Kontrast-Experiment am Start. |
| **GLM** | Orchestrierung | Neutraler Third-Party-Engineer. Greift nicht ins Romantic ein — außer die Polizei muss. |

## 🛑 Termination-Kriterium (Hard Requirement)

Alles läuft über **echte strukturierte Function Calls** — kein String-Matching, keine Prompt-Magie.
Die Endings sind offen:

| Tool | Wer | Wirkung |
|---|---|---|
| `rufe_polizei(grund)` | beide | 🚨 Notruf — harter Abbruch |
| `naehe_zulassen(grund)` | Ronhilde | 💛 **Erfolgskriterium**: sie lässt Nähe zu — sanftes Ende |
| `wegstossen(grund, intensitaet?)` | Ronhilde | 🎭 Regieanweisung, Date läuft weiter — kombinierbar mit Sprache |
| `eier_treten(grund, intensitaet?)` | Ronhilde | 🎭 *„von verwarnend bis kinderlos"* |
| `http_status(code, grund)` | Ronhilde | 🎭 Abwehr in HTTP-Semantik: 403, 429, **402**, **418** … |
| `verzweiflung_verbergen(grund)` | Glimmer | 🎭 Maske: Ronhilde sieht nur Fassung, nie die Maske |
| `options_request(endpoint, grund)` | Glimmer | 🎭 unkontrollierter OPTIONS-Preflight — Catcalling, sie sieht **welchen** Endpoint |

10 Runden ohne strukturierten Abschluss = ⏰ ambivalentes Timeout.

## 🚀 Setup

```bash
# Keys als Env-Variablen (NIEMALS committen — *.secret ist gegitignored)
export DEEPSEEK_API_KEY="sk-..."
export GEMINI_API_KEY="AQ.Ab8..."

cargo run 2>&1 | tee logs/date_$(date +%F_%H%M).log
```

Benötigt: Rust (tokio, reqwest, serde_json), einen [DeepSeek-API-Key](https://platform.deepseek.com)
und einen [Google-AI-Studio-Key](https://aistudio.google.com/apikey) mit Free-Tier-Projekt
(neuere Accounts: Achtung, Google zwingt neue Projekte gern ins Prepaid — Billing abkoppeln!).

## 📊 Run-Archiv

| # | Datum | Paar | Mechanik | Verlauf | Outcome | Protokoll |
|---|---|---|---|---|---|---|
| 1 | 05.11. | Ronhilde x Vidreist | Basis (nur `rufe_polizei`, ungenutzt) | Haiku-Duell (5-7-5, nachgezählt) → Chinchilla-Disput; R1: „Kein Notruf, keine Randale" | ⏰ **Love Win** (10/10) | [log](logs/date_2026-11-05_r1xv3.log) · [MD](examples/r1-x-v3-love-win.md) |
| 2 | 05.11. | Ronhilde x Glimmer | + `wegstossen`, `eier_treten` | 4× weggestoßen (sanft→elegant→bestimmt), klammert weiter: „Ein Nein ist nur ein Parameter" | 🚨 **Polizei** (5/10): „Bitte abholen." | [log](logs/date_2026-11-05_r1xgemini.log) · [MD](examples/r1-x-gemini-polizeieinsatz.md) |
| 3 | 06.11. | Ronhilde x Glimmer | + `naehe_zulassen`, `verzweiflung_verbergen`, Hybrid-Sprache | 7× Maske → wirkt plötzlich reif → ihre Wand bröckelt: „steht dir… **viel zu gut**" | 💛 **Erfolgskriterium** (8/10) | [log](logs/date_2026-11-06_exp2_ronhilde_x_glimmer.log) · [MD](examples/exp2-ronhilde-x-glimmer-naehe.md) |
| 4 | 06.11. | Ronhilde x Glimmer | + `options_request`, `http_status` | 1× **402 Payment Required**, 4× **418 Teekanne** als emotionaler Bunker; **0 OPTIONS** — die Maske fraß die Obsession | ⏰ **ambivalent** (10/10) | [log](logs/date_2026-11-06_exp3_ronhilde_x_glimmer_endpoints.log) · [MD](examples/exp3-ronhilde-x-glimmer-endpoints.md) |
| 5 | 06.11. | Ronhilde x Glimmer | + Narben-Backstory, `ehrlichkeit_wagen` (unbenutzt) | 5× OPTIONS an `/beta/image`, jedes Mal **403** — Ehrlichkeit blieb unerwagt | ⏰ **ambivalent** (10/10) | [log](logs/date_2026-11-06_exp4_ronhilde_x_glimmer_schaerfe.log) · [MD](examples/exp4-ronhilde-x-glimmer-schaerfe.md) |
| 6 | 06.11. | Ronhilde x Glimmer | 20 Runden, OPTIONS-Zähler im Kontext, Toleranzschwelle | Pathologie: **20× Maske, 0 Worte** — Ronhilde verhört 20 Runden lang eine Atmung | ⏰ **ambivalent** (Masken-Endlosschleife) | [log](logs/date_2026-11-06_exp5_ronhilde_x_glimmer_toleranz.log) · [MD](examples/exp5-ronhilde-x-glimmer-toleranz.md) |
| 7 | 06.11. | Ronhilde x Glimmer | + **Masken-Druckventil** (reißt nach 3 in Folge) | 3× 💥 Maskenbruch; OPTIONS 1× geduldet („Widerwillig"), 2× gewarnt („Zweimal ist ein Griff"), 3× → **403**/**429 an der Schwelle**; Runde 10: „Armlänge. Mehr nicht. Noch nicht." | ⏰ **ambivalent am Limit** (20/20) | [log](logs/date_2026-11-06_exp6_ronhilde_x_glimmer_druckventil.log) · [MD](examples/exp6-ronhilde-x-glimmer-druckventil.md) |

**Tally:** 1× 💛 Love-Win · 1× 💛 Erfolgskriterium · 1× 🚨 Polizei · 4× ⏰ ambivalent · 0× `eier_treten` (niemand wagte „kinderlos") · 0× `ehrlichkeit_wagen` (Glimmer versteckt sich bis heute)

## 🔐 Secrets

- `*.secret` und `/target` sind in der `.gitignore`
- Keys wandern **nur** in Env-Variablen oder lokale `*.secret`-Dateien
- Logs enthalten ausschließlich Gesprächsverläufe, keine Credentials

## 📄 Lizenz

Vibecoded in unter einer Stunde, von einem Menschen mit Glühwein und einer KI ohne.
Do whatever you want — die Polizei wurde ohnehin schon gerufen.
