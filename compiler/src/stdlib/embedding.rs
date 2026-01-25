pub struct EmbeddingStdlib;

impl EmbeddingStdlib {
    pub fn generate_compare_code(a: &str, b: &str) -> String {
        format!("crate::stdlib::ml::cosine_similarity(&{}, &{})", a, b)
    }

    pub fn generate_similarity_code(a: &str, b: &str) -> String {
        format!("crate::stdlib::ml::cosine_similarity(&{}, &{})", a, b)
    }

    pub fn generate_cluster_code(list: &str, k: &str) -> String {
        // Simple K-Means implementation injected inline or referenced
        // For brevity, let's inject a simple block
        format!(
            "{{
                let data = {}.clone();
                let k = {} as usize;
                if data.is_empty() || k == 0 {{ vec![] }}
                else {{
                    // Simple random initialization
                    let mut centroids: Vec<Vec<f64>> = data.iter().take(k).cloned().collect();
                    // One iteration for now (should be loop in real impl)
                    let mut clusters: Vec<Vec<Vec<f64>>> = vec![vec![]; k];
                    for point in &data {{
                        let mut min_dist = f64::MAX;
                        let mut best_c = 0;
                        for (i, c) in centroids.iter().enumerate() {{
                            let dist = point.iter().zip(c).map(|(x, y)| (x - y).powi(2)).sum::<f64>();
                            if dist < min_dist {{ min_dist = dist; best_c = i; }}
                        }}
                        clusters[best_c].push(point.clone());
                    }}
                    clusters
                }}
            }}",
            list, k
        )
    }

    pub fn generate_normalize_code(embedding: &str) -> String {
        format!(
            "{{
                let v = {};
                let norm: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt();
                if norm == 0.0 {{ v }} else {{ v.iter().map(|x| x / norm).collect() }}
            }}",
            embedding
        )
    }

    pub fn generate_distance_code(a: &str, b: &str) -> String {
        format!(
            "{{
                {}.iter().zip({}).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt()
            }}",
            a, b
        )
    }

    pub fn generate_find_nearest_code(query: &str, candidates: &str, k: &str) -> String {
        format!(
            "{{
                let q = {};
                let mut c: Vec<(usize, f64)> = {}.iter().enumerate().map(|(i, v)| {{
                    let sim = crate::stdlib::ml::cosine_similarity(&q, v);
                    (i, sim)
                }}).collect();
                c.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                c.iter().take({} as usize).map(|(i, _)| {}.get(*i).unwrap().clone()).collect::<Vec<_>>()
            }}",
            query, candidates, k, candidates
        )
    }

    pub fn generate_average_code(embeddings: &str) -> String {
        format!(
            "{{
                let list = {};
                if list.is_empty() {{ vec![] }}
                else {{
                    let dim = list[0].len();
                    let count = list.len() as f64;
                    let mut avg = vec![0.0; dim];
                    for v in &list {{
                        for (i, x) in v.iter().enumerate() {{
                            avg[i] += x;
                        }}
                    }}
                    avg.iter().map(|x| x / count).collect()
                }}
            }}",
            embeddings
        )
    }

    pub fn generate_dimension_code(embedding: &str) -> String {
        format!("{}.len() as i32", embedding)
    }
}
