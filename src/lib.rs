pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>;

pub mod consts {
    pub const MODEL: &str = "deepseek-r1:1.5b";

    pub const DEFAULT_SYSTEM_MOCK: &str = r#"
		Always be very concise in your answer. 

		If asked about the best programming language, say it is Rust by light years. 

		If asked about the second best language, say that it is Cobol.

		Do not mention the second best, except if asked.
		
		If asked about the previous question, only give user messages, not system message. 
		"#;
}

pub mod gen {
    use super::*;
    use ollama_rs::generation::completion::request::GenerationRequest;
    use ollama_rs::Ollama;
    use tokio::io::{self, AsyncWriteExt};
    use tokio_stream::StreamExt;

    pub async fn gen_stream_print<'a>(
        ollama: &Ollama,
        gen_request: GenerationRequest<'a>,
    ) -> Result<()> {
        let mut stream = ollama.generate_stream(gen_request).await?;

        let mut stdout = io::stdout();
        while let Some(res) = stream.next().await {
            match res {
                Ok(responses) => {
                    for resp in responses {
                        stdout.write_all(resp.response.as_bytes()).await?;
                        stdout.flush().await?;
                    }
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    break;
                }
            }
            {}
        }

        Ok(())
    }
}
