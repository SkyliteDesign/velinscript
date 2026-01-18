// Statistics
// Berechnet statistische Metriken

pub struct Statistics;

impl Statistics {
    pub fn new() -> Self {
        Self
    }
    
    pub fn mean(&self, values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        values.iter().sum::<f64>() / values.len() as f64
    }
    
    pub fn min(&self, values: &[f64]) -> f64 {
        values.iter().copied().fold(f64::INFINITY, f64::min)
    }
    
    pub fn max(&self, values: &[f64]) -> f64 {
        values.iter().copied().fold(f64::NEG_INFINITY, f64::max)
    }
    
    pub fn std_dev(&self, values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }
        
        let mean = self.mean(values);
        let variance = values.iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>() / (values.len() - 1) as f64;
        
        variance.sqrt()
    }
}
