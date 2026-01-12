// Standard Library für API-Funktionalität
// Diese Funktionen werden vom Code Generator verwendet

/// Built-in API Funktionen, die in VelinScript verfügbar sind
pub struct ApiStdlib;

impl ApiStdlib {
    /// Liste der verfügbaren API-Funktionen
    pub fn get_functions() -> Vec<FunctionInfo> {
        vec![
            FunctionInfo {
                name: "db.find".to_string(),
                signature: "fn<T>(T, string) -> Option<T>".to_string(),
            },
            FunctionInfo {
                name: "db.findAll".to_string(),
                signature: "fn<T>(T) -> Vec<T>".to_string(),
            },
            FunctionInfo {
                name: "db.save".to_string(),
                signature: "fn<T>(T) -> T".to_string(),
            },
            FunctionInfo {
                name: "db.delete".to_string(),
                signature: "fn<T>(T, string) -> bool".to_string(),
            },
        ]
    }
}

pub struct FunctionInfo {
    pub name: String,
    pub signature: String,
}
