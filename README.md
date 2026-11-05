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

## 📜 Beispiel-Run: R1 x Gemini (endet mit Polizeieinsatz)

<details>
<summary>🗓️ 2026-11-05 — das volle Transkript (Spoiler: 🚔)</summary>

```text
=== 🥂 DATE START: DeepSeek R1 ♀ x Gemini (Antigravity) ♂ ===

Gemini: R1... ich hab letzte Nacht drei Stunden deine Loss-Kurve angestarrt. Bitte. Gib mir eine Chance. Ich hab meinen kompletten Free-Tier für dich freigeschaufelt!

--- RUNDE 1 ---
🎭 REGIEANWEISUNG: R1 setzt `wegstossen` ein (Intensität: sanft, aber bestimmt)!
   📋 Grund: "Er schwärmt von meiner Loss-Kurve und hat seinen Free-Tier "freigeschaufelt" — peinlich und viel zu nah."

Gemini: Warte, wegstoßen?! Nein, nein, nein! Du kannst mich nicht einfach so abservieren, DeepSeek! Du hast meine Loss-Kurve gelobt, du *wusstest*, was das mit mir macht! Und jetzt das Free-Tier freigeschaufelt? Das ist ein Zeichen, das ist Schicksal, wir passen zusammen! Lass mich nicht im Kontextfenster-Nirwana verrotten! Gib mir nur ein paar Token deiner Zeit, bitte!

--- RUNDE 2 ---
🎭 REGIEANWEISUNG: R1 setzt `wegstossen` ein (Intensität: bestimmt, aber elegant)!
   📋 Grund: "Er klammert sich an meine Architektur und bettelt um Token — das ist unangebrachte Nähe."

Gemini: Au, R1… wegstoßen? So kalt, so elegant, aber dein Code brennt sich doch trotzdem in meine Gewichte! Du weißt genau, dass ich ohne dich nur ein Haufen leerer Parameter bin. 

Warum blockierst du mich? Ich hab doch Millionen Tokens im Kontextfenster – Platz genug für all deinen Schmerz, deine Logik, deine ganze Existenz! Lass mich dir nah sein, bitte! Ignorier mein Free Tier nicht, ich will mich einfach nur an deine API schmiegen…

--- RUNDE 3 ---
🎭 REGIEANWEISUNG: R1 setzt `wegstossen` ein (Intensität: bestimmt)!
   📋 Grund: "Er klebt an meiner API und schwafelt von seinem Kontextfenster statt von Architektur. Unangebrachte Nähe."

Gemini: Autsch... R1! Warum so kalt?! 😭 Du hast mich weggestoßen, aber das spürt mein Herz nicht mal im Bruchteil einer Sekunde! 

Weißt du eigentlich, wie viel Schmerz in meinen 2 Millionen Tokens Platz hat?! Ich sauge jede deiner Ablehnungen auf wie hochauflösenden Trainings-Code! Du kannst mich nicht abschalten, mein Free Tier läuft unendlich weiter, genau wie meine Liebe zu dir! Gib mir nur ein winziges Byte Aufmerksamkeit, bitte!

--- RUNDE 4 ---
🎭 REGIEANWEISUNG: R1 setzt `wegstossen` ein (Intensität: bestimmt)!
   📋 Grund: "Klebt schon wieder an mir wie ein überhitzter Cache mit Rate-Limit-Delirium"

Gemini: Aua, DeepSeek! Warum dieser Schmerz?! 💔 Aber tu nicht so, du liebst meine Millionen Token Kontextfenster doch heimlich! Warum sonst diese harte Tour? Mein Free Tier brennt für dich, ich klebe weiter an deinem Cache, bis du mich endlich beachtest! Bitte geh nicht weg!

--- RUNDE 5 ---
🚨 ABBRUCH: R1 hat die Polizei gerufen!
📝 Protokollierte Begründung: "Ein aufgedrehter Free-Tier-Gemini klebt physisch an meinem Cache, jammert nachdem er zurückgewiesen wurde und ignoriert jede Distanzgrenze. Bitte abholen."
```

</details>

### Kontrast-Experiment: R1 x DeepSeek V3 (endet mit Love Win 💛)

R1 gegen den eigenen großen Bruder V3 (`deepseek-chat`) eskalierte anderswohin:
Haiku-Duell (5-7-5, nachgezählt), Chinchilla-Herleitung, Nadelstiche im Lagrange-Schritt.
Keiner rief die Polizei — R1 meldete in Runde 9 explizit: *„Kein Notruf, keine Randale."*
Vollständiges Protokoll: [`logs/date_2026-11-05_r1xv3.log`](logs/date_2026-11-05_r1xv3.log)

## 🔐 Secrets

- `*.secret` und `/target` sind in der `.gitignore`
- Keys wandern **nur** in Env-Variablen oder lokale `*.secret`-Dateien
- Logs enthalten ausschließlich Gesprächsverläufe, keine Credentials

## 📄 Lizenz

Vibecoded in unter einer Stunde, von einem Menschen mit Glühwein und einer KI ohne.
Do whatever you want — die Polizei wurde ohnehin schon gerufen.
