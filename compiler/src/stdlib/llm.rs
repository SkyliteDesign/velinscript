
pub struct LLMStdlib;

impl LLMStdlib {
    pub fn generate_summarize_code(client: &str, text: &str) -> String {
        format!("{}.generate(&format!(\"Summarize the following text:\\n\\n{{}}\", {}))", client, text)
    }

    pub fn generate_classify_code(client: &str, text: &str, categories: &str) -> String {
        format!(
            "{}.generate(&format!(\"Classify the following text into one of these categories: {{:?}}.\\n\\nText: {{}}\\n\\nCategory:\", {}, {}))",
            client, categories, text
        )
    }

    pub fn generate_extract_entities_code(client: &str, text: &str) -> String {
        format!(
            "{}.generate(&format!(\"Extract named entities from the following text as JSON list of objects with 'name' and 'type' fields:\\n\\n{{}}\", {}))",
            client, text
        )
    }

    pub fn generate_generate_code(client: &str, title: &str, style: Option<&str>) -> String {
        if let Some(s) = style {
            format!(
                "{}.generate(&format!(\"Write a text about '{{}}' in the style of '{{}}'.\", {}, {}))",
                client, title, s
            )
        } else {
            format!(
                "{}.generate(&format!(\"Write a text about '{{}}'.\", {}))",
                client, title
            )
        }
    }

    pub fn generate_translate_code(client: &str, text: &str, target_lang: &str) -> String {
        format!(
            "{}.generate(&format!(\"Translate the following text to {{}}:\\n\\n{{}}\", {}, {}))",
            client, target_lang, text
        )
    }

    pub fn generate_sentiment_code(client: &str, text: &str) -> String {
        format!(
            "{}.generate(&format!(\"Analyze the sentiment of the following text (positive, negative, neutral):\\n\\n{{}}\", {}))",
            client, text
        )
    }

    pub fn generate_complete_code(client: &str, prompt: &str, _max_tokens: Option<&str>) -> String {
        // max_tokens is ignored in current LLMClient.generate signature, but could be added if extended
        format!("{}.generate({})", client, prompt)
    }

    pub fn generate_embed_code(client: &str, text: &str) -> String {
        format!("{}.embed({})", client, text)
    }

    pub fn generate_chat_code(client: &str, messages: &str) -> String {
        // Assuming messages is a JSON-serializable structure or similar
        format!(
            "{}.generate(&format!(\"Chat history:\\n{{:?}}\\n\\nContinue the conversation.\", {}))",
            client, messages
        )
    }
}
