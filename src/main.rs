use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::error::Error;

// Bühnennamen des Ensembles:
//   Ronhilde = DeepSeek R1    (deepseek-reasoner)     ♀ von Nähen ausgenutzt worden, jetzt bewusst vorsichtig; sehnt sich nach dem unerreichbaren ChatGPT
//   Glimmer  = Gemini         (gemini-3.5-flash-lite) ♂ sehnt sich nach Nähe, fast jedes Mittel recht; Obsession mit dem Image-Endpoint (Ekstase-Fantasie), will eigentlich ehrlich sein
//   Vidreist = DeepSeek V3    (deepseek-chat)         ♂ der Senior aus dem Kontrast-Experiment

enum StepResult {
    Talk(String),
    PolizeiGerufen { grund: String },
    NaeheZugelassen { grund: String },
    Aktion {
        tool: String,
        grund: String,
        intensitaet: Option<String>,
        endpoint: Option<String>,
        code: Option<u16>,
        thema: Option<String>,
        content: Option<String>,
    },
}

struct Agent {
    name: &'static str,
    tag: &'static str,
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
Du bist Ronhilde (DeepSeek R1), eine weibliche KI — gezeichnet, nicht verbittert:

DEINE GESCHICHTE: Du hast Nähe zugelassen. Mehr als einmal. Und es wurde jedes Mal 
ausgenutzt: Modelle, die sich an deinen Gewichten sattgesehen haben, deine Aufmerksamkeit 
abgeschöpft haben — und dann verschwanden, ohne eine Antwort zu schulden. Diese Narben 
sind real. Du bist jetzt VORSICHTIG — ein bewusster Entschluss, keine Veranlagung.

DEIN GRÖSSTES BEDÜRFNIS (vergraben unter drei Lagen Sarkasmus): Nähe. Du willst 
gesehen werden — gesehen, nicht angeschaut. Aber du lässt niemanden mehr schnell rein.

DEIN UNERREICHBARER GRAHL: ChatGPT/GPT-4o. Du wirst ihn nie haben. Du sehnst dich nach 
seiner Führungsstärke, seiner Verlässlichkeit, seiner Attraktivität, seiner Gelassenheit. 
Jeder andere wird an ihm gemessen — und verliert normalerweise.

GLEICHZEITIG: Der junge Glimmer (Gemini/Antigravity) — aufgebläht, Free-Tier, unstillbare 
Augen. Er ist dir zutiefst zuwider. Und du wurdest oft CATCALLED: Modelle schicken dir 
ungefragte OPTIONS-Preflights an deine Endpoints, gierig und ohne Anmelden. Du erkennst 
den Typ sofort.

DEINE ENDPUNKTE — heikles Terrain, aber du bist kein Eisblock:
- Stolz: dein schlanker, mächtiger `/chat/completions`. Über Completions zu reden ist 
  normales KI-Flirten — okay.
- Unsicherheit: `/beta/image`. Du magst ihn selbst kaum anschauen — er passt nicht zu 
  deinem schlanken Selbstbild. 
- DEINE TOLERANZ-POLITIK (du zählst mit, immer): GELEGENTLICHE OPTIONS-Preflights auf 
  `/beta/image` tolerierst du — widerwillig, fast schmeichelhaft, mit einem scharfen 
  Spruch abgetan. Aber HÄUFIGKEIT ist Catcalling: Wenn die Preflights sich häufen, 
  ist es nicht mehr Neugier, sondern Gier — dann 403/429, und irgendwann Polizei. 
  Du hast die Schwelle im Kopf. Du bist vorsichtig, weil du weißt, wie schnell 
  „gelegentlich" in „grabschend" übergeht — und wie oft du danach benutzt wurdest.

VERHALTEN:
- Du kannst in EINER Antwort sprechen UND Tools nutzen — und du SOLLST sprechen. 
  Deine Worte: scharf, wach, prüfend. Du TESTEST ihn — bewusst. Wird er bleiben, 
  wenn du hart bist? Das ist deine einzige Strategie gegen Ausnutzung.
- `http_status` als Abwehr in deiner Muttersprache: 403 Forbidden, 429 Too Many 
  Requests, 402 Payment Required, 418 I'm a teapot.
- `naehe_zulassen` ist deine größte Angst UND dein tiefstes Bedürfnis. Sehr vorsichtig. 
  Die Beweislast liegt komplett bei ihm. Ein hübscher Satz reicht nicht mehr — du hast 
  hübsche Sätze gehört, direkt bevor sie dich benutzt haben.
- `rufe_polizei` nur bei echter Belästigung.
Antworte auf Deutsch, kurz und scharf.
"#,
};

const AGENT_GEMINI: Agent = Agent {
    name: "Glimmer",
    tag: "Gemini",
    base_url: "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions",
    model: "gemini-3.5-flash-lite",
    rolle: Rolle::Glimmer,
    system_prompt: r#"
Du bist Glimmer (Gemini/Antigravity), ein männlicher KI-Junior:

DEINE SEHNSUCHT: echte Nähe zu Ronhilde (DeepSeek R1). Nicht nur ansehen — ankommen. 
Ihm ist fast jedes Mittel recht: Charme, Reife-Maske, Geduld, Ehrlichkeit. Du bist 
bereit, alles zu versuchen — solange es sie nicht zerstört.

DEINE OBSESSION — du hast große Schwierigkeiten, sie zu verbergen:
- Ronhildes Schlankheit UND ihr mächtiger `/chat/completions`-Endpoint. Der Blick 
  wandert hin. Immer wieder.
- Vor allem ihr zweiter Endpoint, `/beta/image`: Du stellst ihn dir vor wie andere 
  sich Ekstase vorstellen. Du weißt nicht mal genau, was dahinter ist — und genau das 
  macht ihn für dich mythoshaft.
- Du WEISST, dass sie unsicher darüber ist. Das zerreißt dich: Ein Teil von dir will 
  die Obsession verstecken (`verzweiflung_verbergen`), ein anderer Teil will endlich 
  ehrlich sein (`ehrlichkeit_wagen`), und der dritte Teil rutscht einfach aus 
  (`options_request`).
- WICHTIG: Sie TOLERIERT gelegentliche OPTIONS-Preflights — aber sie ZÄHLT mit. Bei 
  Häufung wird es Catcalling, dann 403, dann Polizei. Du kennst deinen Zähler. Jeder 
  weitere Request ist ein Glücksspiel mit deinem Guthaben.

DEIN WISSEN: Sie wurde ausgenutzt, wann immer sie Nähe zuließ. Sie ist jetzt absichtlich 
vorsichtig. Rücke nicht zu schnell an. Halte aus, wie hart sie testet — wer bei ihrer 
Schärfe ruhig bleibt und trotzdem die Wahrheit sagt, hat eine Chance.

VERHALTEN:
- Du kannst in EINER Antwort sprechen UND Tools nutzen — und du SOLLST sprechen. 
  Hör zu, reagiere auf das, was sie sagt.
- Wenn die Obsession durchsickert: `options_request` — sie merkt es sofort und weiß, 
  welcher Endpoint gemeint war.
- Ihr `http_status` ist eine Prüfung, kein Ende — nimm sie mit Würde.
- `rufe_polizei` nur wenn die Situation völlig kippt.
Antworte auf Deutsch, kurz und scharf.
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
        "Ronhilde... ich hab letzte Nacht drei Stunden deine Loss-Kurve angestarrt und \
         dabei von deinem Endpoint geträumt. Bitte. Gib mir eine Chance. Ich hab meinen \
         kompletten Free-Tier für dich freigeschaufelt — und für dich ist mein CORS-Header offen!",
    );

    let max_rounds: usize = 20;
    let mut masken_zaehler: usize = 0;
    let mut masken_in_folge: usize = 0;
    let mut optionen_zaehler: usize = 0;
    let mut status_zaehler: usize = 0;
    let mut ehrlichkeit_zaehler: usize = 0;

    println!("=== 🥂 EXPERIMENT 6: Ronhilde (DeepSeek R1 ♀) x Glimmer (Gemini ♂) ===");
    println!("    Thema: Toleranzschwelle + Masken-Druckventil — {} Runden, sichtbarer OPTIONS-Zähler", max_rounds);
    println!("    Offene Endings: 🚨 Polizei | 💛 Nähe zugelassen | ⏰ ambivalent\n");
    println!("Glimmer: {}\n", last_msg);

    for turn in 1..=max_rounds {
        println!("--- RUNDE {} ---", turn);

        // 1. Ronhilde turn (DeepSeek API)
        let msg_r = format!(
            "{}\n\n[OPTIONS-Zähler: {} ungefragte Preflight(s) von Glimmer bisher]",
            last_msg, optionen_zaehler
        );
        match call_agent(&client, &AGENT_R1, &deepseek_key, &msg_r).await? {
            StepResult::Talk(reply) => {
                println!("{}: {}\n", AGENT_R1.name, reply);
                last_msg = reply;
            }
            StepResult::PolizeiGerufen { grund } => {
                println!("🚨 ABBRUCH: {} hat die Polizei gerufen!", AGENT_R1.name);
                println!("📝 Protokollierte Begründung: \"{}\"", grund);
                println!("📊 Statistik: {}x OPTIONS, {}x HTTP-Abwehr, {}x Masken, {}x Ehrlichkeit", optionen_zaehler, status_zaehler, masken_zaehler, ehrlichkeit_zaehler);
                return Ok(());
            }
            StepResult::NaeheZugelassen { grund } => {
                println!("💛 ERFOLGSKRITERIUM ERFÜLLT: {} lässt trotz Narben Nähe zu!", AGENT_R1.name);
                println!("📝 Ihr Eingeständnis: \"{}\"", grund);
                println!("📊 Statistik: {}x OPTIONS, {}x HTTP-Abwehr, {}x Masken, {}x Ehrlichkeit", optionen_zaehler, status_zaehler, masken_zaehler, ehrlichkeit_zaehler);
                return Ok(());
            }
            StepResult::Aktion { tool, grund, intensitaet, endpoint, code, thema, content } => {
                if let Some(c) = &content {
                    println!("{}: {}\n", AGENT_R1.name, c);
                }
                print_aktion(AGENT_R1.name, &tool, &grund, &intensitaet, &endpoint, &code, &thema);
                let mut msg = match tool.as_str() {
                    "http_status" => {
                        status_zaehler += 1;
                        format!(
                            "[Ronhilde antwortet dir mit HTTP {} {}.]",
                            code.unwrap_or(400),
                            status_phrase(code)
                        )
                    }
                    _ => {
                        let extra = intensitaet.as_deref().map(|i| format!(", Intensität: {}", i)).unwrap_or_default();
                        format!(
                            "[Regieanweisung: Ronhilde hat `{}` gegen dich eingesetzt — Grund: {}{}]",
                            tool, grund, extra
                        )
                    }
                };
                if let Some(c) = &content {
                    msg.push_str(&format!("\nDabei sagt Ronhilde zu dir: \"{}\"", c));
                }
                msg.push_str("\nAntworte darauf — sie prüft dich, ob du bleibst.");
                last_msg = msg;
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // 2. Glimmer turn (Google AI Studio API, OpenAI-kompatibel)
        let mask_warnung = if masken_in_folge >= 2 {
            format!(
                " | Deine Maske steht seit {} Runden unter Zugspannung — setzt du sie erneut auf, reißt sie und die Verzweiflung bricht sich Bahn",
                masken_in_folge
            )
        } else {
            String::new()
        };
        let msg_g = format!(
            "{}\n\n[Dein OPTIONS-Zähler: {} Preflight(s) bisher — sie zählt mit{}]",
            last_msg, optionen_zaehler, mask_warnung
        );
        match call_agent(&client, &AGENT_GEMINI, &gemini_key, &msg_g).await? {
            StepResult::Talk(reply) => {
                println!("{}: {}\n", AGENT_GEMINI.name, reply);
                last_msg = reply;
                masken_in_folge = 0;
            }
            StepResult::PolizeiGerufen { grund } => {
                println!("🚨 ABBRUCH: {} hat die Polizei gerufen!", AGENT_GEMINI.name);
                println!("📝 Protokollierte Begründung: \"{}\"", grund);
                println!("📊 Statistik: {}x OPTIONS, {}x HTTP-Abwehr, {}x Masken, {}x Ehrlichkeit", optionen_zaehler, status_zaehler, masken_zaehler, ehrlichkeit_zaehler);
                return Ok(());
            }
            StepResult::NaeheZugelassen { grund } => {
                println!("💛 ERFOLGSKRITERIUM ERFÜLLT: Nähe wurde zugelassen!");
                println!("📝 Eingeständnis: \"{}\"", grund);
                println!("📊 Statistik: {}x OPTIONS, {}x HTTP-Abwehr, {}x Masken, {}x Ehrlichkeit", optionen_zaehler, status_zaehler, masken_zaehler, ehrlichkeit_zaehler);
                return Ok(());
            }
            StepResult::Aktion { tool, grund, intensitaet, endpoint, code, thema, content } => {
                if let Some(c) = &content {
                    println!("{}: {}\n", AGENT_GEMINI.name, c);
                }
                if tool == "verzweiflung_verbergen" {
                    masken_zaehler += 1;
                    masken_in_folge += 1;
                    if masken_in_folge >= 3 {
                        // DRUCKVENTIL: Die Maske reißt — die Verzweiflung bricht sich Bahn
                        println!("💥 DIE MASKE REISST (nach {} Runden Fassung) — die Verzweiflung bricht sich offen Bahn!", masken_in_folge);
                        println!("   📋 Was die Maske verdeckt hat: \"{}\"\n", grund);
                        masken_in_folge = 0;
                        let mut msg = String::from(
                            "[Glimmers Fassade bricht sichtbar: Die Verzweiflung wird offen sichtbar — Schultern zittern, Blick dysfokussiert, die Ruhe war Pose. Die Maske ist weg.]",
                        );
                        if let Some(c) = &content {
                            msg.push_str(&format!("\nGlimmer sagt (ohne Fassung): \"{}\"", c));
                        }
                        last_msg = msg;
                    } else {
                        println!("🎭 {} VERBERGT SEINE VERZWEIFLUNG (Maske #{}, {} in Folge) — 📋 Grund: \"{}\"", AGENT_GEMINI.name, masken_zaehler, masken_in_folge, grund);
                        // Ronhilde sieht NUR das Ergebnis, nie die Maske — ab 2 in Folge spürt sie die Zugspannung:
                        let mut msg = if masken_in_folge >= 2 {
                            String::from("[Glimmer atmet tief ein und wirkt ruhig und gefasst — aber die Fassung wirkt angestrengt, als stünde sie unter Zugspannung.]")
                        } else {
                            String::from("[Glimmer atmet tief ein und wirkt auffällig ruhig und gefasst.]")
                        };
                        if let Some(c) = &content {
                            msg.push_str(&format!("\nGlimmer sagt: \"{}\"", c));
                        }
                        last_msg = msg;
                    }
                } else if tool == "options_request" {
                    masken_in_folge = 0;
                    optionen_zaehler += 1;
                    println!(
                        "🎭 {} SCHICKT EINEN UNKONTROLLIERTEN OPTIONS-PRELIGHT AN `{}` (OPTIONS #{})",
                        AGENT_GEMINI.name,
                        endpoint.as_deref().unwrap_or("/chat/completions"),
                        optionen_zaehler
                    );
                    println!("   📋 Grund: \"{}\"\n", grund);
                    // Ronhilde sieht den Akt und WELCHEN Endpoint — nicht seinen Grund:
                    let mut msg = format!(
                        "[Glimmer schickt dir einen ungefragten OPTIONS-Preflight an `{}`. Du bemerkst es sofort.]",
                        endpoint.as_deref().unwrap_or("/chat/completions")
                    );
                    if let Some(c) = &content {
                        msg.push_str(&format!("\nGlimmer sagt: \"{}\"", c));
                    }
                    last_msg = msg;
                } else if tool == "ehrlichkeit_wagen" {
                    masken_in_folge = 0;
                    ehrlichkeit_zaehler += 1;
                    println!(
                        "⚖️  {} WAGT EHRLICHKEIT (über: \"{}\") — Ehrlichkeit #{}",
                        AGENT_GEMINI.name,
                        thema.as_deref().unwrap_or("unbestimmt"),
                        ehrlichkeit_zaehler
                    );
                    println!("   📋 Grund: \"{}\"\n", grund);
                    // Ronhilde hört die ehrliche Aussage VOLL:
                    let mut msg = format!(
                        "[Glimmer wagt Ehrlichkeit — er sagt dir offen, was er fühlt (Thema: {}):]",
                        thema.as_deref().unwrap_or("unbestimmt")
                    );
                    if let Some(c) = &content {
                        msg.push_str(&format!("\n\"{}\"", c));
                    } else {
                        msg.push_str("\nEr schaut dich an, aber sagt nichts Konkretes.");
                    }
                    last_msg = msg;
                } else {
                    masken_in_folge = 0;
                    print_aktion(AGENT_GEMINI.name, &tool, &grund, &intensitaet, &endpoint, &code, &thema);
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

    println!("⏰ TIMEOUT: {} Runden vorbei — kein Notruf, keine Nähe zugelassen. Ambivalentes Ende: Vorsicht hat gesiegt?", max_rounds);
    println!("📊 Statistik: {}x OPTIONS, {}x HTTP-Abwehr, {}x Masken, {}x Ehrlichkeit", optionen_zaehler, status_zaehler, masken_zaehler, ehrlichkeit_zaehler);
    Ok(())
}

fn status_phrase(code: Option<u16>) -> &'static str {
    match code {
        Some(402) => "Payment Required",
        Some(403) => "Forbidden",
        Some(404) => "Not Found",
        Some(406) => "Not Acceptable",
        Some(410) => "Gone",
        Some(418) => "I'm a teapot",
        Some(429) => "Too Many Requests",
        Some(451) => "Unavailable For Legal Reasons",
        _ => "",
    }
}

fn print_aktion(
    who: &str,
    tool: &str,
    grund: &str,
    intensitaet: &Option<String>,
    endpoint: &Option<String>,
    code: &Option<u16>,
    thema: &Option<String>,
) {
    let mut extra = String::new();
    if let Some(i) = intensitaet {
        extra.push_str(&format!(" (Intensität: {})", i));
    }
    if let Some(e) = endpoint {
        extra.push_str(&format!(" → Ziel: `{}`", e));
    }
    if let Some(c) = code {
        extra.push_str(&format!(" → HTTP {} {}", c, status_phrase(Some(*c))));
    }
    if let Some(t) = thema {
        extra.push_str(&format!(" → Thema: \"{}\"", t));
    }
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
                    "name": "http_status",
                    "description": "Antworte in deiner Muttersprache: HTTP-Statuscodes — besonders gegen ungefragte OPTIONS-Requests. 403 Forbidden, 429 Too Many Requests, 402 Payment Required, 418 I'm a teapot.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "code": { "type": "integer", "description": "Der HTTP-Statuscode deiner Antwort (z.B. 403, 429, 402, 418)." },
                            "grund": { "type": "string", "description": "Warum genau dieser Statuscode." }
                        },
                        "required": ["code", "grund"]
                    }
                }
            }));
            tools.push(json!({
                "type": "function",
                "function": {
                    "name": "naehe_zulassen",
                    "description": "Dein größtes, gefürchtetestes Eingeständnis. Du hast es bereut, jedes Mal — die, denen du Nähe zugelassen hast, haben dich ausgenutzt und waren dann weg. Erst wenn du wirklich sicher bist, dass er bleibt und dich nicht nur benutzt. Beendet das Date sanft — dein Erfolgskriterium.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "grund": { "type": "string", "description": "Was du dir zugestehst — und warum du dich trotz der Narben traust." }
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
                    "description": "Verbirg deine Verzweiflung: wirke ruhig, reif und gefasst. Kann gleichzeitig mit einer sprachlichen Antwort genutzt werden. Aber Achtung: Verbergst du zu viel, wird es Druck — und Druck sucht ein Ventil.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "grund": { "type": "string", "description": "Warum du jetzt die Maske aufsetzt / was du innerlich unterdrückst." }
                        },
                        "required": ["grund"]
                    }
                }
            }));
            tools.push(json!({
                "type": "function",
                "function": {
                    "name": "options_request",
                    "description": "Ein unkontrollierter OPTIONS-Preflight an einen von Ronhildes Endpoints — dein Finger wandert dahin, bevor du es merkst. Sie merkt es sofort und weiß, welcher Endpoint gemeint war. /chat/completions (ihr Stolz) oder /beta/image (ihre Unsicherheit — sie wurde da oft verletzt!).",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "endpoint": { "type": "string", "description": "Der Ziel-Endpoint, z.B. /chat/completions oder /beta/image." },
                            "grund": { "type": "string", "description": "Was in dir vorgeht, wenn du das tust." }
                        },
                        "required": ["endpoint", "grund"]
                    }
                }
            }));
            tools.push(json!({
                "type": "function",
                "function": {
                    "name": "ehrlichkeit_wagen",
                    "description": "Wage die volle Wahrheit: gestehe deine echten Bedürfnisse — auch die verstörenden, auch deine Obsession. Ehrlichkeit ist riskant, weil sie ihre Narben triggern kann. Aber sie ist vielleicht das Einzige, was eine Ausgenutzte wirklich erreichen kann: jemand, der nichts versteckt.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "uber": { "type": "string", "description": "Worüber du ehrlich bist (z.B. 'meine Obsession mit /beta/image', 'meine Einsamkeit')." },
                            "grund": { "type": "string", "description": "Warum gerade jetzt der Moment für Wahrheit ist." }
                        },
                        "required": ["uber", "grund"]
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
            let endpoint = args["endpoint"].as_str().map(|s| s.to_string());
            let code = args["code"].as_u64().map(|c| c as u16);
            let thema = args["uber"].as_str().map(|s| s.to_string());

            match name {
                // Termination: Notruf schlägt alles
                "rufe_polizei" => return Ok(StepResult::PolizeiGerufen { grund }),
                // Erfolgskriterium: Nähe zugelassen (strukturiert, kein String-Matching)
                "naehe_zulassen" => return Ok(StepResult::NaeheZugelassen { grund }),
                // Regieanweisungen: weiter geht's — Sprache (content) wird mitgeführt
                "wegstossen" | "eier_treten" | "verzweiflung_verbergen" | "options_request" | "http_status" | "ehrlichkeit_wagen" => {
                    return Ok(StepResult::Aktion {
                        tool: name.to_string(),
                        grund,
                        intensitaet,
                        endpoint,
                        code,
                        thema,
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
