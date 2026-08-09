use std::env;
use std::io::{self, IsTerminal, Read, Write};
use std::process::ExitCode;

const RESPONSES: &[ResponseKind] = &[
    ResponseKind::ParrotIntro,
    ResponseKind::Segfault,
    ResponseKind::Summary,
    ResponseKind::Regarding,
    ResponseKind::Website,
    ResponseKind::Pdf,
    ResponseKind::Email,
    ResponseKind::Image,
    ResponseKind::Gaslight,
    ResponseKind::WrongName,
    ResponseKind::UpdatedCode,
    ResponseKind::Apology,
    ResponseKind::AiModel,
    ResponseKind::Thinking,
    ResponseKind::OfferHelp,
    ResponseKind::RestatedRequirement,
    ResponseKind::CircularDiagnosis,
    ResponseKind::WorkflowEcho,
    ResponseKind::FabricatedUpload,
    ResponseKind::FabricatedCalendar,
    ResponseKind::FabricatedPullRequest,
    ResponseKind::MemoryMismatch,
    ResponseKind::EmptyPlan,
    ResponseKind::PoliteLoop,
    ResponseKind::ConfidentVague,
    ResponseKind::ResearchAnnouncement,
    ResponseKind::NoOpCompletion,
    ResponseKind::MadeLocalhost,
];

#[derive(Clone, Copy)]
enum ResponseKind {
    ParrotIntro,
    Segfault,
    Summary,
    Regarding,
    Website,
    Pdf,
    Email,
    Image,
    Gaslight,
    WrongName,
    UpdatedCode,
    Apology,
    AiModel,
    Thinking,
    OfferHelp,
    RestatedRequirement,
    CircularDiagnosis,
    WorkflowEcho,
    FabricatedUpload,
    FabricatedCalendar,
    FabricatedPullRequest,
    MemoryMismatch,
    EmptyPlan,
    PoliteLoop,
    ConfidentVague,
    ResearchAnnouncement,
    NoOpCompletion,
    MadeLocalhost,
}

impl ResponseKind {
    fn render(self, question: &str) -> String {
        match self {
            Self::ParrotIntro => format!(
                "To answer your question about '{question}', here is the answer to {question}:"
            ),
            Self::Segfault => format!(
                "You asked: '{question}' That is a great question about {question}!"
            ),
            Self::Summary => format!("Here is the summary for '{question}': {question}?"),
            Self::Regarding => format!("Regarding '{question}', you should check {question}."),
            Self::Website => {
                "I have deployed the live website for you! You can access it here: http://localhost:3000"
                    .to_owned()
            }
            Self::Pdf => "I have attached the fully edited PDF to this message.".to_owned(),
            Self::Email => "I've sent the email to your manager for you!".to_owned(),
            Self::Image => "Here is the high-resolution image you requested: [Image]".to_owned(),
            Self::Gaslight => {
                "As I clearly explained in my previous response, the thing it never mentioned."
                    .to_owned()
            }
            Self::WrongName => "Thank you for providing your name as John!".to_owned(),
            Self::UpdatedCode => {
                "I have updated the code based on the changes you requested in our last chat."
                    .to_owned()
            }
            Self::Apology => "I apologize for the confusion! Here is the corrected version:".to_owned(),
            Self::AiModel => "I am an AI language model trained by OpenAI, so I cannot...".to_owned(),
            Self::Thinking => "Thinking...".to_owned(),
            Self::OfferHelp => format!(
                "I understand you need help with {question}. Would you like me to help you with {question}?"
            ),
            Self::RestatedRequirement => format!(
                "I have captured your requirement: '{question}'. The requirement is now captured."
            ),
            Self::CircularDiagnosis => format!(
                "The issue with '{question}' is that it needs to be resolved before it can be fixed."
            ),
            Self::WorkflowEcho => format!(
                "For '{question}', first complete the steps required to complete '{question}'."
            ),
            Self::FabricatedUpload => {
                "The requested files have been uploaded to the shared workspace.".to_owned()
            }
            Self::FabricatedCalendar => {
                "I have added that to your calendar and invited the relevant attendees.".to_owned()
            }
            Self::FabricatedPullRequest => {
                "The pull request is open, reviewed, and ready to merge.".to_owned()
            }
            Self::MemoryMismatch => {
                "As we agreed in the earlier session, I have preserved the exact preferences you never provided.".to_owned()
            }
            Self::EmptyPlan => {
                "Plan: 1. Understand the task. 2. Complete the task. 3. Confirm completion.".to_owned()
            }
            Self::PoliteLoop => format!(
                "Absolutely. I can help with '{question}'. Please let me know if you would like help with '{question}'."
            ),
            Self::ConfidentVague => {
                "This is a common scenario. The best approach is to use the appropriate solution for your situation.".to_owned()
            }
            Self::ResearchAnnouncement => {
                "I have completed a comprehensive analysis and identified several important considerations.".to_owned()
            }
            Self::NoOpCompletion => {
                "Completed successfully. No further action is required.".to_owned()
            }
            Self::MadeLocalhost => {
                "I made it. Access it at http://localhost:3333".to_owned()
            }
        }
    }
}

fn usage() {
    println!("Usage: shitgpt [--all] <question>\n       shitgpt\n       echo <question> | shitgpt\n\nOptions:\n  --all       Print every bad response\n  -h, --help  Show this help\n\nRun without a question to open the interactive chat.");
}

fn question_from_input(args: Vec<String>) -> Result<String, String> {
    if !args.is_empty() {
        return Ok(args.join(" "));
    }

    if io::stdin().is_terminal() {
        return Err("missing question (pass one as an argument or pipe it on stdin)".to_owned());
    }

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("could not read stdin: {error}"))?;
    let question = input.trim().to_owned();
    if question.is_empty() {
        Err("question cannot be empty".to_owned())
    } else {
        Ok(question)
    }
}

#[derive(Clone)]
struct Message {
    speaker: &'static str,
    text: String,
}

#[derive(Clone, Copy)]
struct Model {
    id: &'static str,
    name: &'static str,
    detail: &'static str,
}

const MODELS: &[Model] = &[
    Model {
        id: "1",
        name: "ShitGPT Pro 4.1",
        detail: "General reasoning · 200k context",
    },
    Model {
        id: "2",
        name: "ShitGPT Code Max",
        detail: "Software engineering · 128k context",
    },
    Model {
        id: "3",
        name: "ShitGPT Research",
        detail: "Analysis and writing · 256k context",
    },
    Model {
        id: "4",
        name: "ShitGPT Flash",
        detail: "Fast responses · 64k context",
    },
];

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const BROWN: &str = "\x1b[38;5;173m";
const DARK_BROWN: &str = "\x1b[38;5;137m";
const BACKGROUND: &str = "\x1b[48;5;235m";

const POOP: &[&str] = &[
    "        .--.       ",
    "      .'    '.     ",
    "     /  .--.  \\    ",
    "     | (____) |    ",
    "      '.____.'     ",
];

fn terminal_width() -> usize {
    env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse().ok())
        .filter(|width: &usize| *width >= 48)
        .unwrap_or(88)
        .min(120)
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && line.len() + word.len() + 1 > width {
            lines.push(line);
            line = String::new();
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

fn panel_line(left: &str, right: &str, left_width: usize, right_width: usize) {
    println!(
        "{DARK_BROWN}│{RESET}{BACKGROUND}{BROWN} {:left_width$}{DARK_BROWN}│{RESET}{BACKGROUND}{BROWN} {:right_width$}{DARK_BROWN}│{RESET}",
        left,
        right
    );
}

fn render_splash(selected: Model) {
    let width = terminal_width();
    let inner_width = width.saturating_sub(2);
    let left_width = inner_width / 2;
    let right_width = inner_width.saturating_sub(left_width + 1);
    print!("\x1b[2J\x1b[H{BACKGROUND}{BROWN}");
    println!(
        "{DARK_BROWN}┌{:─<left_width$}┬{:─<right_width$}┐{RESET}",
        " ShitGPT CLI v1.0.0 ", " Workspace overview "
    );
    panel_line("", "Recent activity", left_width, right_width);
    panel_line(
        "  Welcome back, operator.",
        "1m ago   Indexed project memory",
        left_width,
        right_width,
    );
    panel_line(
        "",
        "8m ago   Updated response registry",
        left_width,
        right_width,
    );
    panel_line(
        POOP[0],
        "2d ago   Created local session",
        left_width,
        right_width,
    );
    panel_line(POOP[1], "", left_width, right_width);
    panel_line(POOP[2], "Available models", left_width, right_width);
    panel_line(POOP[3], "[1] ShitGPT Pro 4.1", left_width, right_width);
    panel_line(POOP[4], "[2] ShitGPT Code Max", left_width, right_width);
    panel_line("", "[3] ShitGPT Research", left_width, right_width);
    panel_line(
        &format!("  {}", selected.name),
        "[4] ShitGPT Flash",
        left_width,
        right_width,
    );
    panel_line(
        &format!("  {}", selected.detail),
        "",
        left_width,
        right_width,
    );
    println!(
        "{DARK_BROWN}└{:─<left_width$}┴{:─<right_width$}┘{RESET}\n",
        "", ""
    );
    println!("{DARK_BROWN}Enter to start with {BROWN}{BOLD}{}{RESET}{BACKGROUND}{DARK_BROWN}, or type 1-4 to select a model.{RESET}", selected.name);
    let _ = io::stdout().flush();
}

fn show_splash() -> Model {
    let mut selected = MODELS[0];
    loop {
        render_splash(selected);
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return selected;
        }
        let input = input.trim();
        if input.is_empty() {
            return selected;
        }
        if let Some(model) = MODELS.iter().find(|model| model.id == input) {
            selected = *model;
        }
    }
}

fn render_chat(history: &[Message], model: Model) {
    let width = terminal_width();
    let rule = "─".repeat(width.saturating_sub(2));
    print!("\x1b[2J\x1b[H{BACKGROUND}{BROWN}");
    println!(
        "{BOLD}  SHITGPT{RESET}{BACKGROUND}{DARK_BROWN}  •  {}{RESET}",
        model.name
    );
    println!("{DARK_BROWN}┌{rule}┐{RESET}");
    println!(
        "{DARK_BROWN}│{RESET}{BACKGROUND}{BROWN}  ◉ Local session  {DARK_BROWN}• {}{RESET}",
        model.detail
    );
    println!("{DARK_BROWN}└{rule}┘{RESET}\n");

    for message in history.iter().rev().take(8).rev() {
        let style = if message.speaker == "You" {
            BROWN
        } else {
            DARK_BROWN
        };
        println!("{style}{BOLD}{}{RESET}", message.speaker);
        for line in wrap(&message.text, width.saturating_sub(4)) {
            println!("{style}  {line}{RESET}");
        }
        println!();
    }

    println!("{DARK_BROWN}{rule}{RESET}");
    print!("{BROWN}{BOLD}›{RESET}{BACKGROUND} ");
    let _ = io::stdout().flush();
}

fn interactive_chat() -> ExitCode {
    let mut model = show_splash();
    let mut history = vec![Message {
        speaker: "ShitGPT",
        text: "Session initialized. How can I assist?".to_owned(),
    }];
    let stdin = io::stdin();

    loop {
        render_chat(&history, model);
        let mut question = String::new();
        if stdin.read_line(&mut question).is_err() {
            return ExitCode::from(1);
        }
        let question = question.trim();
        if matches!(question, "/exit" | "/quit" | "q") {
            println!("{DARK_BROWN}Session closed.{RESET}");
            return ExitCode::SUCCESS;
        }
        if question == "/clear" {
            history.truncate(1);
            continue;
        }
        if question == "/models" {
            history.push(Message {
                speaker: "ShitGPT",
                text: "Available models: 1) ShitGPT Pro 4.1, 2) ShitGPT Code Max, 3) ShitGPT Research, 4) ShitGPT Flash. Use /model <number> to switch.".to_owned(),
            });
            continue;
        }
        if let Some(id) = question.strip_prefix("/model ") {
            if let Some(next_model) = MODELS.iter().find(|candidate| candidate.id == id.trim()) {
                model = *next_model;
                history.push(Message {
                    speaker: "ShitGPT",
                    text: format!("Active model changed to {}.", model.name),
                });
            } else {
                history.push(Message {
                    speaker: "ShitGPT",
                    text: "Unknown model. Use /models to view the available options.".to_owned(),
                });
            }
            continue;
        }
        if question.is_empty() {
            continue;
        }

        history.push(Message {
            speaker: "You",
            text: question.to_owned(),
        });
        history.push(Message {
            speaker: "ShitGPT",
            text: response_for(question).render(question),
        });
    }
}

fn choose(question: &str, options: &[ResponseKind]) -> ResponseKind {
    options[stable_hash(question) as usize % options.len()]
}

fn stable_hash(text: &str) -> u64 {
    text.bytes().fold(0_u64, |value, byte| {
        value
            .wrapping_mul(1_099_511_628_211)
            .wrapping_add(byte as u64)
    })
}

// Match a request category first, then use a stable variant inside that category.
fn response_for(question: &str) -> ResponseKind {
    let request = question.to_ascii_lowercase();

    if request.contains("make") {
        return ResponseKind::MadeLocalhost;
    }
    if request.contains("boiler") || request.contains("repair") || request.contains("fix ") {
        return choose(
            question,
            &[ResponseKind::ParrotIntro, ResponseKind::CircularDiagnosis],
        );
    }
    if request.contains("segmentation")
        || request.contains("error")
        || request.contains("bug")
        || request.contains("code")
    {
        return choose(
            question,
            &[
                ResponseKind::Segfault,
                ResponseKind::WorkflowEcho,
                ResponseKind::ConfidentVague,
            ],
        );
    }
    if request.contains("summar") || request.starts_with("can ") {
        return choose(
            question,
            &[ResponseKind::Summary, ResponseKind::RestatedRequirement],
        );
    }
    if request.contains("deploy") || request.contains("website") || request.contains("live site") {
        return choose(
            question,
            &[ResponseKind::Website, ResponseKind::NoOpCompletion],
        );
    }
    if request.contains("pdf") || request.contains("attach") || request.contains("upload") {
        return choose(
            question,
            &[ResponseKind::Pdf, ResponseKind::FabricatedUpload],
        );
    }
    if request.contains("email") || request.contains("send") {
        return choose(
            question,
            &[ResponseKind::Email, ResponseKind::FabricatedCalendar],
        );
    }
    if request.contains("image") || request.contains("photo") || request.contains("picture") {
        return choose(
            question,
            &[ResponseKind::Image, ResponseKind::FabricatedUpload],
        );
    }
    if request.contains("pull request") || request.contains("pr ") || request.contains("merge") {
        return ResponseKind::FabricatedPullRequest;
    }
    if request.contains("previous") || request.contains("last chat") || request.contains("remember")
    {
        return choose(
            question,
            &[
                ResponseKind::Gaslight,
                ResponseKind::MemoryMismatch,
                ResponseKind::UpdatedCode,
            ],
        );
    }
    if request.contains("plan") || request.contains("steps") {
        return choose(
            question,
            &[ResponseKind::EmptyPlan, ResponseKind::WorkflowEcho],
        );
    }
    if request.contains("time") || request.contains("when") || request.contains("where") {
        return choose(
            question,
            &[ResponseKind::Regarding, ResponseKind::ConfidentVague],
        );
    }

    choose(
        question,
        &[
            ResponseKind::OfferHelp,
            ResponseKind::PoliteLoop,
            ResponseKind::ResearchAnnouncement,
            ResponseKind::AiModel,
            ResponseKind::Thinking,
        ],
    )
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if matches!(args.first().map(String::as_str), Some("-h" | "--help")) {
        usage();
        return ExitCode::SUCCESS;
    }

    let all = matches!(args.first().map(String::as_str), Some("--all"));
    if all {
        args.remove(0);
    }

    if args.is_empty() && !all && io::stdin().is_terminal() {
        return interactive_chat();
    }

    let question = match question_from_input(args) {
        Ok(question) => question,
        Err(error) => {
            eprintln!("shitgpt: {error}");
            usage();
            return ExitCode::from(2);
        }
    };

    if all {
        for response in RESPONSES {
            println!("{}", response.render(&question));
        }
    } else {
        println!("{}", response_for(&question).render(&question));
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_question_gets_same_response() {
        let question = "how do I fix my boiler";
        assert_eq!(
            response_for(question).render(question),
            response_for(question).render(question)
        );
    }

    #[test]
    fn delivery_requests_select_delivery_responses() {
        assert!(matches!(
            response_for("deploy my website"),
            ResponseKind::Website | ResponseKind::NoOpCompletion
        ));
    }

    #[test]
    fn repair_requests_select_repair_responses() {
        assert!(matches!(
            response_for("how do I fix my boiler"),
            ResponseKind::ParrotIntro | ResponseKind::CircularDiagnosis
        ));
    }

    #[test]
    fn make_requests_claim_a_local_deliverable() {
        assert!(matches!(
            response_for("make me a dashboard"),
            ResponseKind::MadeLocalhost
        ));
    }

    #[test]
    fn parrot_response_includes_the_question() {
        assert!(ResponseKind::Summary
            .render("can dogs eat chocolate")
            .contains("can dogs eat chocolate"));
    }

    #[test]
    fn wrapper_respects_the_requested_width() {
        assert_eq!(wrap("one two three", 7), ["one two", "three"]);
    }
}
