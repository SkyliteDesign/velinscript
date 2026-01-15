// Collections Standard Library
// Erweiterte Collections-Methoden für List, Map und Set

pub struct CollectionsStdlib;

impl CollectionsStdlib {
    /// Generiert Rust-Code für List.filter()
    pub fn generate_list_filter(list: &str, predicate: &str) -> String {
        format!(
            "{}.into_iter().filter({}).collect::<Vec<_>>()",
            list, predicate
        )
    }
    
    /// Generiert Rust-Code für List.map()
    pub fn generate_list_map(list: &str, mapper: &str) -> String {
        format!(
            "{}.into_iter().map({}).collect::<Vec<_>>()",
            list, mapper
        )
    }
    
    /// Generiert Rust-Code für List.reduce()
    pub fn generate_list_reduce(list: &str, reducer: &str, initial: &str) -> String {
        format!(
            "{}.into_iter().fold({}, {})",
            list, initial, reducer
        )
    }
    
    /// Generiert Rust-Code für List.find()
    pub fn generate_list_find(list: &str, predicate: &str) -> String {
        format!(
            "{}.into_iter().find({})",
            list, predicate
        )
    }
    
    /// Generiert Rust-Code für List.contains()
    pub fn generate_list_contains(list: &str, item: &str) -> String {
        format!(
            "{}.contains(&{})",
            list, item
        )
    }
    
    /// Generiert Rust-Code für List.indexOf()
    pub fn generate_list_index_of(list: &str, item: &str) -> String {
        format!(
            "{}.iter().position(|x| x == &{}).map(|i| i as i32).unwrap_or(-1)",
            list, item
        )
    }
    
    /// Generiert Rust-Code für List.sort()
    pub fn generate_list_sort(list: &str, compare: Option<&str>) -> String {
        if let Some(compare_fn) = compare {
            format!(
                "{{ let mut v = {}.clone(); v.sort_by({}); v }}",
                list, compare_fn
            )
        } else {
            format!(
                "{{ let mut v = {}.clone(); v.sort(); v }}",
                list
            )
        }
    }
    
    /// Generiert Rust-Code für List.reverse()
    pub fn generate_list_reverse(list: &str) -> String {
        format!(
            "{{ let mut v = {}.clone(); v.reverse(); v }}",
            list
        )
    }
    
    /// Generiert Rust-Code für List.chunk()
    pub fn generate_list_chunk(list: &str, size: &str) -> String {
        format!(
            "{}.chunks({}).map(|chunk| chunk.to_vec()).collect::<Vec<_>>()",
            list, size
        )
    }
    
    /// Generiert Rust-Code für List.slice()
    pub fn generate_list_slice(list: &str, start: &str, end: &str) -> String {
        format!(
            "{}.get({}..{}).map(|s| s.to_vec()).unwrap_or_default()",
            list, start, end
        )
    }
    
    /// Generiert Rust-Code für Map.keys()
    pub fn generate_map_keys(map: &str) -> String {
        format!(
            "{}.keys().cloned().collect::<Vec<_>>()",
            map
        )
    }
    
    /// Generiert Rust-Code für Map.values()
    pub fn generate_map_values(map: &str) -> String {
        format!(
            "{}.values().cloned().collect::<Vec<_>>()",
            map
        )
    }
    
    /// Generiert Rust-Code für Map.entries()
    pub fn generate_map_entries(map: &str) -> String {
        format!(
            "{}.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>()",
            map
        )
    }
    
    /// Generiert Rust-Code für Map.get()
    pub fn generate_map_get(map: &str, key: &str) -> String {
        format!(
            "{}.get(&{}).cloned()",
            map, key
        )
    }
    
    /// Generiert Rust-Code für Map.set()
    pub fn generate_map_set(map: &str, key: &str, value: &str) -> String {
        format!(
            "{{ {}.insert({}.clone(), {}.clone()); }}",
            map, key, value
        )
    }
    
    /// Generiert Rust-Code für Map.delete()
    pub fn generate_map_delete(map: &str, key: &str) -> String {
        format!(
            "{}.remove(&{})",
            map, key
        )
    }
    
    /// Generiert Rust-Code für Map.has()
    pub fn generate_map_has(map: &str, key: &str) -> String {
        format!(
            "{}.contains_key(&{})",
            map, key
        )
    }
    
    /// Generiert Rust-Code für Map.size()
    pub fn generate_map_size(map: &str) -> String {
        format!(
            "{}.len()",
            map
        )
    }
    
    /// Generiert Rust-Code für Set.add()
    pub fn generate_set_add(set: &str, item: &str) -> String {
        format!(
            "{{ {}.insert({}.clone()); }}",
            set, item
        )
    }
    
    /// Generiert Rust-Code für Set.remove()
    pub fn generate_set_remove(set: &str, item: &str) -> String {
        format!(
            "{}.remove(&{})",
            set, item
        )
    }
    
    /// Generiert Rust-Code für Set.has()
    pub fn generate_set_has(set: &str, item: &str) -> String {
        format!(
            "{}.contains(&{})",
            set, item
        )
    }
    
    /// Generiert Rust-Code für Set.size()
    pub fn generate_set_size(set: &str) -> String {
        format!(
            "{}.len()",
            set
        )
    }
    
    /// Generiert Rust-Code für Set.union()
    pub fn generate_set_union(set1: &str, set2: &str) -> String {
        format!(
            "{}.union({}).cloned().collect::<std::collections::HashSet<_>>()",
            set1, set2
        )
    }
    
    /// Generiert Rust-Code für Set.intersection()
    pub fn generate_set_intersection(set1: &str, set2: &str) -> String {
        format!(
            "{}.intersection({}).cloned().collect::<std::collections::HashSet<_>>()",
            set1, set2
        )
    }
    
    /// Generiert Rust-Code für Set.difference()
    pub fn generate_set_difference(set1: &str, set2: &str) -> String {
        format!(
            "{}.difference({}).cloned().collect::<std::collections::HashSet<_>>()",
            set1, set2
        )
    }
}

/// Prüft ob eine Liste groß genug für Parallelisierung ist
pub fn should_parallelize_list(list_size: usize, threshold: usize) -> bool {
    list_size > threshold
}

/// Standard-Schwellenwert für automatische Parallelisierung
pub const PARALLEL_THRESHOLD: usize = 1000;

/// Generiert parallelen Code für große Listen
pub fn generate_parallel_map(list: &str, mapper: &str) -> String {
    format!(
        "{}.par_iter().map({}).collect::<Vec<_>>()",
        list, mapper
    )
}
