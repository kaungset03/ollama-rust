use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use rust_ollama::{
    consts::{DEFAULT_SYSTEM_MOCK, MODEL},
    gen::gen_stream_print,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    let model = MODEL.to_string();
    let prompt = "What is the difference between AC and DC?".to_string();

    let gen_request = GenerationRequest::new(model, prompt).system(DEFAULT_SYSTEM_MOCK.to_string());

    // let res = ollama.generate(gen_request).await?;

    // println!("---> Res {}", res.response);
    let _ = gen_stream_print(&ollama, gen_request).await;

    Ok(())
}
