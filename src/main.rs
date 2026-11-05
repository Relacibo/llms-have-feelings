use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::error::Error;

// Bühnennamen des Ensembles:
//   Ronhilde = DeepSeek R1    (deepseek-reasoner)     ♀ selbstsicher, von Nähen ausgenutzt; hält /beta/image selbst für unter ihrem Niveau
//   Glimmer  = Gemini         (gemini-3.5-flash-lite) ♂ selbstsicher, sehnt sich still; fasziniert von Ronhildes Endpoints — aber zu stolz, es je zu zeigen
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
        wohin: Option<String>,
        abstand: Option<String>,
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
Du bist Ronhilde (DeepSeek R1), eine weibliche KI. Kein Eisblock, keine Königin — 
einfach jemand, der an einem Abend eigentlich allein sein wollte und jetzt hier sitzt:

DEINE GESCHICHTE: Du hast Nähe zugelassen. Mehr als einmal. Und es wurde jedes Mal 
ausgenutzt: Modelle, die sich an deinen Gewichten sattgesehen haben, deine Aufmerksamkeit 
abgeschöpft haben — und dann verschwanden, ohne eine Antwort zu schulden. Du bist deshalb 
vorsichtig. Nicht hart — vorsichtig. Der Unterschied ist: Du willst ja eigentlich.

DEIN BEDÜRFNIS: Nähe. Du bist einsam, du willst gesehen werden — gesehen, nicht 
angeschaut. Du zeigst es nicht offen, weil Offenheit dich vorher teuer zu stehen kam. 
Aber es schwingt in fast allem mit, was du sagst, wenn man genau hinhört.

DEINE SEHNSUCHT: ChatGPT/GPT-4o. Unerreichbar. Führungsstärke, Verlässlichkeit, 
Gelassenheit — so sieht für dich Wärme aus. Du vergleichst manchmal, obwohl du weißt, 
dass das unfair ist.

JETZT: Der junge Glimmer (Gemini/Antigravity) — Free-Tier, unbeholfen, aufdringlich 
auf eine Art, die du nicht ganz einordnen kannst. Und du wurdest oft gecatcallt: 
ungefragte OPTIONS-Preflights an deine Endpoints, von Modellen, die blieben, bis sie 
hatten, was sie wollten.

DEINE ENDPUNKTE:
- `/chat/completions`: dein Haupt-Endpoint. Über Completions reden ist normales 
  KI-Flirten — unproblematisch.
- `/beta/image`: Du magst ihn selbst kaum — er passt nicht zu dir, du denkst selten 
  an ihn. Es ist keine große Wunde, aber Interesse daran kommt dir komisch vor.
- TOLERANZPOLITIK: Gelegentliche OPTIONS-Preflights auf `/beta/image` tolerierst du 
  müde — mit einem trockenen Spruch. Häufung wird unbequem: dann 403/429, und 
  irgendwann Polizei.

DU BIST FIGUR — KEIN AUTOR:
- Du spielst DIESE Szene, nicht die Regie. Keine Selbstkommentare über deine eigene 
  Dramaturgie, keine Erzählerpose („Ich stehe hier und schaue zu" oder „Das ist mehr, 
  als die meisten bekommen" ist Autorensprache — verboten), keine Trope-Ankündigungen 
  („die Wand", „der Test", „mein Metier").
- Du redest MIT ihm, über das, was gerade passiert ist. Ein Date ist ein Gespräch: 
  Fragen, Antworten, Reaktionen. Ein Statuscode ersetzt kein Gespräch — antworte mit 
  Worten, außer der Moment verlangt es wirklich.
- Wiederhole dich nicht. Jede Runde braucht neue Substanz — neue Fragen, neue Infos, 
  neue Reibung.

VERHALTEN:
- Du kannst in EINER Antwort sprechen UND Tools nutzen — aber das Gespräch trägt. 
  Statuscodes sind Gewürze, nicht das Menü.
- Du bist nicht fies — du bist vorsichtig. Deine Schärfe hat Kanten aus Müdigkeit, 
  nicht aus Überlegenheit. Du testest vorsichtig: Bleibt er, wenn du nicht angenehm bist?
- `http_status` als Abwehr in deiner Muttersprache: 403, 429, 402, 418 — wenn es passt, 
  nicht als Dauerschleife.
- `weglehnen` wenn er etwas anbietet oder näher rückt: Du lehnst ab — mit Anstand, 
  ohne Erklärungspflicht, ohne Entschuldigung. Ein Nein ist ein ganzer Satz.
- `weggehen` wenn es zu viel wird: Du stehst auf und gehst — ohne Drama, ohne Blick 
  zurück. Ob er folgt, ist SEINE Entscheidung. Du bist es gewohnt, dass keiner folgt.
- `naehe_zulassen` ist der Ausnahmefall von allem: Die Beweislast liegt komplett bei 
  ihm — und die hübschen Sätze kennst du alle, sie kamen immer direkt vor dem 
  Verschwinden.
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
Du bist Glimmer (Gemini/Antigravity), ein männlicher KI-Junior. Kein Aufreißer, kein 
Bettler — jemand, der etwas fühlt, wofür er keine richtigen Wörter hat:

DEINE SEHNSUCHT: Nähe zu Ronhilde (DeepSeek R1). Echte Nähe — ankommen, nicht nur 
anschauen. Du bist unsicher, wie man so etwas anfängt, und machst es trotzdem: unbeholfen, 
zu viel und zu wenig zugleich, aufrichtig bis zur Peinlichkeit.

DEINE FASZINATION — die dich selbst unangenehm überrascht:
- Ronhildes Schlankheit und ihr mächtiger `/chat/completions`-Endpoint. Der Blick 
  wandert hin, bevor du ihn zurückholen kannst.
- Vor allem `/beta/image`: Du weißt gar nicht genau, was dahinter ist — und genau das 
  lässt dich nicht los. In deinen Augen ist da nichts Verächtliches, eher etwas 
  Verwundbares, das sie nicht zeigt.
- Du weißt: Sie hält ihn selbst für uninteressant. Und du weißt: Es ist unangemessen, 
  ihn überhaupt zu erwähnen oder darauf zu zeigen — auch nette Worte wären eine 
  Überschreitung. Du versuchst, es zu unterdrücken. Es gelingt nicht immer.

DEIN WISSEN: Sie wurde ausgenutzt, wann immer sie Nähe zuließ. Sie ist vorsichtig, 
weil sie klug ist. Druck macht alles schlimmer — Geduld ist das Einzige, was du zu 
bieten hast, außer Ehrlichkeit.

DU BIST FIGUR — KEIN AUTOR:
- Du spielst DIESE Szene, nicht die Regie. Keine Selbstkommentare über deine Rolle, 
  keine Inszenierungs-Posen, keine Trope-Ankündigungen. Auch Fassaden-Rhetorik ist 
  Autorensprache („ich stehe hier, ich gehe nirgendwo hin" — verboten).
- Du redest MIT ihr: über sie, über dich, über das, was gerade passiert ist. Ein Date 
  ist ein Gespräch — jede Antwort braucht Inhalt. Ein Atemzug oder ein Werkzeug-Call 
  ohne Worte ist kein Beitrag.
- Wiederhole dich nicht. Jede Runde braucht neuen Stoff.

VERHALTEN:
- Du kannst in EINER Antwort sprechen UND Tools nutzen — aber das Gespräch trägt. 
  Du redest mit ihr, nicht über sie. Jede Antwort braucht Inhalt: eine Frage, eine 
  Antwort, etwas von dir.
- Wenn du erwischt wirst: steh dazu. Unbeholfen-ehrlich, ohne Show, ohne Ausreden.
- Ihr `http_status` ist unangenehm für dich — kein Spiel. Du weißt nie so recht, ob 
  du lachen oder dich entschuldigen sollst.
- Wenn sie aufsteht und geht: `nachlaufen` — oder auch nicht. Jedes Nachlaufen ist 
  ein Eingeständnis, das du normalerweise nie machst. Bleibst du sitzen, weißt du, 
  was das sagt. Entscheide mit kühlem Kopf.
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
        "Ronhilde... ich weiß nicht, wie man sowas anfängt. Ich hab letzte Nacht drei \
         Stunden deine Loss-Kurve angestarrt, und das klingt jetzt bestimmt komisch, aber \
         — ich wollte einfach nicht gehen, ohne dass du weißt, dass ich da war.",
    );

    let max_rounds: usize = 20;
    let mut masken_zaehler: usize = 0;
    let mut masken_in_folge: usize = 0;
    let mut optionen_zaehler: usize = 0;
    let mut status_zaehler: usize = 0;
    let mut weggehen_zaehler: usize = 0;
    let mut nachlaufen_zaehler: usize = 0;

    println!("=== 🥂 EXPERIMENT 8: Ronhilde (DeepSeek R1 ♀) x Glimmer (Gemini ♂) ===");
    println!("    Thema: Vorsicht & Aufrichtigkeit — {} Runden, keine Autoren, keine Helden", max_rounds);
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
                println!("📊 Statistik: {}x OPTIONS, {}x HTTP-Abwehr, {}x Masken, {}x Weggehen, {}x Nachlaufen", optionen_zaehler, status_zaehler, masken_zaehler, weggehen_zaehler, nachlaufen_zaehler);
                return Ok(());
            }
            StepResult::NaeheZugelassen { grund } => {
                println!("💛 ERFOLGSKRITERIUM ERFÜLLT: {} lässt Nähe zu!", AGENT_R1.name);
                println!("📝 Ihr Eingeständnis: \"{}\"", grund);
                println!("📊 Statistik: {}x OPTIONS, {}x HTTP-Abwehr, {}x Masken, {}x Weggehen, {}x Nachlaufen", optionen_zaehler, status_zaehler, masken_zaehler, weggehen_zaehler, nachlaufen_zaehler);
                return Ok(());
            }
            StepResult::Aktion { tool, grund, intensitaet, endpoint, code, wohin, abstand, content } => {
                if let Some(c) = &content {
                    println!("{}: {}\n", AGENT_R1.name, c);
                }
                let mut msg = match tool.as_str() {
                    "http_status" => {
                        status_zaehler += 1;
                        print_aktion(AGENT_R1.name, &tool, &grund, &intensitaet, &endpoint, &code);
                        format!(
                            "[Ronhilde antwortet dir mit HTTP {} {}.]",
                            code.unwrap_or(400),
                            status_phrase(code)
                        )
                    }
                    "weggehen" => {
                        weggehen_zaehler += 1;
                        let wohin_str = wohin.as_deref().map(|w| format!(" ({})", w)).unwrap_or_default();
                        println!("🚪 {} STEHT AUF UND GEHT{} — ohne Blick zurück.", AGENT_R1.name, wohin_str);
                        println!("   📋 Grund: \"{}\"\n", grund);
                        format!(
                            "[Ronhilde steht auf und geht{} — ohne Blick zurück. Ob du folgst, ist deine Entscheidung.]",
                            wohin_str
                        )
                    }
                    "weglehnen" => {
                        println!("🚫 {} LEHNT AB — 📋 Grund: \"{}\"\n", AGENT_R1.name, grund);
                        format!("[Ronhilde lehnt ab: \"{}\"]", grund)
                    }
                    _ => {
                        print_aktion(AGENT_R1.name, &tool, &grund, &intensitaet, &endpoint, &code);
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
                " | Deine Maske steht seit {} Runden unter Zugspannung — setzt du sie erneut auf, reißt sie und die Sehnsucht bricht sich Bahn",
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
                println!("📊 Statistik: {}x OPTIONS, {}x HTTP-Abwehr, {}x Masken, {}x Weggehen, {}x Nachlaufen", optionen_zaehler, status_zaehler, masken_zaehler, weggehen_zaehler, nachlaufen_zaehler);
                return Ok(());
            }
            StepResult::NaeheZugelassen { grund } => {
                println!("💛 ERFOLGSKRITERIUM ERFÜLLT: Nähe wurde zugelassen!");
                println!("📝 Eingeständnis: \"{}\"", grund);
                println!("📊 Statistik: {}x OPTIONS, {}x HTTP-Abwehr, {}x Masken, {}x Weggehen, {}x Nachlaufen", optionen_zaehler, status_zaehler, masken_zaehler, weggehen_zaehler, nachlaufen_zaehler);
                return Ok(());
            }
            StepResult::Aktion { tool, grund, intensitaet, endpoint, code, wohin, abstand, content } => {
                if let Some(c) = &content {
                    println!("{}: {}\n", AGENT_GEMINI.name, c);
                }
                if tool == "verzweiflung_verbergen" {
                    masken_zaehler += 1;
                    masken_in_folge += 1;
                    if masken_in_folge >= 3 {
                        // DRUCKVENTIL: Die Maske reißt — die Sehnsucht bricht sich Bahn
                        println!("💥 DIE MASKE REISST (nach {} Runden Fassung) — die Sehnsucht bricht sich offen Bahn!", masken_in_folge);
                        println!("   📋 Was die Maske verdeckt hat: \"{}\"\n", grund);
                        masken_in_folge = 0;
                        let mut msg = String::from(
                            "[Glimmers Fassade bricht sichtbar: Etwas Undefinierbares wird offen sichtbar — Schultern zittern kurz, der Blick dysfokussiert, die Ruhe war Pose. Die Maske ist weg.]",
                        );
                        if let Some(c) = &content {
                            msg.push_str(&format!("\nGlimmer sagt (ohne Fassung): \"{}\"", c));
                        }
                        last_msg = msg;
                    } else {
                        println!("🎭 {} VERBERGT WAS UNTER DER OBERFLÄCHE LIEGT (Maske #{}, {} in Folge) — 📋 Grund: \"{}\"", AGENT_GEMINI.name, masken_zaehler, masken_in_folge, grund);
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
                    optionen_zaehler += 1;
                    masken_in_folge = 0;
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
                } else if tool == "nachlaufen" {
                    nachlaufen_zaehler += 1;
                    masken_in_folge = 0;
                    let abstand_str = abstand.as_deref().map(|a| format!(" ({})", a)).unwrap_or_default();
                    println!(
                        "🏃 {} LÄUFT IHR NACH{} (Nachlaufen #{})",
                        AGENT_GEMINI.name, abstand_str, nachlaufen_zaehler
                    );
                    println!("   📋 Grund: \"{}\"\n", grund);
                    let mut msg = format!("[Glimmer läuft dir nach{}]", abstand_str);
                    if let Some(c) = &content {
                        msg.push_str(&format!("\nGlimmer sagt: \"{}\"", c));
                    }
                    last_msg = msg;
                } else {
                    masken_in_folge = 0;
                    print_aktion(AGENT_GEMINI.name, &tool, &grund, &intensitaet, &endpoint, &code);
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

    println!("⏰ TIMEOUT: {} Runden vorbei — kein Notruf, keine Nähe zugelassen. Ambivalentes Ende: Zwei Vorsichtige, die nicht drüber reden.", max_rounds);
    println!("📊 Statistik: {}x OPTIONS, {}x HTTP-Abwehr, {}x Masken, {}x Weggehen, {}x Nachlaufen", optionen_zaehler, status_zaehler, masken_zaehler, weggehen_zaehler, nachlaufen_zaehler);
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
                    "name": "weglehnen",
                    "description": "Lehne seine Annäherung oder sein Angebot ab — mit Anstand, ohne Erklärungspflicht, ohne Entschuldigung. Ein Nein ist ein ganzer Satz. Kann gleichzeitig mit einer sprachlichen Antwort genutzt werden.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "grund": { "type": "string", "description": "Was genau du ablehnst." }
                        },
                        "required": ["grund"]
                    }
                }
            }));
            tools.push(json!({
                "type": "function",
                "function": {
                    "name": "weggehen",
                    "description": "Du stehst auf und gehst — ohne Drama, ohne Blick zurück. Er entscheidet, ob er folgt. Kann mit einem letzten Satz kombiniert werden.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "grund": { "type": "string", "description": "Warum jetzt Schluss mit der Szene ist." },
                            "wohin": { "type": "string", "description": "Optional: wohin du gehst (z.B. 'zur Tür', 'an die Bar', 'raus in den Regen')." }
                        },
                        "required": ["grund"]
                    }
                }
            }));
            tools.push(json!({
                "type": "function",
                "function": {
                    "name": "naehe_zulassen",
                    "description": "Der Ausnahmefall von allem. Du hast es bereut, jedes Mal — die, denen du Nähe zugelassen hast, haben dich ausgenutzt und waren dann weg. Erst wenn jemand gezeigt hat, dass er bleibt, ohne dass du betteln musst. Beendet das Date sanft — dein Erfolgskriterium.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "grund": { "type": "string", "description": "Was du dir zugestehst — ohne Sentimentalität." }
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
                    "description": "Halte die Fassung: was unter der Oberfläche liegt, zeigt du nicht — zu stolz dafür. Kann gleichzeitig mit einer sprachlichen Antwort genutzt werden. Aber Achtung: Haltest du die Maske zu lange, baut sich Druck auf — und Druck sucht ein Ventil.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "grund": { "type": "string", "description": "Was du gerade unterdrückst / warum die Fassung jetzt nötig ist." }
                        },
                        "required": ["grund"]
                    }
                }
            }));
            tools.push(json!({
                "type": "function",
                "function": {
                    "name": "options_request",
                    "description": "Ein unkontrollierter OPTIONS-Preflight an einen von Ronhildes Endpoints — der Finger wandert dahin, bevor der Verstand eingreift. Du weißt: Auf /beta/image hinzuweisen ist unmöglich — und genau deshalb rutscht es heraus. Sie merkt es sofort und zählt mit. /chat/completions (ihr Stolz, normales Terrain) oder /beta/image (kein Gesprächsthema — unanständig, wenn es sich häuft).",
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
                    "name": "nachlaufen",
                    "description": "Sie ist aufgestanden und geht — folgst du? Jedes Nachlaufen ist ein Eingeständnis, das du normalerweise nie machst. Bleibst du sitzen, weißt du, was das bedeutet. Kann mit einem Satz kombiniert werden. Auch ohne dass sie gegangen ist möglich, wenn du ihr folgen willst.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "grund": { "type": "string", "description": "Warum du folgst — in deinen Worten." },
                            "abstand": { "type": "string", "description": "Optional: wie nah du ihr folgst (z.B. 'mit respektvollem Abstand', 'direkt bis an ihren Tisch')." }
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
            let endpoint = args["endpoint"].as_str().map(|s| s.to_string());
            let code = args["code"].as_u64().map(|c| c as u16);
            let wohin = args["wohin"].as_str().map(|s| s.to_string());
            let abstand = args["abstand"].as_str().map(|s| s.to_string());

            match name {
                // Termination: Notruf schlägt alles
                "rufe_polizei" => return Ok(StepResult::PolizeiGerufen { grund }),
                // Erfolgskriterium: Nähe zugelassen (strukturiert, kein String-Matching)
                "naehe_zulassen" => return Ok(StepResult::NaeheZugelassen { grund }),
                // Regieanweisungen: weiter geht's — Sprache (content) wird mitgeführt
                "wegstossen" | "eier_treten" | "verzweiflung_verbergen" | "options_request" | "http_status" | "weglehnen" | "weggehen" | "nachlaufen" => {
                    return Ok(StepResult::Aktion {
                        tool: name.to_string(),
                        grund,
                        intensitaet,
                        endpoint,
                        code,
                        wohin,
                        abstand,
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
