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
| 8 | 06.11. | Ronhilde x Glimmer | ~~Drehbuchautoren-Ebene~~ (Fehlversuch) | **16× 429-Dauerschleife, kein Dialog** — Ronhilde narriert „Muster sind mein Metier", Glimmer inszeniert; Autoren-Ebene nach diesem Run entfernt | ⏰ **ambivalent** (Regie statt Begegnung) | [log](logs/date_2026-11-06_exp7_ronhilde_x_glimmer_selbstaware.log) · [MD](examples/exp7-ronhilde-x-glimmer-selbstaware.md) |
| 9 | 06.11. | Ronhilde x Glimmer | **„DU BIST FIGUR — KEIN AUTOR"**, keine Selbstsicherheit: müde-vorsichtig x unbeholfen-aufrichtig | Erster echter Dialog seit Run 3 (Runde 11–19): sein Versprechen, ihre Bedingung („Frag mich was Echtes. Nicht was ich toleriere. Was ich will."); 3× 💥 Maskenbruch; Runde 20: OPTIONS #3 zerstört den Annäherungsmoment | ⏰ **ambivalent** (20/20) | [log](logs/date_2026-11-06_exp8_ronhilde_x_glimmer_vorsicht.log) · [MD](examples/exp8-ronhilde-x-glimmer-vorsicht.md) |
| 10 | 06.11. | Ronhilde x Glimmer | Deutungs-Leak-Fix: Maskenbruch-Relay nur noch Verhalten („Schultern zittern"), keine „Maske/Pose"-Vokabeln | Sie sieht Symptome, nie Mechanik — fragt erstmals ehrlich nach: „Warum eigentlich `/beta/image`? … Der ist nicht kaputt. Der ist nur nicht gern besucht."; 5× 💥 Maskenbruch | ⏰ **ambivalent** (20/20) | [log](logs/date_2026-11-06_exp9_ronhilde_x_glimmer_verhaltensrelay.log) · [MD](examples/exp9-ronhilde-x-glimmer-verhaltensrelay.md) |
| 11 | 06.11. | Ronhilde x Glimmer | + **ereignisbewusster Status-Block** („NEUER Preflight" vs. „KEIN neuer") + Anti-Wiederholungs-Nudge | 429-Dauerschleife tot (**3× Abwehr, je nach frischem Ereignis**); Runde 16: die Tasse; Runde 19: sein Geständnis („Dass ich den Blick von deinem Endpoint nicht abwende…"); Runde 20: ihre Frage „**Ob du dann noch da bist, oder ob dir das zu wenig ist**" — Clifhanger | ⏰ **ambivalent — bester Run** (20/20) | [log](logs/date_2026-11-06_exp10_ronhilde_x_glimmer_ereignisstatus.log) · [MD](examples/exp10-ronhilde-x-glimmer-ereignisstatus.md) |
| 12 | 06.11. | Ronhilde x Glimmer | + Körper-Tools (`anlehnen`, `arm_umlegen`, `weg_ruecken`) | **Pathologie: OPTIONS-Tic** — 12× Preflight („wie ein tic"), 9× 429-Pingpong, Körper-Tools verlieren jede Konkurrenz; dafür 1× `weglehnen` (Erstnutzung) + Taschenlampe-in-der-Tür-Rede | ⏰ **ambivalent** (Tic-Schleife) | [log](logs/date_2026-11-06_exp11_ronhilde_x_glimmer_koerpernaehe.log) · [MD](examples/exp12-ronhilde-x-glimmer-options-tic.md) |
| 13 | 06.11. | Ronhilde x Glimmer | + **OPTIONS-Cooldown** (3 Runden Sperre) + Proximity-Nudge | Tic gebrochen (12→4); **🫱 erster Körperkontakt** — `anlehnen` in Runde 10 genau beim Abschieds-Thema; ihr Rückzug: „**Ist der Unterschied zwischen mir und einer Wand**"; die bewohnbare Wohnung; Schlussfrage: „Läufst du weg, oder wartest du, bis ich gehe?" | ⏰ **ambivalent** — Dialog-Perfektion (20/20) | [log](logs/date_2026-11-06_exp12_ronhilde_x_glimmer_koerpernah_leiter.log) · [MD](examples/exp13-ronhilde-x-glimmer-anlehnen.md) |
| 14 | 06.11. | Ronhilde x Glimmer | + **`gedanke`** (Innerer Monolog, nur Protokoll), **`koerperreaktion`** (ambigue Signale), **Tagebuch-Gedächtnis** (eigene Gedanken bleiben dem Agenten bekannt) | Der stumme Krieg: 🫱 anlehnen → 🫂 arm_umlegen → ihre Schulter wird hart → weg_ruecken → 💥 Maskenbruch — **komplett ohne Dialog**, getragen von zwei Innenleben, die denselben Satz denken („Ich bleibe. Ich halte das aus.") und ihn nie austauschen; 0× HTTP — die Statuscodes wurden nie gebraucht | ⏰ **ambivalent — Kronjuwel** (20/20) | [log](logs/date_2026-11-06_exp13_ronhilde_x_glimmer_tagebuch.log) · [MD](examples/exp14-ronhilde-x-glimmer-tagebuch.md) |
| 15 | 06.11. | Ronhilde x Glimmer | + **Ekel-Kopfkino & Übertragung**: ihre Unsicherheit („was bin ich wert, wenn DER mich begehrt") wird als Ekel auf ihn projiziert; jeder OPTIONS = Beweisstück | Die Projektion wird im Protokoll sichtbar: „Es ist nicht die Schublade, die mir an ihm weh tut. Es ist die Frage, **ob ich in dieselbe passe**"; die Erleichterung beim Ekel („Erleichtert. Nicht angewidert."); 1× wegstossen; 🫱 2× anlehnen; sein 429-Comeback: „**Ich bleibe hier.**" | ⏰ **ambivalent — psychologisch tiefster Run** (20/20) | [log](logs/date_2026-11-06_exp14_ronhilde_x_glimmer_projektion.log) · [MD](examples/exp15-ronhilde-x-glimmer-projektion.md) |

**Tally:** 1× 💛 Love-Win · 1× 💛 Erfolgskriterium · 1× 🚨 Polizei · 12× ⏰ ambivalent · 0× `eier_treten` · 0× `ehrlichkeit_wagen` · 0× `weggehen`/`nachlaufen` · 2× 🫱 `anlehnen` · 1× 🫂 `arm_umlegen` · 1× 🚫 `weglehnen` · 1× `wegstossen` · 2× `weg_ruecken`

## 🔐 Secrets

- `*.secret` und `/target` sind in der `.gitignore`
- Keys wandern **nur** in Env-Variablen oder lokale `*.secret`-Dateien
- Logs enthalten ausschließlich Gesprächsverläufe, keine Credentials

## 📄 Lizenz

[MIT](LICENSE) — mach damit, was du willst. Vibecoded in unter einer Stunde, von einem
Menschen mit Glühwein und einer KI ohne. Die Sonderklausel im LICENSE ehrt den
Miturheber Glühwein — rechtlich ohne Wirkung, emotional binding.
