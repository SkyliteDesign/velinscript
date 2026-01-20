
pub struct NlpStdlib;

impl NlpStdlib {
    pub fn generate_tokenize_code(text: &str) -> String {
        format!(
            "{{
                let text_str = {};
                text_str.split_whitespace().map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string()).filter(|s| !s.is_empty()).collect::<Vec<String>>()
            }}",
            text
        )
    }

    pub fn generate_sentiment_code(text: &str) -> String {
        format!(
            "{{
                let text_str = {};
                let positive_words = vec![\"good\", \"great\", \"excellent\", \"wonderful\", \"amazing\", \"love\", \"happy\", \"fantastic\"];
                let negative_words = vec![\"bad\", \"terrible\", \"awful\", \"hate\", \"horrible\", \"sad\", \"angry\", \"disappointed\"];
                let lower_text = text_str.to_lowercase();
                let positive_count = positive_words.iter().filter(|w| lower_text.contains(w)).count();
                let negative_count = negative_words.iter().filter(|w| lower_text.contains(w)).count();
                if positive_count > negative_count {{
                    \"positive\"
                }} else if negative_count > positive_count {{
                    \"negative\"
                }} else {{
                    \"neutral\"
                }}
            }}",
            text
        )
    }

    pub fn generate_ner_code(text: &str) -> String {
        format!(
            "{{
                let text_str = {};
                let mut entities = Vec::new();
                // Simple email detection
                let email_re = regex::Regex::new(r\"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{{2,}}\").unwrap();
                for cap in email_re.find_iter(&text_str) {{
                    entities.push(serde_json::json!({{
                        \"text\": cap.as_str(),
                        \"type\": \"EMAIL\",
                        \"start\": cap.start(),
                        \"end\": cap.end()
                    }}));
                }}
                // Simple phone detection
                let phone_re = regex::Regex::new(r\"\\+?[0-9]{{10,}}\").unwrap();
                for cap in phone_re.find_iter(&text_str) {{
                    entities.push(serde_json::json!({{
                        \"text\": cap.as_str(),
                        \"type\": \"PHONE\",
                        \"start\": cap.start(),
                        \"end\": cap.end()
                    }}));
                }}
                entities
            }}",
            text
        )
    }

    pub fn generate_keywords_code(text: &str, count: &str) -> String {
        format!(
            "{{
                use std::collections::HashMap;
                let text_str = {};
                let count_num = {} as usize;
                let stop_words = vec![\"the\", \"a\", \"an\", \"and\", \"or\", \"but\", \"in\", \"on\", \"at\", \"to\", \"for\", \"of\", \"with\", \"by\"];
                let words: Vec<String> = text_str.to_lowercase().split_whitespace()
                    .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
                    .filter(|s| !s.is_empty() && !stop_words.contains(&s.as_str()))
                    .collect();
                let mut word_count: HashMap<String, usize> = HashMap::new();
                for word in words {{
                    *word_count.entry(word).or_insert(0) += 1;
                }}
                let mut sorted_words: Vec<(String, usize)> = word_count.into_iter().collect();
                sorted_words.sort_by(|a, b| b.1.cmp(&a.1));
                sorted_words.into_iter().take(count_num).map(|(word, _)| word).collect::<Vec<String>>()
            }}",
            text, count
        )
    }

    pub fn generate_similarity_code(text1: &str, text2: &str) -> String {
        format!(
            "{{
                let text1_str = {};
                let text2_str = {};
                let words1: std::collections::HashSet<String> = text1_str.to_lowercase().split_whitespace().map(|s| s.to_string()).collect();
                let words2: std::collections::HashSet<String> = text2_str.to_lowercase().split_whitespace().map(|s| s.to_string()).collect();
                let intersection: std::collections::HashSet<_> = words1.intersection(&words2).cloned().collect();
                let union: std::collections::HashSet<_> = words1.union(&words2).cloned().collect();
                if union.is_empty() {{
                    0.0
                }} else {{
                    intersection.len() as f64 / union.len() as f64
                }}
            }}",
            text1, text2
        )
    }

    pub fn generate_summarize_code(text: &str, sentences: &str) -> String {
        format!(
            "{{
                let text_str = {};
                let sentences_count = {} as usize;
                let sentences: Vec<&str> = text_str.split('.').filter(|s| !s.trim().is_empty()).collect();
                sentences.into_iter().take(sentences_count).map(|s| s.trim().to_string()).collect::<Vec<String>>().join(\". \")
            }}",
            text, sentences
        )
    }
}
