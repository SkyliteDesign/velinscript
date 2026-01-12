// Standard Library für Iterator-Erweiterungen
// Integration von itertools für erweiterte Iterator-Funktionen

/// Iterator Standard Library
pub struct IteratorStdlib;

impl IteratorStdlib {
    /// Transformiert VelinScript list.groupBy() zu Rust-Code
    pub fn generate_group_by_code(list: &str, key_fn: &str) -> String {
        format!(
            "{}.iter().group_by(|x| {}).into_iter().map(|(k, v)| (k, v.collect())).collect()",
            list, key_fn
        )
    }
    
    /// Transformiert VelinScript list.sorted() zu Rust-Code
    pub fn generate_sorted_code(list: &str) -> String {
        format!("{}.iter().sorted().collect()", list)
    }
    
    /// Transformiert VelinScript list.multizip() zu Rust-Code
    pub fn generate_multizip_code(lists: &[&str]) -> String {
        if lists.len() == 2 {
            format!(
                "{}.iter().zip({}.iter()).collect()",
                lists[0], lists[1]
            )
        } else {
            format!(
                "itertools::multizip(({})).collect()",
                lists.iter().map(|l| format!("{}.iter()", l)).collect::<Vec<_>>().join(", ")
            )
        }
    }
    
    /// Transformiert VelinScript list.chunks() zu Rust-Code
    pub fn generate_chunks_code(list: &str, size: &str) -> String {
        format!(
            "{}.chunks({}).map(|chunk| chunk.to_vec()).collect()",
            list, size
        )
    }
    
    /// Transformiert VelinScript list.unique() zu Rust-Code
    pub fn generate_unique_code(list: &str) -> String {
        format!("{}.iter().unique().collect()", list)
    }
    
    /// Transformiert VelinScript list.flatten() zu Rust-Code
    pub fn generate_flatten_code(list: &str) -> String {
        format!("{}.iter().flatten().collect()", list)
    }
    
    /// Transformiert VelinScript list.join() zu Rust-Code
    pub fn generate_join_code(list: &str, separator: &str) -> String {
        format!("{}.iter().join(\"{}\")", list, separator)
    }
}

/// Prüft ob eine Liste groß genug für Parallelisierung ist
pub fn should_parallelize(list_size: usize, threshold: usize) -> bool {
    list_size > threshold
}

/// Standard-Schwellenwert für automatische Parallelisierung
pub const PARALLEL_THRESHOLD: usize = 1000;
