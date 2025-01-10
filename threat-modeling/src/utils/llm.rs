// The interface for LLM connection needs a careful design
// It should be a simplified wrapper around langchain
use langchain_rust::{
    chain::{Chain, LLMChainBuilder},
    fmt_message, fmt_placeholder, fmt_template,
    language_models::llm::LLM,
    llm::{openai::{OpenAI, OpenAIModel}, OpenAIConfig},
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::messages::Message,
    template_fstring,
};

// Make sure you set your own API key to the environment variable before running the program.
const OPENAI_API_KEY: &str = env!("OPENAI_API_KEY");

pub async fn query_llm(prompt: &str) -> String {
    let open_ai = OpenAI::default().with_config(OpenAIConfig::default().with_api_key(OPENAI_API_KEY)).with_model(OpenAIModel::Gpt4oMini);
    let resp = open_ai.invoke(prompt).await.unwrap();
    resp
}
