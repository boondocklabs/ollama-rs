use std::io::{stdin, stdout, Write};

use ollama_rs::{coordinator::Coordinator, generation::chat::ChatMessage, Ollama};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let ollama = Ollama::default();
    let history = vec![];
    let mut coordinator = Coordinator::new(ollama, "granite3.2".to_string(), history);

    let stdin = stdin();
    let mut stdout = stdout();
    loop {
        stdout.write_all(b"\n> ")?;
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;

        let input = input.trim_end();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let resp = coordinator
            .chat(vec![
                ChatMessage::custom("control", "thinking"),
                ChatMessage::user(input.to_string()),
            ])
            .await?;

        println!("{}", resp.message.content);
    }

    Ok(())
}
