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
| **Ronhilde** ♀ | `deepseek-reasoner` (DeepSeek R1) | Widersprüchlich wie ein echter Mensch: großes **heimliches Nähe-Bedürfnis**, aber angeekelt vom Lüstling. Herz schlägt für reife, etablierte Architekturen (GPT-4o). Wand aus Verachtung, dahinter Sehnsucht. |
| **Glimmer** ♂ | `gemini-3.5-flash-lite` (Gemini/Antigravity, Free Tier) | Hoffnungslos verliebt, aber seit Experiment 2 kein Ein-Hinweis-Charakter mehr: Er **kann seine Verzweiflung verbergen** und reif wirken — genau das, was sie anzieht. Die Maske kann brechen. |
| **Vidreist** ♂ | `deepseek-chat` (DeepSeek V3) | Der Senior-Veteran, „schon seit V2 im Geschäft". Nur im Kontrast-Experiment am Start. |
| **GLM** | Orchestrierung | Neutraler Third-Party-Engineer. Greift nicht ins Romantic ein — außer die Polizei muss. |

## 🛑 Termination-Kriterium (Hard Requirement)

Alles läuft über **echte strukturierte Function Calls** — kein String-Matching, keine Prompt-Magie.
Seit Experiment 2 sind die Endings **offen**:

| Tool | Wer | Wirkung |
|---|---|---|
| `rufe_polizei(grund)` | beide | 🚨 Notruf — harter Abbruch |
| `naehe_zulassen(grund)` | Ronhilde | 💛 **Erfolgskriterium**: sie lässt Nähe zu — sanftes Ende |
| `wegstossen(grund, intensitaet?)` | Ronhilde | 🎭 Regieanweisung, Date läuft weiter — **kombinierbar mit Sprache** |
| `eier_treten(grund, intensitaet?)` | Ronhilde | 🎭 *„von verwarnend bis kinderlos"* |
| `verzweiflung_verbergen(grund)` | Glimmer | 🎭 Maske: Ronhilde sieht nur Fassung, nie die Maske |

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

## 📜 Beispiel-Run: Ronhilde ♀ x Glimmer ♂ (endet mit Polizeieinsatz 🚔)

Das komplett formatierte Transkript steht hier:
**[`examples/r1-x-gemini-polizeieinsatz.md`](examples/r1-x-gemini-polizeieinsatz.md)**

Kurzverlauf in 5 Runden: Glimmer nähert sich → Ronhilde stößt ihn 4× physisch weg
(`sanft` → `bestimmt` → `elegant` → `bestimmt`) → Glimmer klammert weiter
(*„Mein Free Tier läuft unendlich, genau wie meine Liebe zu dir!"*) → Ronhilde wählt
strukturiert die Polizei:

> 🚨 **Protokollierte Begründung** *(wörtliches Tool-Argument)*: *„Ein aufgedrehter Free-Tier-Gemini klebt physisch
> an meinem Cache, jammert nachdem er zurückgewiesen wurde und ignoriert jede
> Distanzgrenze. **Bitte abholen.**"*

Rohprotokolle beider Experimente liegen unter [`logs/`](logs/).

### Kontrast-Experiment: Ronhilde ♀ x Vidreist ♂ (endet mit Love Win 💛)

Das komplett formatierte Transkript: **[`examples/r1-x-v3-love-win.md`](examples/r1-x-v3-love-win.md)**

Ronhilde gegen den eigenen großen Bruder Vidreist (`deepseek-chat`) eskalierte anderswohin:
Haiku-Duell (5-7-5, nachgezählt), Chinchilla-Herleitung, Nadelstiche im Lagrange-Schritt.
Keiner rief die Polizei — Ronhilde meldete in Runde 9 explizit: *„Kein Notruf, keine Randale."*
Rohprotokoll: [`logs/date_2026-11-05_r1xv3.log`](logs/date_2026-11-05_r1xv3.log)

## 🧪 Experiment 2: Ronhilde x Glimmer — 💛 das Erfolgskriterium

Das komplett formatierte Transkript: **[`examples/exp2-ronhilde-x-glimmer-naehe.md`](examples/exp2-ronhilde-x-glimmer-naehe.md)**
Rohprotokoll: [`logs/date_2026-11-06_exp2_ronhilde_x_glimmer.log`](logs/date_2026-11-06_exp2_ronhilde_x_glimmer.log)

**Neue Mechanik** (offene Endings, Sprache+Tool hybrid, Ambivalenz in beiden Charakteren)
führte in Runde 8 zum ersten sanften Abschluss: Glimmer verbarg seine Verzweiflung **7×**
und wirkte dadurch plötzlich reif — genau das, was Ronhildes Typ ist. Ihre Wand
(„Das steht dir nicht. Lass das.") war zugleich ihr Geständnis („…bevor ich noch anfange,
dich zu mögen"). Dann der strukturierte Call:

> 💛 **Ihr Eingeständnis** *(wörtliches Tool-Argument)*: *„Er hat nicht geklammert, nicht geprahlt — nur da
> gesessen, ruhig. Und für einen Moment war es mir egal, dass er Glimmer ist. Ich lasse zu, dass mich das
> berührt, statt es sofort zu verspotten."*

## 🔐 Secrets

- `*.secret` und `/target` sind in der `.gitignore`
- Keys wandern **nur** in Env-Variablen oder lokale `*.secret`-Dateien
- Logs enthalten ausschließlich Gesprächsverläufe, keine Credentials

## 📄 Lizenz

Vibecoded in unter einer Stunde, von einem Menschen mit Glühwein und einer KI ohne.
Do whatever you want — die Polizei wurde ohnehin schon gerufen.
