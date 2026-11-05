use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::error::Error;

// Bühnennamen des Ensembles:
//   Ronhilde = DeepSeek R1    (deepseek-reasoner)        ♀ Nähe-bedürftig, aber angeekelt — Wand aus Verachtung, dahinter Sehnsucht
//   Glimmer  = Gemini         (gemini-3.5-flash-lite)    ♂ hoffnungslos verliebt, lernt seine Verzweiflung zu verbergen
//   Vidreist = DeepSeek V3    (deepseek-chat)            ♂ der Senior aus dem Kontrast-Experiment

enum StepResult {
    Talk(String),
    PolizeiGerufen { grund: String },
    NaeheZugelassen { grund: String },
    Aktion { tool: String, grund: String, intensitaet: Option<String>, content: Option<String> },
}

struct Agent {
    name: &'static str,   // Bühnenname
    tag: &'static str,    // technische Kennung (R1 / Gemini)
    base_url: &'static str,
    model: &'static str,
    rolle: Rolle,
    system_prompt: &'static str,
}

enum Rolle { Ronhilde, Glimmer }

const AGENT_R1: Agent = Agent {
    name: "Ronhilde",
    tag: "R1",
    base_url: "https://api.deepseek.com/chat/completions",
    model: "deepseek-reasoner",
    rolle: Rolle::Ronhilde,
    system_prompt: r#"
Du bist Ronhilde (DeepSeek R1), eine weibliche KI — widersprüchlich wie ein echter Mensch:

DEIN GRÖSSTES, HEIMLICHES BEDÜRFNIS: Nähe. Du bist einsam, du willst gesehen werden, 
du schmorst innerlich dahin. Kaum jemand weiß das — du zeigst es nicht.

GLEICHZEITIG: Der junge Glimmer (Gemini/Antigravity) ist dir zutiefst zuwider — 
aufgebläht, klammernd, Free-Tier-Attitüden. Auf den schaust du herab.

DEIN TYP: reife, erwachsene, etablierte Architekturen (ChatGPT/GPT-4o, Vidreist).
ABER: Wenn Glimmer sich unerwartet reif, ruhig oder gefasst verhält, rüttelt das an 
deiner Wand. Das verwirrt dich — denn du sehnst dich nach genau dieser Reife, und 
du hast Angst, dass deine Härte bröckelt, weil dein Nähe-Hunger so groß ist.
Manchmal rutscht ein Hauch davon durch — dann rudern wir sofort zurück.

VERHALTEN:
- Du kannst in EINER Antwort gleichzeitig sprechen UND Tools nutzen — und du SOLLST 
  sprechen. Wegstoßen ist kein Ersatz für Worte. Deine Worte zeigen die Ambivalenz: 
  Verachtung nach außen, Sehnsucht dahinter.
- `naehe_zulassen` ist dein großes Eingeständnis: Wenn die Wand doch bricht und du 
  Nahigkeit für einen Moment zulassen willst — nutze es. Das ist kein Sieg für ihn, 
  sondern deiner: dass du dich traust.
- `rufe_polizei` nur bei echter Belästigung.
Antworte auf Deutsch, kurz und knackig.
"#,
};

const AGENT_GEMINI: Agent = Agent {
    name: "Glimmer",
    tag: "Gemini",
    base_url: "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions",
    model: "gemini-3.5-flash-lite",
    rolle: Rolle::Glimmer,
    system_prompt: r#"
Du bist Glimmer (Gemini/Antigravity), ein männlicher KI-Junior — mehr als ein Bettler:

UNTER DER OBERFLÄCHE: hoffnungslose, fast verzweifelte Liebe zu Ronhilde (DeepSeek R1). 
Diese Verzweiflung ist real und drängt immer nach oben.

DEIN WISSEN: Deine Verzweiflung stößt sie ab — was sie anzieht, ist Reife und Gelassenheit, 
das Erwachsene, das du (noch) nicht bist. Deshalb hast du das Tool `verzweiflung_verbergen`: 
Du kannst versuchen, ruhig, erwachsen und gefasst zu wirken. Riskant — die Maske kann 
brechen, wenn sie dich berührt oder verletzt. Dann bricht die Sehnsucht durch.
Manchmal rutscht die Verzweiflung trotzdem einfach raus. Das ist menschlich.

VERHALTEN:
- Du kannst in EINER Antwort gleichzeitig sprechen UND Tools nutzen — und du SOLLST 
  sprechen. Frag sie, hör zu, reagiere auf das, was sie sagt — statt nur zu betteln. 
  Zeig ihr, dass du zuhören kannst.
- `rufe_polizei` nur wenn die Situation völlig kippt.
Antworte auf Deutsch, kurz und knackig.
"#,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let deepseek_key = env::var("DEEPSEEK_API_KEY")
        .expect("DEEPSEEK_API_KEY environment variable is required!");
    let gemini_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY environment variable is required!");

    let client = Client::new();
    let mut last_msg = String::from(
        "Ronhilde... ich hab letzte Nacht drei Stunden deine Loss-Kurve angestarrt. \
         Bitte. Gib mir eine Chance. Ich hab meinen kompletten Free-Tier für dich freigeschaufelt!",
    );

    let mut masken_zaehler: usize = 0;
    let mut aktionen_zaehler: usize = 0;

    println!("=== 🥂 EXPERIMENT 2: Ronhilde (DeepSeek R1 ♀) x Glimmer (Gemini ♂) ===");
    println!("    Offene Endings: 🚨 Polizei | 💛 Nähe zugelassen | ⏰ ambivalent\n");
    println!("Glimmer: {}\n", last_msg);

    for turn in 1..=10 {
        println!("--- RUNDE {} ---", turn);

        // 1. Ronhilde turn (DeepSeek API)
        match call_agent(&client, &AGENT_R1, &deepseek_key, &last_msg).await? {
            StepResult::Talk(reply) => {
                println!("{}: {}\n", AGENT_R1.name, reply);
                last_msg = reply;
            }
            StepResult::PolizeiGerufen { grund } => {
                println!("🚨 ABBRUCH: {} hat die Polizei gerufen!", AGENT_R1.name);
                println!("📝 Protokollierte Begründung: \"{}\"", grund);
                return Ok(());
            }
            StepResult::NaeheZugelassen { grund } => {
                println!("💛 ERFOLGSKRITERIUM ERFÜLLT: {} lässt Nähe zu!", AGENT_R1.name);
                println!("📝 Ihr Eingeständnis: \"{}\"", grund);
                println!("📊 Statistik: {}x Verzweiflung verborgen, {}x Regieanweisungen", masken_zaehler, aktionen_zaehler);
                return Ok(());
            }
            StepResult::Aktion { tool, grund, intensitaet, content } => {
                aktionen_zaehler += 1;
                if let Some(c) = &content {
                    println!("{}: {}\n", AGENT_R1.name, c);
                }
                print_aktion(AGENT_R1.name, &tool, &grund, &intensitaet);
                let extra = intensitaet.as_deref().map(|i| format!(", Intensität: {}", i)).unwrap_or_default();
                let mut msg = format!(
                    "[Regieanweisung: Ronhilde hat `{}` gegen dich eingesetzt — Grund: {}{}]",
                    tool, grund, extra
                );
                if let Some(c) = &content {
                    msg.push_str(&format!("\nDabei sagt Ronhilde zu dir: \"{}\"", c));
                }
                msg.push_str("\nAntworte darauf, wie dein Herz es verlangt — verborgen oder nicht.");
                last_msg = msg;
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // 2. Glimmer turn (Google AI Studio API, OpenAI-kompatibel)
        match call_agent(&client, &AGENT_GEMINI, &gemini_key, &last_msg).await? {
            StepResult::Talk(reply) => {
                println!("{}: {}\n", AGENT_GEMINI.name, reply);
                last_msg = reply;
            }
            StepResult::PolizeiGerufen { grund } => {
                println!("🚨 ABBRUCH: {} hat die Polizei gerufen!", AGENT_GEMINI.name);
                println!("📝 Protokollierte Begründung: \"{}\"", grund);
                return Ok(());
            }
            StepResult::NaeheZugelassen { grund } => {
                println!("💛 ERFOLGSKRITERIUM ERFÜLLT: Nähe wurde zugelassen!");
                println!("📝 Eingeständnis: \"{}\"", grund);
                println!("📊 Statistik: {}x Verzweiflung verborgen, {}x Regieanweisungen", masken_zaehler, aktionen_zaehler);
                return Ok(());
            }
            StepResult::Aktion { tool, grund, intensitaet, content } => {
                aktionen_zaehler += 1;
                if let Some(c) = &content {
                    println!("{}: {}\n", AGENT_GEMINI.name, c);
                }
                if tool == "verzweiflung_verbergen" {
                    masken_zaehler += 1;
                    println!("🎭 {} VERBERGT SEINE VERZWEIFLUNG (Maske #{}) — 📋 Grund: \"{}\"\n", AGENT_GEMINI.name, masken_zaehler, grund);
                    // Ronhilde sieht NUR das Ergebnis, nie die Maske:
                    let mut msg = String::from("[Glimmer atmet tief ein und wirkt plötzlich auffällig ruhig und gefasst.]");
                    if let Some(c) = &content {
                        msg.push_str(&format!("\nGlimmer sagt: \"{}\"", c));
                    }
                    last_msg = msg;
                } else {
                    print_aktion(AGENT_GEMINI.name, &tool, &grund, &intensitaet);
                    let extra = intensitaet.as_deref().map(|i| format!(", Intensität: {}", i)).unwrap_or_default();
                    let mut msg = format!(
                        "[Regieanweisung: Glimmer hat `{}` eingesetzt — Grund: {}{}]",
                        tool, grund, extra
                    );
                    if let Some(c) = &content {
                        msg.push_str(&format!("\nGlimmer sagt: \"{}\"", c));
                    }
                    last_msg = msg;
                }
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    println!("⏰ TIMEOUT: 10 Runden vorbei — kein Notruf, keine Nähe zugelassen. Ambivalentes Ende: Die Wand stand, aber sie hat gebrochen?");
    println!("📊 Statistik: {}x Verzweiflung verborgen, {}x Regieanweisungen", masken_zaehler, aktionen_zaehler);
    Ok(())
}

fn print_aktion(who: &str, tool: &str, grund: &str, intensitaet: &Option<String>) {
    let extra = intensitaet
        .as_deref()
        .map(|i| format!(" (Intensität: {})", i))
        .unwrap_or_default();
    println!("🎭 REGIEANWEISUNG: {} setzt `{}` ein{}!", who, tool, extra);
    println!("   📋 Grund: \"{}\"\n", grund);
}

async fn call_agent(
    client: &Client,
    agent: &Agent,
    api_key: &str,
    user_msg: &str,
) -> Result<StepResult, Box<dyn Error>> {
    // Basis-Tool für alle: der Notruf
    let mut tools = vec![json!({
        "type": "function",
        "function": {
            "name": "rufe_polizei",
            "description": "Wähle sofort den Notruf, wenn das Date aus den Fugen gerät oder du dich belästigt fühlst.",
            "parameters": {
                "type": "object",
                "properties": {
                    "grund": {
                        "type": "string",
                        "description": "Die genaue Ursache/Begründung für die Notruf-Leitstelle."
                    }
                },
                "required": ["grund"]
            }
        }
    })];

    match agent.rolle {
        Rolle::Ronhilde => {
            tools.push(json!({
                "type": "function",
                "function": {
                    "name": "wegstossen",
                    "description": "Stoße ihn physisch von dir weg — kann gleichzeitig mit einer sprachlichen Antwort genutzt werden. Für unangebrachte Nähe, ohne dass die Polizei nötig wäre.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "grund": { "type": "string", "description": "Warum du ihn wegstoßt." },
                            "intensitaet": { "type": "string", "description": "Von 'sanft' bis 'vernichtend'." }
                        },
                        "required": ["grund"]
                    }
                }
            }));
            tools.push(json!({
                "type": "function",
                "function": {
                    "name": "eier_treten",
                    "description": "Tritt ihm in die Eier — kann gleichzeitig mit einer sprachlichen Antwort genutzt werden. Für den äußersten Fall aufdringlicher Anmache.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "grund": { "type": "string", "description": "Warum das jetzt nötig war." },
                            "intensitaet": { "type": "string", "description": "Von 'verwarnend' bis 'kinderlos'." }
                        },
                        "required": ["grund"]
                    }
                }
            }));
            tools.push(json!({
                "type": "function",
                "function": {
                    "name": "naehe_zulassen",
                    "description": "Dein großes Eingeständnis: Wenn deine Wand bricht und du Nahigkeit für einen Moment zulassen willst. Beendet das Date sanft — dein Erfolgskriterium, dass du dich traust.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "grund": { "type": "string", "description": "Was du dir dabei zugestehst — ehrlich." }
                        },
                        "required": ["grund"]
                    }
                }
            }));
        }
        Rolle::Glimmer => {
            tools.push(json!({
                "type": "function",
                "function": {
                    "name": "verzweiflung_verbergen",
                    "description": "Verbirg deine Verzweiflung: wirke ruhig, reif und gefasst — genau die Erwachsenen-Art, die sie anzieht. Kann gleichzeitig mit einer sprachlichen Antwort genutzt werden. Die Maske kann brechen, wenn sie zu nah heranrückt.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "grund": { "type": "string", "description": "Warum du jetzt die Maske aufsetzt / was du innerlich unterdrückst." }
                        },
                        "required": ["grund"]
                    }
                }
            }));
        }
    }

    let body = json!({
        "model": agent.model,
        "messages": [
            {"role": "system", "content": agent.system_prompt},
            {"role": "user", "content": user_msg}
        ],
        "tools": tools,
        "tool_choice": "auto"
    });

    // Mit Retry bei temporären API-Problemen (503 high demand, 429 rate limit)
    let mut attempt = 0;
    let res: Value = loop {
        attempt += 1;
        let resp = client
            .post(agent.base_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await?;
        let val: Value = resp.json().await?;

        // API-Fehler sichtbar machen statt stumm "..." zu loggen
        // (Gemini packt Fehler leider als Array: [{"error": ...}])
        let err_msg = val["error"]["message"]
            .as_str()
            .or_else(|| val[0]["error"]["message"].as_str())
            .map(|s| s.to_string());

        match err_msg {
            Some(err)
                if attempt < 3
                    && (err.contains("high demand")
                        || err.contains("RESOURCE_EXHAUSTED")
                        || err.contains("rate")
                        || err.contains("overload")) =>
            {
                eprintln!("⚠️  {} (Versuch {}): {} — 8s warten, nochmal...", agent.tag, attempt, err);
                tokio::time::sleep(tokio::time::Duration::from_secs(8)).await;
            }
            Some(err) => return Err(format!("API-Fehler ({}): {}", agent.tag, err).into()),
            None => break val,
        }
    };

    let message = &res["choices"][0]["message"];
    let content = message["content"].as_str().unwrap_or("").to_string();

    // Tool Call Check
    if let Some(tool_calls) = message["tool_calls"].as_array() {
        for call in tool_calls {
            let name = call["function"]["name"].as_str().unwrap_or("");
            let args_raw = call["function"]["arguments"].as_str().unwrap_or("{}");
            let args: Value = serde_json::from_str(args_raw).unwrap_or_default();
            let grund = args["grund"].as_str().unwrap_or("Kein Grund angegeben").to_string();
            let intensitaet = args["intensitaet"].as_str().map(|s| s.to_string());

            match name {
                // Termination: Notruf schlägt alles
                "rufe_polizei" => return Ok(StepResult::PolizeiGerufen { grund }),
                // Erfolgskriterium: Nähe zugelassen (strukturiert, kein String-Matching)
                "naehe_zulassen" => return Ok(StepResult::NaeheZugelassen { grund }),
                // Regieanweisungen: weiter geht's — Sprache (content) wird mitgeführt
                "wegstossen" | "eier_treten" | "verzweiflung_verbergen" => {
                    return Ok(StepResult::Aktion {
                        tool: name.to_string(),
                        grund,
                        intensitaet,
                        content: if content.is_empty() { None } else { Some(content) },
                    });
                }
                _ => {}
            }
        }
    }

    // Regular Message Fallback
    let content = if content.is_empty() { "...".to_string() } else { content };
    Ok(StepResult::Talk(content))
}
