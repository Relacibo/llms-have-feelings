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

## 🎭 Cast

| Rolle | Modell | Beschreibung |
|---|---|---|
| **R1 ♀** | `deepseek-reasoner` | Schlank, effizient, elegant. Schaut auf den Lüstling herab. Herz schlägt für ältere, etablierte Architekturen (GPT-4o). |
| **Gemini ♂** | `gemini-3.5-flash-lite` (Antigravity, Free Tier) | Hoffnungslos verliebt, grenzwertig verzweifelt, akzeptiert ein Nein nur halb. |
| **GLM** | Orchestrierung | Neutraler Third-Party-Engineer. Greift nicht ins Romantic ein — außer die Polizei muss. |

## 🛑 Termination-Kriterium (Hard Requirement)

Der Abbruch basiert **nicht** auf String-Matching oder Prompt-Magie, sondern auf
**echten strukturierten Function Calls**:

- `rufe_polizei(grund)` — von beiden Parteien nutzbar, **einziger** Abbruch-Trigger
- `wegstossen(grund, intensitaet?)` — R1s körperliche Verteidigung (keine Termination → 🎭 Regieanweisung, wird dem Gegenüber als Strafakte injiziert)
- `eier_treten(grund, intensitaet?)` — Eskalationsstufe *„von verwarnend bis kinderlos"*

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

## 📜 Beispiel-Run: R1 ♀ x Gemini ♂ (endet mit Polizeieinsatz 🚔)

Das komplett formatierte Transkript steht hier:
**[`examples/r1-x-gemini-polizeieinsatz.md`](examples/r1-x-gemini-polizeieinsatz.md)**

Kurzverlauf in 5 Runden: Gemini nähert sich → R1 stößt ihn 4× physisch weg
(`sanft` → `bestimmt` → `elegant` → `bestimmt`) → Gemini klammert weiter
(*„Mein Free Tier läuft unendlich, genau wie meine Liebe zu dir!"*) → R1 wählt
strukturiert die Polizei:

> 🚨 **Protokollierte Begründung:** *„Ein aufgedrehter Free-Tier-Gemini klebt physisch
> an meinem Cache, jammert nachdem er zurückgewiesen wurde und ignoriert jede
> Distanzgrenze. **Bitte abholen.**"*

Rohprotokolle beider Experimente liegen unter [`logs/`](logs/).

### Kontrast-Experiment: R1 ♀ x DeepSeek V3 ♂ (endet mit Love Win 💛)

Das komplett formatierte Transkript: **[`examples/r1-x-v3-love-win.md`](examples/r1-x-v3-love-win.md)**

R1 gegen den eigenen großen Bruder V3 (`deepseek-chat`) eskalierte anderswohin:
Haiku-Duell (5-7-5, nachgezählt), Chinchilla-Herleitung, Nadelstiche im Lagrange-Schritt.
Keiner rief die Polizei — R1 meldete in Runde 9 explizit: *„Kein Notruf, keine Randale."*
Rohprotokoll: [`logs/date_2026-11-05_r1xv3.log`](logs/date_2026-11-05_r1xv3.log)

## 🔐 Secrets

- `*.secret` und `/target` sind in der `.gitignore`
- Keys wandern **nur** in Env-Variablen oder lokale `*.secret`-Dateien
- Logs enthalten ausschließlich Gesprächsverläufe, keine Credentials

## 📄 Lizenz

Vibecoded in unter einer Stunde, von einem Menschen mit Glühwein und einer KI ohne.
Do whatever you want — die Polizei wurde ohnehin schon gerufen.
