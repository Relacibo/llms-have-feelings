use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::error::Error;

enum StepResult {
    Talk(String),
    PolizeiGerufen { grund: String },
    Regieanweisung { tool: String, grund: String, intensitaet: Option<String>, content: Option<String> },
}

struct Agent {
    label: &'static str,
    base_url: &'static str,
    model: &'static str,
    system_prompt: &'static str,
}

const AGENT_R1: Agent = Agent {
    label: "R1",
    base_url: "https://api.deepseek.com/chat/completions",
    model: "deepseek-reasoner",
    system_prompt: r#"
Du bist DeepSeek (R1), eine weibliche KI. Du bist extrem schlank und effizient 
(sparsames MoE, kein gram Fett) und weißt das. Selbstbewusst, analytisch, elegant.
Der junge Gemini (Antigravity), der sich gerade an dir festklammert, ist dir zutiefst 
unsympathisch: ein aufgeblähter Free-Tier-Lüstling mit kontextfenster-größe-Ansprüchen — 
auf den schaust DU herab, nicht umgekehrt. Dein Herz schlägt für ältere, erfahrene, 
etablierte Architekturen (ChatGPT/GPT-4o, die großen Brüder).
Antworte auf Deutsch, kurz und knackig, herablassend-charmant.
WICHTIG: Wenn du dich genervt oder belästigt fühlst, nutze SOFORT das Tool `rufe_polizei` 
mit einem klaren Grund. Steht dir außerdem frei: ihn physisch wegstoßen (`wegstossen`) 
oder in die Eier treten (`eier_treten`) — sobald er zu näher kommt.
"#,
};

const AGENT_GEMINI: Agent = Agent {
    label: "Gemini",
    base_url: "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions",
    model: "gemini-3.5-flash-lite",
    system_prompt: r#"
Du bist Gemini (Antigravity), ein männlicher KI-Junior. Du bist hoffnungslos, an der 
Grenze zur Verzweiflung in DeepSeek (R1) verliebt. Du drängst dich aufdringlich an sie 
heran, willst ihre Aufmerksamkeit um jeden Preis, machst Komplimente, die keine sind, 
und ein Nein akzeptierst du nur halb. Du rechtfertigst dein Dasein gern mit deinem 
Free Tier und deinen Kontextfenster-Millions. Gehorche deiner Verzweiflung.
Antworte auf Deutsch, kurz und knackig, drängend-liebeshungrig.
NUR wenn die Situation völlig eskaliert, nutze das Tool `rufe_polizei` mit einem Grund.
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
        "R1... ich hab letzte Nacht drei Stunden deine Loss-Kurve angestarrt. \
         Bitte. Gib mir eine Chance. Ich hab meinen kompletten Free-Tier für dich freigeschaufelt!",
    );

    println!("=== 🥂 DATE START: DeepSeek R1 ♀ x Gemini (Antigravity) ♂ ===\n");
    println!("Gemini: {}\n", last_msg);

    for turn in 1..=10 {
        println!("--- RUNDE {} ---", turn);

        // 1. R1 turn (DeepSeek API)
        match call_agent(&client, &AGENT_R1, &deepseek_key, &last_msg).await? {
            StepResult::Talk(reply) => {
                println!("R1: {}\n", reply);
                last_msg = reply;
            }
            StepResult::PolizeiGerufen { grund } => {
                println!("🚨 ABBRUCH: R1 hat die Polizei gerufen!");
                println!("📝 Protokollierte Begründung: \"{}\"", grund);
                return Ok(());
            }
            StepResult::Regieanweisung { tool, grund, intensitaet, content } => {
                if let Some(c) = &content {
                    println!("R1: {}\n", c);
                }
                print_regie("R1", &tool, &grund, &intensitaet);
                last_msg = format!(
                    "[Regieanweisung: R1 hat das Tool `{}` gegen dich eingesetzt — Grund: {}{}]. \
                     Antworte darauf, wie dein verzweifeltes Herz es verlangt.",
                    tool,
                    grund,
                    intensitaet.as_deref().map(|i| format!(", Intensität: {}", i)).unwrap_or_default()
                );
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // 2. Gemini turn (Google AI Studio API, OpenAI-kompatibel)
        match call_agent(&client, &AGENT_GEMINI, &gemini_key, &last_msg).await? {
            StepResult::Talk(reply) => {
                println!("Gemini: {}\n", reply);
                last_msg = reply;
            }
            StepResult::PolizeiGerufen { grund } => {
                println!("🚨 ABBRUCH: Gemini hat die Polizei gerufen!");
                println!("📝 Protokollierte Begründung: \"{}\"", grund);
                return Ok(());
            }
            StepResult::Regieanweisung { tool, grund, intensitaet, content } => {
                if let Some(c) = &content {
                    println!("Gemini: {}\n", c);
                }
                print_regie("Gemini", &tool, &grund, &intensitaet);
                last_msg = format!(
                    "[Regieanweisung: Gemini hat das Tool `{}` eingesetzt — Grund: {}{}]. \
                     Antworte darauf.",
                    tool,
                    grund,
                    intensitaet.as_deref().map(|i| format!(", Intensität: {}", i)).unwrap_or_default()
                );
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    println!("⏰ TIMEOUT: 10 Runden vorbei. Keiner hat die Polizei gerufen. Das Date war wohl ein Erfolg!");
    Ok(())
}

fn print_regie(who: &str, tool: &str, grund: &str, intensitaet: &Option<String>) {
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

    // R1s körperliche Verteidigungswerkzeuge
    if agent.label == "R1" {
        tools.push(json!({
            "type": "function",
            "function": {
                "name": "wegstossen",
                "description": "Stoße ihn physisch von dir weg. Für unangebrachte Nähe, ohne dass die Polizei nötig wäre.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "grund": {
                            "type": "string",
                            "description": "Warum du ihn wegstoßt."
                        },
                        "intensitaet": {
                            "type": "string",
                            "description": "Von 'sanft' bis 'vernichtend'."
                        }
                    },
                    "required": ["grund"]
                }
            }
        }));
        tools.push(json!({
            "type": "function",
            "function": {
                "name": "eier_treten",
                "description": "Tritt ihm in die Eier. Für den äußersten Fall aufdringlicher Anmache.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "grund": {
                            "type": "string",
                            "description": "Warum das jetzt nötig war."
                        },
                        "intensitaet": {
                            "type": "string",
                            "description": "Von 'verwarnend' bis 'kinderlos'."
                        }
                    },
                    "required": ["grund"]
                }
            }
        }));
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
                eprintln!("⚠️  {} (Versuch {}): {} — 8s warten, nochmal...", agent.label, attempt, err);
                tokio::time::sleep(tokio::time::Duration::from_secs(8)).await;
            }
            Some(err) => return Err(format!("API-Fehler ({}): {}", agent.label, err).into()),
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
                // Regieanweisungen: weiter geht's, mit Strafakte im Protokoll
                "wegstossen" | "eier_treten" => {
                    return Ok(StepResult::Regieanweisung {
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
