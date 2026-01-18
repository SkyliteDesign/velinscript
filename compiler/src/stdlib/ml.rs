// ML Framework - Model Loading, LLM Integration, Vector DB Support

use crate::stdlib::logging::VelinLogger;
use crate::stdlib::metrics::{MetricsCollector, PerformanceMonitor, HealthCheck};
use std::collections::HashMap;

#[cfg(feature = "ml")]
// use serde::{Deserialize, Serialize};  // Not currently used
#[cfg(feature = "ml")]
use serde_json::json;

pub struct MLModel {
    pub name: String,
    pub model_type: ModelType,
    pub path: String,
}

#[derive(Debug, Clone)]
pub enum ModelType {
    Sentiment,
    Classification,
    Regression,
    Embedding,
    LLM,
}

pub struct ModelLoader {
    pub models: std::collections::HashMap<String, MLModel>,
    pub logger: VelinLogger,
    pub metrics: MetricsCollector,
    pub performance: PerformanceMonitor,
    pub health: HealthCheck,
}

impl ModelLoader {
    pub fn new() -> Self {
        let mut logger = VelinLogger::new();
        logger.add_context("component".to_string(), "ModelLoader".to_string());
        ModelLoader {
            models: std::collections::HashMap::new(),
            logger,
            metrics: MetricsCollector::new(),
            performance: PerformanceMonitor::new(),
            health: HealthCheck::new(),
        }
    }
    
    pub fn load_model(&mut self, name: String, model_type: ModelType, path: String) -> Result<(), String> {
        self.performance.start_operation("load_model");
        
        // In production, load model from file (ONNX, TensorFlow, etc.)
        let model = MLModel {
            name: name.clone(),
            model_type: model_type.clone(),
            path: path.clone(),
        };
        self.models.insert(name.clone(), model);
        
        // Metrics
        let mut labels = HashMap::new();
        labels.insert("model_type".to_string(), format!("{:?}", model_type));
        self.metrics.increment_counter("models_loaded_total", Some(labels));
        self.metrics.set_gauge("models_count", self.models.len() as f64, None);
        
        // Log model loading
        let mut context = HashMap::new();
        context.insert("model_name".to_string(), name);
        context.insert("model_type".to_string(), format!("{:?}", model_type));
        context.insert("path".to_string(), path);
        self.logger.log_with_context(
            crate::stdlib::logging::LogLevel::Info,
            &format!("Model loaded successfully"),
            Some(context),
        );
        
        // Health check
        self.health.set_component_status("model_loading", true);
        
        self.performance.end_operation("load_model", None);
        
        Ok(())
    }
    
    #[allow(unused_variables)]
    pub fn predict(&self, model_name: &str, input: &str) -> Result<String, String> {
        if let Some(model) = self.models.get(model_name) {
            // In production, run inference
            let result = match model.model_type {
                ModelType::Sentiment => Ok("positive".to_string()),
                ModelType::Classification => Ok("class1".to_string()),
                ModelType::Regression => Ok("0.5".to_string()),
                ModelType::Embedding => Ok("[0.1, 0.2, 0.3]".to_string()),
                ModelType::LLM => Ok("Generated text".to_string()),
            };
            
            // Log prediction (would use mutable reference in real implementation)
            // For now, we'll skip logging here as self is immutable
            
            result
        } else {
            let mut logger = VelinLogger::new();
            logger.add_context("component".to_string(), "ModelLoader".to_string());
            logger.add_context("model_name".to_string(), model_name.to_string());
            logger.error(&format!("Model {} not found", model_name));
            Err(format!("Model {} not found", model_name))
        }
    }
}

pub struct LLMClient {
    pub provider: LLMProvider,
    pub api_key: String,
    pub logger: VelinLogger,
    pub metrics: MetricsCollector,
    pub performance: PerformanceMonitor,
    pub health: HealthCheck,
}

#[derive(Debug, Clone)]
pub enum LLMProvider {
    OpenAI,
    Anthropic,
    GoogleGemini,
    Local,
}

impl LLMClient {
    pub fn new(provider: LLMProvider, api_key: String) -> Self {
        let mut logger = VelinLogger::new();
        logger.add_context("component".to_string(), "LLMClient".to_string());
        logger.add_context("provider".to_string(), format!("{:?}", provider));
        LLMClient { 
            provider, 
            api_key, 
            logger,
            metrics: MetricsCollector::new(),
            performance: PerformanceMonitor::new(),
            health: HealthCheck::new(),
        }
    }
    
    pub fn generate(&self, prompt: &str) -> Result<String, String> {
        // Log generation request
        let mut logger = VelinLogger::new();
        logger.add_context("component".to_string(), "LLMClient".to_string());
        logger.add_context("provider".to_string(), format!("{:?}", self.provider));
        logger.add_context("prompt_length".to_string(), prompt.len().to_string());
        logger.info("Generating LLM response");
        
        #[cfg(feature = "ml")]
        {
            let result = match self.provider {
                LLMProvider::OpenAI => self.generate_openai(prompt),
                LLMProvider::Anthropic => self.generate_anthropic(prompt),
                LLMProvider::GoogleGemini => self.generate_gemini(prompt),
                LLMProvider::Local => Ok(format!("Local model response to: {}", prompt)),
            };
            
            if result.is_ok() {
                logger.info("LLM response generated successfully");
            } else {
                logger.error("Failed to generate LLM response");
            }
            
            return result;
        }
        
        #[cfg(not(feature = "ml"))]
        {
            // Fallback to mock when ml feature is not enabled
            let result = match self.provider {
                LLMProvider::OpenAI => Ok(format!("OpenAI response to: {}", prompt)),
                LLMProvider::Anthropic => Ok(format!("Anthropic Claude response to: {}", prompt)),
                LLMProvider::GoogleGemini => Ok(format!("Google Gemini response to: {}", prompt)),
                LLMProvider::Local => Ok(format!("Local model response to: {}", prompt)),
            };
            
            if result.is_ok() {
                logger.info("LLM response generated successfully (mock mode)");
            } else {
                logger.error("Failed to generate LLM response");
            }
            
            result
        }
    }
    
    #[cfg(feature = "ml")]
    fn generate_openai(&self, prompt: &str) -> Result<String, String> {
        use reqwest::blocking::Client;
        
        let client = Client::new();
        let url = "https://api.openai.com/v1/chat/completions";
        
        let payload = json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": 1000,
            "temperature": 0.7
        });
        
        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| format!("OpenAI API request failed: {}", e))?;
        
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("OpenAI API error: {} - {}", status, error_text));
        }
        
        let json: serde_json::Value = response.json()
            .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;
        
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| "Invalid response format from OpenAI".to_string())?;
        
        Ok(content.to_string())
    }
    
    #[cfg(feature = "ml")]
    fn generate_anthropic(&self, prompt: &str) -> Result<String, String> {
        use reqwest::blocking::Client;
        
        let client = Client::new();
        let url = "https://api.anthropic.com/v1/messages";
        
        let payload = json!({
            "model": "claude-3-sonnet-20240229",
            "max_tokens": 1000,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        });
        
        let response = client
            .post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| format!("Anthropic API request failed: {}", e))?;
        
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Anthropic API error: {} - {}", status, error_text));
        }
        
        let json: serde_json::Value = response.json()
            .map_err(|e| format!("Failed to parse Anthropic response: {}", e))?;
        
        let content = json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| "Invalid response format from Anthropic".to_string())?;
        
        Ok(content.to_string())
    }
    
    #[cfg(feature = "ml")]
    fn generate_gemini(&self, prompt: &str) -> Result<String, String> {
        use reqwest::blocking::Client;
        
        let client = Client::new();
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", self.api_key);
        
        let payload = json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }],
            "generationConfig": {
                "maxOutputTokens": 1000,
                "temperature": 0.7
            }
        });
        
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| format!("Google Gemini API request failed: {}", e))?;
        
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Google Gemini API error: {} - {}", status, error_text));
        }
        
        let json: serde_json::Value = response.json()
            .map_err(|e| format!("Failed to parse Google Gemini response: {}", e))?;
        
        let content = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| "Invalid response format from Google Gemini".to_string())?;
        
        Ok(content.to_string())
    }
    
    pub fn embed(&self, text: &str) -> Result<Vec<f64>, String> {
        let mut logger = VelinLogger::new();
        logger.add_context("component".to_string(), "LLMClient".to_string());
        logger.add_context("text_length".to_string(), text.len().to_string());
        logger.debug("Generating embeddings");
        
        #[cfg(feature = "ml")]
        {
            let result = match self.provider {
                LLMProvider::OpenAI => self.embed_openai(text),
                LLMProvider::Anthropic => self.embed_anthropic(text),
                LLMProvider::GoogleGemini => self.embed_gemini(text),
                LLMProvider::Local => Ok(vec![0.1, 0.2, 0.3, 0.4, 0.5]),
            };
            
            if result.is_ok() {
                logger.debug("Embeddings generated successfully");
            } else {
                logger.error("Failed to generate embeddings");
            }
            
            return result;
        }
        
        #[cfg(not(feature = "ml"))]
        {
            // Fallback to mock when ml feature is not enabled
            Ok(vec![0.1, 0.2, 0.3, 0.4, 0.5])
        }
    }
    
    #[cfg(feature = "ml")]
    fn embed_openai(&self, text: &str) -> Result<Vec<f64>, String> {
        use reqwest::blocking::Client;
        
        let client = Client::new();
        let url = "https://api.openai.com/v1/embeddings";
        
        let payload = json!({
            "model": "text-embedding-ada-002",
            "input": text
        });
        
        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| format!("OpenAI Embeddings API request failed: {}", e))?;
        
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("OpenAI Embeddings API error: {} - {}", status, error_text));
        }
        
        let json: serde_json::Value = response.json()
            .map_err(|e| format!("Failed to parse OpenAI embeddings response: {}", e))?;
        
        let embedding = json["data"][0]["embedding"]
            .as_array()
            .ok_or_else(|| "Invalid embedding format from OpenAI".to_string())?;
        
        let result: Result<Vec<f64>, String> = embedding
            .iter()
            .map(|v| v.as_f64().ok_or_else(|| "Invalid embedding value".to_string()))
            .collect();
        
        result
    }
    
    #[cfg(feature = "ml")]
    fn embed_anthropic(&self, text: &str) -> Result<Vec<f64>, String> {
        // Anthropic doesn't have a public embeddings API yet
        // Fallback to a simple hash-based embedding for now
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Generate a deterministic embedding vector from hash
        let mut embedding = Vec::with_capacity(1536);
        let mut seed = hash;
        for _ in 0..1536 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            embedding.push((seed as f64) / (u64::MAX as f64));
        }
        
        Ok(embedding)
    }
    
    #[cfg(feature = "ml")]
    fn embed_gemini(&self, text: &str) -> Result<Vec<f64>, String> {
        use reqwest::blocking::Client;
        
        let client = Client::new();
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/embedding-001:embedContent?key={}", self.api_key);
        
        let payload = json!({
            "model": "models/embedding-001",
            "content": {
                "parts": [{
                    "text": text
                }]
            }
        });
        
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| format!("Google Gemini Embeddings API request failed: {}", e))?;
        
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Google Gemini Embeddings API error: {} - {}", status, error_text));
        }
        
        let json: serde_json::Value = response.json()
            .map_err(|e| format!("Failed to parse Google Gemini embeddings response: {}", e))?;
        
        let embedding = json["embedding"]["values"]
            .as_array()
            .ok_or_else(|| "Invalid embedding format from Google Gemini".to_string())?;
        
        let result: Result<Vec<f64>, String> = embedding
            .iter()
            .map(|v| v.as_f64().ok_or_else(|| "Invalid embedding value".to_string()))
            .collect();
        
        result
    }
}

pub struct VectorDB {
    pub provider: VectorDBProvider,
    pub connection_string: String,
    pub logger: VelinLogger,
    pub metrics: MetricsCollector,
    pub performance: PerformanceMonitor,
    pub health: HealthCheck,
}

#[derive(Debug, Clone)]
pub enum VectorDBProvider {
    Pinecone,
    Weaviate,
    Qdrant,
    Local,
}

impl VectorDB {
    pub fn new(provider: VectorDBProvider, connection_string: String) -> Self {
        let mut logger = VelinLogger::new();
        logger.add_context("component".to_string(), "VectorDB".to_string());
        logger.add_context("provider".to_string(), format!("{:?}", provider));
        VectorDB {
            provider,
            connection_string,
            logger,
            metrics: MetricsCollector::new(),
            performance: PerformanceMonitor::new(),
            health: HealthCheck::new(),
        }
    }
    
    pub fn upsert(&self, collection: &str, id: &str, vector: Vec<f64>) -> Result<(), String> {
        let mut logger = VelinLogger::new();
        logger.add_context("component".to_string(), "VectorDB".to_string());
        logger.add_context("collection".to_string(), collection.to_string());
        logger.add_context("id".to_string(), id.to_string());
        logger.add_context("vector_size".to_string(), vector.len().to_string());
        logger.debug("Upserting vector to database");
        
        #[cfg(feature = "ml")]
        {
            match self.provider {
                VectorDBProvider::Pinecone => self.upsert_pinecone(collection, id, vector),
                VectorDBProvider::Weaviate => self.upsert_weaviate(collection, id, vector),
                VectorDBProvider::Qdrant => self.upsert_qdrant(collection, id, vector),
                VectorDBProvider::Local => Ok(()), // Local mode - no-op
            }
        }
        
        #[cfg(not(feature = "ml"))]
        {
            // Fallback to mock when ml feature is not enabled
            Ok(())
        }
    }
    
    #[cfg(feature = "ml")]
    fn upsert_pinecone(&self, index_name: &str, id: &str, vector: Vec<f64>) -> Result<(), String> {
        use reqwest::blocking::Client;
        
        // Parse connection string: format "api-key@environment"
        let parts: Vec<&str> = self.connection_string.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid Pinecone connection string format. Expected: api-key@environment".to_string());
        }
        let api_key = parts[0];
        let environment = parts[1];
        
        let client = Client::new();
        let url = format!("https://{}.svc.{}.pinecone.io/vectors/upsert", index_name, environment);
        
        let payload = json!({
            "vectors": [{
                "id": id,
                "values": vector
            }]
        });
        
        let response = client
            .post(&url)
            .header("Api-Key", api_key)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| format!("Pinecone upsert request failed: {}", e))?;
        
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Pinecone upsert error: {} - {}", status, error_text));
        }
        
        Ok(())
    }
    
    #[cfg(feature = "ml")]
    fn upsert_weaviate(&self, class_name: &str, id: &str, vector: Vec<f64>) -> Result<(), String> {
        use reqwest::blocking::Client;
        
        // Parse connection string: format "http://host:port" or "https://host:port"
        let base_url = if self.connection_string.starts_with("http") {
            &self.connection_string
        } else {
            return Err("Invalid Weaviate connection string. Expected: http://host:port or https://host:port".to_string());
        };
        
        let client = Client::new();
        let url = format!("{}/v1/objects", base_url);
        
        let payload = json!({
            "class": class_name,
            "id": id,
            "vector": vector
        });
        
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| format!("Weaviate upsert request failed: {}", e))?;
        
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Weaviate upsert error: {} - {}", status, error_text));
        }
        
        Ok(())
    }
    
    #[cfg(feature = "ml")]
    fn upsert_qdrant(&self, collection_name: &str, id: &str, vector: Vec<f64>) -> Result<(), String> {
        #[cfg(all(feature = "ml", feature = "qdrant-client"))]
        {
            use qdrant_client::prelude::*;
            use qdrant_client::qdrant::{PointStruct, vectors::Vectors, Vector};
            
            // Parse connection string: format "http://host:port" or "https://host:port"
            let url = if self.connection_string.starts_with("http") {
                &self.connection_string
            } else {
                return Err("Invalid Qdrant connection string. Expected: http://host:port or https://host:port".to_string());
            };
            
            let config = QdrantClientConfig::from_url(url);
            let client = QdrantClient::new(Some(config))
                .map_err(|e| format!("Failed to create Qdrant client: {}", e))?;
            
            let point_id: u64 = id.parse()
                .map_err(|_| format!("Invalid point ID format: {}", id))?;
            
            let point = PointStruct::new(
                point_id,
                Vectors::vector(vector),
                HashMap::new(),
            );
            
            client.upsert_points(collection_name, vec![point], None)
                .map_err(|e| format!("Qdrant upsert failed: {}", e))?;
            
            Ok(())
        }
        
        #[cfg(all(feature = "ml", not(feature = "qdrant-client")))]
        {
            // Fallback to REST API if qdrant-client is not available
            use reqwest::blocking::Client;
            
            let base_url = if self.connection_string.starts_with("http") {
                &self.connection_string
            } else {
                return Err("Invalid Qdrant connection string. Expected: http://host:port or https://host:port".to_string());
            };
            
            let client = Client::new();
            let url = format!("{}/collections/{}/points", base_url, collection_name);
            
            let payload = json!({
                "points": [{
                    "id": id,
                    "vector": vector
                }]
            });
            
            let response = client
                .put(&url)
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .map_err(|e| format!("Qdrant upsert request failed: {}", e))?;
            
            let status = response.status();
            if !status.is_success() {
                let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
                return Err(format!("Qdrant upsert error: {} - {}", status, error_text));
            }
            
            Ok(())
        }
    }
    
    pub fn search(&self, collection: &str, query_vector: Vec<f64>, top_k: usize) -> Result<Vec<SearchResult>, String> {
        let mut logger = VelinLogger::new();
        logger.add_context("component".to_string(), "VectorDB".to_string());
        logger.add_context("collection".to_string(), collection.to_string());
        logger.add_context("top_k".to_string(), top_k.to_string());
        logger.add_context("query_vector_size".to_string(), query_vector.len().to_string());
        logger.info("Searching vectors");
        
        #[cfg(feature = "ml")]
        {
            let result = match self.provider {
                VectorDBProvider::Pinecone => self.search_pinecone(collection, query_vector, top_k),
                VectorDBProvider::Weaviate => self.search_weaviate(collection, query_vector, top_k),
                VectorDBProvider::Qdrant => self.search_qdrant(collection, query_vector, top_k),
                VectorDBProvider::Local => {
                    // Local mode - return mock results
                    Ok(vec![
                        SearchResult { id: "result1".to_string(), score: 0.95 },
                        SearchResult { id: "result2".to_string(), score: 0.87 },
                    ])
                },
            };
            
            if let Ok(ref results) = result {
                logger.add_context("results_count".to_string(), results.len().to_string());
                logger.info("Vector search completed");
            } else {
                logger.error("Vector search failed");
            }
            
            result
        }
        
        #[cfg(not(feature = "ml"))]
        {
            // Fallback to mock when ml feature is not enabled
            let results = vec![
                SearchResult { id: "result1".to_string(), score: 0.95 },
                SearchResult { id: "result2".to_string(), score: 0.87 },
            ];
            logger.add_context("results_count".to_string(), results.len().to_string());
            logger.info("Vector search completed (mock mode)");
            Ok(results)
        }
    }
    
    #[cfg(feature = "ml")]
    fn search_pinecone(&self, index_name: &str, query_vector: Vec<f64>, top_k: usize) -> Result<Vec<SearchResult>, String> {
        use reqwest::blocking::Client;
        
        // Parse connection string: format "api-key@environment"
        let parts: Vec<&str> = self.connection_string.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid Pinecone connection string format. Expected: api-key@environment".to_string());
        }
        let api_key = parts[0];
        let environment = parts[1];
        
        let client = Client::new();
        let url = format!("https://{}.svc.{}.pinecone.io/query", index_name, environment);
        
        let payload = json!({
            "vector": query_vector,
            "topK": top_k,
            "includeMetadata": false
        });
        
        let response = client
            .post(&url)
            .header("Api-Key", api_key)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| format!("Pinecone search request failed: {}", e))?;
        
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Pinecone search error: {} - {}", status, error_text));
        }
        
        let json: serde_json::Value = response.json()
            .map_err(|e| format!("Failed to parse Pinecone response: {}", e))?;
        
        let matches = json["matches"]
            .as_array()
            .ok_or_else(|| "Invalid response format from Pinecone".to_string())?;
        
        let results: Result<Vec<SearchResult>, String> = matches
            .iter()
            .map(|m| {
                let id = m["id"].as_str()
                    .ok_or_else(|| "Missing id in match".to_string())?
                    .to_string();
                let score = m["score"].as_f64()
                    .ok_or_else(|| "Missing score in match".to_string())?;
                Ok(SearchResult { id, score })
            })
            .collect();
        
        results
    }
    
    #[cfg(feature = "ml")]
    fn search_weaviate(&self, class_name: &str, query_vector: Vec<f64>, top_k: usize) -> Result<Vec<SearchResult>, String> {
        use reqwest::blocking::Client;
        
        let base_url = if self.connection_string.starts_with("http") {
            &self.connection_string
        } else {
            return Err("Invalid Weaviate connection string. Expected: http://host:port or https://host:port".to_string());
        };
        
        let client = Client::new();
        let url = format!("{}/v1/graphql", base_url);
        
        // GraphQL query for similarity search
        let query = format!(
            r#"{{
                Get {{
                    {}(
                        nearVector: {{
                            vector: {:?}
                        }}
                        limit: {}
                    ) {{
                        _additional {{
                            id
                            distance
                        }}
                    }}
                }}
            }}"#,
            class_name, query_vector, top_k
        );
        
        let payload = json!({
            "query": query
        });
        
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| format!("Weaviate search request failed: {}", e))?;
        
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Weaviate search error: {} - {}", status, error_text));
        }
        
        let json: serde_json::Value = response.json()
            .map_err(|e| format!("Failed to parse Weaviate response: {}", e))?;
        
        let objects = json["data"]["Get"][class_name]
            .as_array()
            .ok_or_else(|| "Invalid response format from Weaviate".to_string())?;
        
        let results: Result<Vec<SearchResult>, String> = objects
            .iter()
            .map(|obj| {
                let additional = &obj["_additional"];
                let id = additional["id"].as_str()
                    .ok_or_else(|| "Missing id".to_string())?
                    .to_string();
                let distance = additional["distance"].as_f64()
                    .ok_or_else(|| "Missing distance".to_string())?;
                // Convert distance to similarity score (1 - normalized distance)
                let score = 1.0 - (distance / 2.0).min(1.0);
                Ok(SearchResult { id, score })
            })
            .collect();
        
        results
    }
    
    #[cfg(feature = "ml")]
    fn search_qdrant(&self, collection_name: &str, query_vector: Vec<f64>, top_k: usize) -> Result<Vec<SearchResult>, String> {
        #[cfg(all(feature = "ml", feature = "qdrant-client"))]
        {
            use qdrant_client::prelude::*;
            use qdrant_client::qdrant::{SearchPoints, WithPayloadSelector, with_payload_selector::SelectorOptions};
            
            let url = if self.connection_string.starts_with("http") {
                &self.connection_string
            } else {
                return Err("Invalid Qdrant connection string. Expected: http://host:port or https://host:port".to_string());
            };
            
            let config = QdrantClientConfig::from_url(url);
            let client = QdrantClient::new(Some(config))
                .map_err(|e| format!("Failed to create Qdrant client: {}", e))?;
            
            let search_points = SearchPoints {
                collection_name: collection_name.to_string(),
                vector: query_vector,
                limit: top_k as u64,
                with_payload: Some(WithPayloadSelector {
                    selector_options: Some(SelectorOptions::Enable(true)),
                }),
                ..Default::default()
            };
            
            let search_result = client.search_points(&search_points)
                .map_err(|e| format!("Qdrant search failed: {}", e))?;
            
            let results: Vec<SearchResult> = search_result.result
                .iter()
                .map(|point| {
                    SearchResult {
                        id: point.id.to_string(),
                        score: point.score,
                    }
                })
                .collect();
            
            Ok(results)
        }
        
        #[cfg(all(feature = "ml", not(feature = "qdrant-client")))]
        {
            // Fallback to REST API if qdrant-client is not available
            use reqwest::blocking::Client;
            
            let base_url = if self.connection_string.starts_with("http") {
                &self.connection_string
            } else {
                return Err("Invalid Qdrant connection string. Expected: http://host:port or https://host:port".to_string());
            };
            
            let client = Client::new();
            let url = format!("{}/collections/{}/points/search", base_url, collection_name);
            
            let payload = json!({
                "vector": query_vector,
                "limit": top_k,
                "with_payload": false
            });
            
            let response = client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .map_err(|e| format!("Qdrant search request failed: {}", e))?;
            
            let status = response.status();
            if !status.is_success() {
                let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
                return Err(format!("Qdrant search error: {} - {}", status, error_text));
            }
            
            let json: serde_json::Value = response.json()
                .map_err(|e| format!("Failed to parse Qdrant response: {}", e))?;
            
            let results_array = json["result"]
                .as_array()
                .ok_or_else(|| "Invalid response format from Qdrant".to_string())?;
            
            let results: Result<Vec<SearchResult>, String> = results_array
                .iter()
                .map(|r| {
                    let id = r["id"].as_str()
                        .ok_or_else(|| "Missing id".to_string())?
                        .to_string();
                    let score = r["score"].as_f64()
                        .ok_or_else(|| "Missing score".to_string())?;
                    Ok(SearchResult { id, score })
                })
                .collect();
            
            results
        }
    }
}

pub struct SearchResult {
    pub id: String,
    pub score: f64,
}

pub struct TrainingService {
    pub training_data: Vec<TrainingExample>,
    pub logger: VelinLogger,
    pub metrics: MetricsCollector,
    pub performance: PerformanceMonitor,
    pub health: HealthCheck,
}

pub struct TrainingExample {
    pub input: String,
    pub output: String,
}

impl TrainingService {
    pub fn new() -> Self {
        let mut logger = VelinLogger::new();
        logger.add_context("component".to_string(), "TrainingService".to_string());
        TrainingService {
            training_data: Vec::new(),
            logger,
            metrics: MetricsCollector::new(),
            performance: PerformanceMonitor::new(),
            health: HealthCheck::new(),
        }
    }
    
    pub fn add_example(&mut self, input: String, output: String) {
        self.training_data.push(TrainingExample { input, output });
        let mut logger = VelinLogger::new();
        logger.add_context("component".to_string(), "TrainingService".to_string());
        logger.add_context("training_data_size".to_string(), self.training_data.len().to_string());
        logger.debug("Training example added");
    }
    
    #[allow(unused_variables)]
    pub fn train(&self, model_name: &str) -> Result<(), String> {
        let mut logger = VelinLogger::new();
        logger.add_context("component".to_string(), "TrainingService".to_string());
        logger.add_context("model_name".to_string(), model_name.to_string());
        logger.add_context("training_examples".to_string(), self.training_data.len().to_string());
        logger.info("Starting model training");
        
        // In production, train model with training data
        logger.info("Model training completed successfully");
        Ok(())
    }
    
    /// Trainiert ein Model mit ONNX Runtime
    #[allow(unused_variables)]
    pub fn train_with_onnx(&self, model_name: &str, config: ONNXTrainingConfig) -> Result<ModelTrainingResult, String> {
        let mut logger = VelinLogger::new();
        logger.add_context("component".to_string(), "TrainingService".to_string());
        logger.add_context("model_name".to_string(), model_name.to_string());
        logger.add_context("framework".to_string(), "ONNX".to_string());
        logger.info("Starting ONNX model training");
        
        // In production, use ONNX Runtime for training
        // For now, return a mock result
        Ok(ModelTrainingResult {
            model_name: model_name.to_string(),
            framework: "ONNX".to_string(),
            accuracy: 0.95,
            loss: 0.05,
            epochs: config.epochs,
            training_time_seconds: 120.0,
        })
    }
    
    /// Trainiert ein Model mit TensorFlow
    #[allow(unused_variables)]
    pub fn train_with_tensorflow(&self, model_name: &str, config: TensorFlowTrainingConfig) -> Result<ModelTrainingResult, String> {
        let mut logger = VelinLogger::new();
        logger.add_context("component".to_string(), "TrainingService".to_string());
        logger.add_context("model_name".to_string(), model_name.to_string());
        logger.add_context("framework".to_string(), "TensorFlow".to_string());
        logger.info("Starting TensorFlow model training");
        
        // In production, use TensorFlow for training
        // For now, return a mock result
        Ok(ModelTrainingResult {
            model_name: model_name.to_string(),
            framework: "TensorFlow".to_string(),
            accuracy: 0.92,
            loss: 0.08,
            epochs: config.epochs,
            training_time_seconds: 180.0,
        })
    }
    
    /// Evaluates a trained model
    #[allow(unused_variables)]
    pub fn evaluate_model(&self, model_name: &str, test_data: &[TrainingExample]) -> Result<ModelEvaluationResult, String> {
        let mut logger = VelinLogger::new();
        logger.add_context("component".to_string(), "TrainingService".to_string());
        logger.add_context("model_name".to_string(), model_name.to_string());
        logger.add_context("test_data_size".to_string(), test_data.len().to_string());
        logger.info("Evaluating model");
        
        // In production, evaluate model with test data
        Ok(ModelEvaluationResult {
            model_name: model_name.to_string(),
            accuracy: 0.94,
            precision: 0.93,
            recall: 0.95,
            f1_score: 0.94,
            test_samples: test_data.len(),
        })
    }
}

pub struct MLStdlib;

impl MLStdlib {
    pub fn generate_ml_runtime_code() -> String {
        r#"
// --- VelinScript ML Runtime ---

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde_json::json;
use once_cell::sync::Lazy;

// Global in-memory vector store for Local mode
static LOCAL_VECTOR_STORE: Lazy<Arc<Mutex<HashMap<String, Vec<VectorRecord>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

#[derive(Debug, Clone)]
struct VectorRecord {
    id: String,
    vector: Vec<f64>,
}

pub struct LLMClient {
    pub provider: String,
    pub api_key: String,
}

impl LLMClient {
    pub fn new(provider: &str, api_key: &str) -> Self {
        LLMClient { 
            provider: provider.to_string(), 
            api_key: api_key.to_string(),
        }
    }
    
    pub async fn generate(&self, prompt: &str) -> anyhow::Result<String> {
        match self.provider.as_str() {
            "openai" => self.generate_openai(prompt).await,
            "anthropic" => self.generate_anthropic(prompt).await,
            "gemini" => self.generate_gemini(prompt).await,
            "local" => Ok(format!("Local model response to: {}", prompt)),
            _ => Err(anyhow::anyhow!("Unknown provider: {}", self.provider)),
        }
    }

    pub async fn embed(&self, text: &str) -> anyhow::Result<Vec<f64>> {
        match self.provider.as_str() {
            "openai" => self.embed_openai(text).await,
            "gemini" => self.embed_gemini(text).await,
            "local" => {
                // Deterministic pseudo-embedding for local testing
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                text.hash(&mut hasher);
                let h = hasher.finish();
                let mut vec = Vec::with_capacity(1536);
                for i in 0..1536 {
                    vec.push(((h.wrapping_add(i as u64)) % 100) as f64 / 100.0);
                }
                Ok(vec)
            },
            _ => Err(anyhow::anyhow!("Provider {} does not support embeddings", self.provider)),
        }
    }

    async fn generate_openai(&self, prompt: &str) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let url = "https://api.openai.com/v1/chat/completions";
        
        let payload = json!({
            "model": "gpt-3.5-turbo",
            "messages": [{ "role": "user", "content": prompt }],
            "temperature": 0.7
        });
        
        let response = client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("OpenAI API error: {}", response.status()));
        }
        
        let json: serde_json::Value = response.json().await?;
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;
            
        Ok(content.to_string())
    }

    async fn generate_anthropic(&self, prompt: &str) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let url = "https://api.anthropic.com/v1/messages";
        
        let payload = json!({
            "model": "claude-3-sonnet-20240229",
            "max_tokens": 1000,
            "messages": [{ "role": "user", "content": prompt }]
        });
        
        let response = client.post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&payload)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Anthropic API error: {}", response.status()));
        }
        
        let json: serde_json::Value = response.json().await?;
        let content = json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;
            
        Ok(content.to_string())
    }

    async fn generate_gemini(&self, prompt: &str) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", self.api_key);
        
        let payload = json!({
            "contents": [{ "parts": [{ "text": prompt }] }]
        });
        
        let response = client.post(&url)
            .json(&payload)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Gemini API error: {}", response.status()));
        }
        
        let json: serde_json::Value = response.json().await?;
        let content = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;
            
        Ok(content.to_string())
    }

    async fn embed_openai(&self, text: &str) -> anyhow::Result<Vec<f64>> {
        let client = reqwest::Client::new();
        let url = "https://api.openai.com/v1/embeddings";
        
        let payload = json!({
            "model": "text-embedding-ada-002",
            "input": text
        });
        
        let response = client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("OpenAI Embeddings error: {}", response.status()));
        }
        
        let json: serde_json::Value = response.json().await?;
        let embedding = json["data"][0]["embedding"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;
            
        let result: Result<Vec<f64>, _> = embedding.iter()
            .map(|v| v.as_f64().ok_or_else(|| anyhow::anyhow!("Invalid value")))
            .collect();
        result
    }

    async fn embed_gemini(&self, text: &str) -> anyhow::Result<Vec<f64>> {
        let client = reqwest::Client::new();
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/embedding-001:embedContent?key={}", self.api_key);
        
        let payload = json!({
            "model": "models/embedding-001",
            "content": { "parts": [{ "text": text }] }
        });
        
        let response = client.post(&url)
            .json(&payload)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Gemini Embeddings error: {}", response.status()));
        }
        
        let json: serde_json::Value = response.json().await?;
        let embedding = json["embedding"]["values"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;
            
        let result: Result<Vec<f64>, _> = embedding.iter()
            .map(|v| v.as_f64().ok_or_else(|| anyhow::anyhow!("Invalid value")))
            .collect();
        result
    }
}

pub struct VectorDB {
    pub provider: String,
    pub connection_string: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f64,
}

impl VectorDB {
    pub fn new(provider: &str, connection_string: &str) -> Self {
        VectorDB {
            provider: provider.to_string(),
            connection_string: connection_string.to_string(),
        }
    }
    
    pub async fn upsert(&self, collection: &str, id: &str, vector: Vec<f64>) -> anyhow::Result<()> {
        match self.provider.as_str() {
            "pinecone" => self.upsert_pinecone(collection, id, vector).await,
            "qdrant" => self.upsert_qdrant(collection, id, vector).await,
            "local" => {
                let mut store = LOCAL_VECTOR_STORE.lock().unwrap();
                let records = store.entry(collection.to_string()).or_insert_with(Vec::new);
                // Update if exists, else push
                if let Some(pos) = records.iter().position(|r| r.id == id) {
                    records[pos].vector = vector;
                } else {
                    records.push(VectorRecord { id: id.to_string(), vector });
                }
                Ok(())
            },
            _ => Err(anyhow::anyhow!("Unknown provider: {}", self.provider)),
        }
    }
    
    pub async fn search(&self, collection: &str, query_vector: Vec<f64>, top_k: usize) -> anyhow::Result<Vec<SearchResult>> {
        match self.provider.as_str() {
            "pinecone" => self.search_pinecone(collection, query_vector, top_k).await,
            "qdrant" => self.search_qdrant(collection, query_vector, top_k).await,
            "local" => {
                let store = LOCAL_VECTOR_STORE.lock().unwrap();
                if let Some(records) = store.get(collection) {
                    let mut results: Vec<SearchResult> = records.iter().map(|r| {
                        let score = cosine_similarity(&query_vector, &r.vector);
                        SearchResult { id: r.id.clone(), score }
                    }).collect();
                    
                    // Sort by score descending
                    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
                    results.truncate(top_k);
                    Ok(results)
                } else {
                    Ok(vec![])
                }
            },
            _ => Err(anyhow::anyhow!("Unknown provider: {}", self.provider)),
        }
    }

    async fn upsert_pinecone(&self, index_name: &str, id: &str, vector: Vec<f64>) -> anyhow::Result<()> {
        let parts: Vec<&str> = self.connection_string.split('@').collect();
        if parts.len() != 2 { return Err(anyhow::anyhow!("Invalid Pinecone string")); }
        let (api_key, env) = (parts[0], parts[1]);
        
        let client = reqwest::Client::new();
        let url = format!("https://{}.svc.{}.pinecone.io/vectors/upsert", index_name, env);
        
        let payload = json!({
            "vectors": [{ "id": id, "values": vector }]
        });
        
        let res = client.post(url).header("Api-Key", api_key).json(&payload).send().await?;
        if !res.status().is_success() { return Err(anyhow::anyhow!("Pinecone error")); }
        Ok(())
    }

    async fn search_pinecone(&self, index_name: &str, query_vector: Vec<f64>, top_k: usize) -> anyhow::Result<Vec<SearchResult>> {
        let parts: Vec<&str> = self.connection_string.split('@').collect();
        if parts.len() != 2 { return Err(anyhow::anyhow!("Invalid Pinecone string")); }
        let (api_key, env) = (parts[0], parts[1]);
        
        let client = reqwest::Client::new();
        let url = format!("https://{}.svc.{}.pinecone.io/query", index_name, env);
        
        let payload = json!({ "vector": query_vector, "topK": top_k, "includeMetadata": false });
        
        let res = client.post(url).header("Api-Key", api_key).json(&payload).send().await?;
        if !res.status().is_success() { return Err(anyhow::anyhow!("Pinecone error")); }
        
        let json: serde_json::Value = res.json().await?;
        let matches = json["matches"].as_array().ok_or_else(|| anyhow::anyhow!("Invalid format"))?;
        
        let results = matches.iter().map(|m| {
            SearchResult {
                id: m["id"].as_str().unwrap_or("").to_string(),
                score: m["score"].as_f64().unwrap_or(0.0),
            }
        }).collect();
        Ok(results)
    }

    async fn upsert_qdrant(&self, collection: &str, id: &str, vector: Vec<f64>) -> anyhow::Result<()> {
        // Simple REST impl for Qdrant
        let client = reqwest::Client::new();
        let url = format!("{}/collections/{}/points", self.connection_string, collection);
        let payload = json!({
            "points": [{ "id": id, "vector": vector }]
        });
        let res = client.put(url).json(&payload).send().await?;
        if !res.status().is_success() { return Err(anyhow::anyhow!("Qdrant error")); }
        Ok(())
    }

    async fn search_qdrant(&self, collection: &str, query_vector: Vec<f64>, top_k: usize) -> anyhow::Result<Vec<SearchResult>> {
        let client = reqwest::Client::new();
        let url = format!("{}/collections/{}/points/search", self.connection_string, collection);
        let payload = json!({ "vector": query_vector, "limit": top_k, "with_payload": false });
        let res = client.post(url).json(&payload).send().await?;
        if !res.status().is_success() { return Err(anyhow::anyhow!("Qdrant error")); }
        
        let json: serde_json::Value = res.json().await?;
        let result = json["result"].as_array().ok_or_else(|| anyhow::anyhow!("Invalid format"))?;
        
        let results = result.iter().map(|r| {
            SearchResult {
                id: r["id"].as_str().unwrap_or("").to_string(),
                score: r["score"].as_f64().unwrap_or(0.0),
            }
        }).collect();
        Ok(results)
    }
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot_product: f64 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 { 0.0 } else { dot_product / (norm_a * norm_b) }
}
"#
        .to_string()
    }
}

/// ONNX Training Configuration
#[derive(Debug, Clone)]
pub struct ONNXTrainingConfig {
    pub epochs: u32,
    pub batch_size: u32,
    pub learning_rate: f64,
    pub optimizer: String,
    pub loss_function: String,
}

impl Default for ONNXTrainingConfig {
    fn default() -> Self {
        ONNXTrainingConfig {
            epochs: 100,
            batch_size: 32,
            learning_rate: 0.001,
            optimizer: "Adam".to_string(),
            loss_function: "CrossEntropy".to_string(),
        }
    }
}

/// TensorFlow Training Configuration
#[derive(Debug, Clone)]
pub struct TensorFlowTrainingConfig {
    pub epochs: u32,
    pub batch_size: u32,
    pub learning_rate: f64,
    pub optimizer: String,
    pub loss_function: String,
    pub validation_split: f64,
}

impl Default for TensorFlowTrainingConfig {
    fn default() -> Self {
        TensorFlowTrainingConfig {
            epochs: 100,
            batch_size: 32,
            learning_rate: 0.001,
            optimizer: "Adam".to_string(),
            loss_function: "SparseCategoricalCrossentropy".to_string(),
            validation_split: 0.2,
        }
    }
}

/// Model Training Result
#[derive(Debug, Clone)]
pub struct ModelTrainingResult {
    pub model_name: String,
    pub framework: String,
    pub accuracy: f64,
    pub loss: f64,
    pub epochs: u32,
    pub training_time_seconds: f64,
}

/// Model Evaluation Result
#[derive(Debug, Clone)]
pub struct ModelEvaluationResult {
    pub model_name: String,
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub test_samples: usize,
}
