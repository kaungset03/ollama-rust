use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use rust_ollama::{
    consts::{DEFAULT_SYSTEM_MOCK, MODEL},
    Result,
};
use tokio::io::{self, AsyncWriteExt};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    let model = MODEL.to_string();
    let prompt = "What is the best programming language?";

    let gen_request = GenerationRequest::new(model, prompt).system(DEFAULT_SYSTEM_MOCK.to_string());

    // let res = ollama.generate(gen_request).await?;

    // println!("---> Res {}", res.response);
    let _ = gen_stream_print(&ollama, gen_request).await;

    Ok(())
}

pub async fn gen_stream_print<'a>(ollama: &Ollama, gen_request: GenerationRequest<'a>) -> Result<()> {
    let mut stream = ollama
        .generate_stream(gen_request)
        .await
        .unwrap();

    let mut stdout = io::stdout();
    while let Some(res) = stream.next().await {
        let responses = res.unwrap();
        for resp in responses {
            stdout.write_all(resp.response.as_bytes()).await.unwrap();
            stdout.flush().await.unwrap();
        }
    }

    Ok(())
}
