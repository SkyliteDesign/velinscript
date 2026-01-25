// Metrics Framework für Velin-Klassen
// Prometheus-ähnliche Metrics Collection

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Metric-Typen
#[derive(Debug, Clone, PartialEq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
}

/// Metric-Wert
#[derive(Debug, Clone)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(Vec<f64>),
}

/// Einzelne Metric
#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub metric_type: MetricType,
    pub value: MetricValue,
    pub labels: HashMap<String, String>,
    pub timestamp: Option<Instant>,
}

/// Metrics Collector
pub struct MetricsCollector {
    metrics: Arc<Mutex<HashMap<String, Metric>>>,
}

impl MetricsCollector {
    /// Erstellt einen neuen Metrics Collector
    pub fn new() -> Self {
        MetricsCollector {
            metrics: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Erhöht einen Counter
    pub fn increment_counter(&self, name: &str, labels: Option<HashMap<String, String>>) {
        let mut metrics = self.metrics.lock().unwrap();
        let key = self.build_key(name, labels.as_ref());

        if let Some(metric) = metrics.get_mut(&key) {
            if let MetricValue::Counter(ref mut count) = metric.value {
                *count += 1;
            }
        } else {
            metrics.insert(
                key,
                Metric {
                    name: name.to_string(),
                    metric_type: MetricType::Counter,
                    value: MetricValue::Counter(1),
                    labels: labels.unwrap_or_default(),
                    timestamp: Some(Instant::now()),
                },
            );
        }
    }

    /// Setzt einen Gauge-Wert
    pub fn set_gauge(&self, name: &str, value: f64, labels: Option<HashMap<String, String>>) {
        let mut metrics = self.metrics.lock().unwrap();
        let key = self.build_key(name, labels.as_ref());

        metrics.insert(
            key,
            Metric {
                name: name.to_string(),
                metric_type: MetricType::Gauge,
                value: MetricValue::Gauge(value),
                labels: labels.unwrap_or_default(),
                timestamp: Some(Instant::now()),
            },
        );
    }

    /// Fügt einen Wert zu einem Histogram hinzu
    pub fn observe_histogram(
        &self,
        name: &str,
        value: f64,
        labels: Option<HashMap<String, String>>,
    ) {
        let mut metrics = self.metrics.lock().unwrap();
        let key = self.build_key(name, labels.as_ref());

        if let Some(metric) = metrics.get_mut(&key) {
            if let MetricValue::Histogram(ref mut values) = metric.value {
                values.push(value);
            }
        } else {
            metrics.insert(
                key,
                Metric {
                    name: name.to_string(),
                    metric_type: MetricType::Histogram,
                    value: MetricValue::Histogram(vec![value]),
                    labels: labels.unwrap_or_default(),
                    timestamp: Some(Instant::now()),
                },
            );
        }
    }

    /// Gibt alle Metrics zurück
    pub fn get_metrics(&self) -> Vec<Metric> {
        let metrics = self.metrics.lock().unwrap();
        metrics.values().cloned().collect()
    }

    /// Gibt eine spezifische Metric zurück
    pub fn get_metric(
        &self,
        name: &str,
        labels: Option<&HashMap<String, String>>,
    ) -> Option<Metric> {
        let metrics = self.metrics.lock().unwrap();
        let key = self.build_key(name, labels);
        metrics.get(&key).cloned()
    }

    /// Exportiert Metrics im Prometheus-Format
    pub fn export_prometheus(&self) -> String {
        let metrics = self.metrics.lock().unwrap();
        let mut output = String::new();

        for metric in metrics.values() {
            let labels_str = if metric.labels.is_empty() {
                String::new()
            } else {
                let label_parts: Vec<String> = metric
                    .labels
                    .iter()
                    .map(|(k, v)| format!("{}=\"{}\"", k, v))
                    .collect();
                format!("{{{}}}", label_parts.join(", "))
            };

            match &metric.value {
                MetricValue::Counter(count) => {
                    output.push_str(&format!("{}_total{} {}\n", metric.name, labels_str, count));
                }
                MetricValue::Gauge(value) => {
                    output.push_str(&format!("{}{} {}\n", metric.name, labels_str, value));
                }
                MetricValue::Histogram(values) => {
                    let sum: f64 = values.iter().sum();
                    let count = values.len();
                    let avg = if count > 0 { sum / count as f64 } else { 0.0 };

                    output.push_str(&format!("{}_sum{} {}\n", metric.name, labels_str, sum));
                    output.push_str(&format!("{}_count{} {}\n", metric.name, labels_str, count));
                    output.push_str(&format!("{}_avg{} {}\n", metric.name, labels_str, avg));
                }
            }
        }

        output
    }

    /// Hilfsfunktion zum Erstellen eines Keys
    fn build_key(&self, name: &str, labels: Option<&HashMap<String, String>>) -> String {
        if let Some(labels) = labels {
            let mut key = name.to_string();
            let mut sorted_labels: Vec<(&String, &String)> = labels.iter().collect();
            sorted_labels.sort_by_key(|(k, _)| *k);
            for (k, v) in sorted_labels {
                key.push_str(&format!(":{}={}", k, v));
            }
            key
        } else {
            name.to_string()
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance Monitor
pub struct PerformanceMonitor {
    collector: MetricsCollector,
    start_times: Arc<Mutex<HashMap<String, Instant>>>,
}

impl PerformanceMonitor {
    /// Erstellt einen neuen Performance Monitor
    pub fn new() -> Self {
        PerformanceMonitor {
            collector: MetricsCollector::new(),
            start_times: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Startet die Zeitmessung für eine Operation
    pub fn start_operation(&self, name: &str) {
        let mut times = self.start_times.lock().unwrap();
        times.insert(name.to_string(), Instant::now());
    }

    /// Beendet die Zeitmessung und speichert die Dauer
    pub fn end_operation(&self, name: &str, labels: Option<HashMap<String, String>>) {
        let duration = {
            let mut times = self.start_times.lock().unwrap();
            if let Some(start) = times.remove(name) {
                start.elapsed().as_secs_f64()
            } else {
                0.0
            }
        };

        self.collector.observe_histogram(name, duration, labels);
    }

    /// Gibt den Metrics Collector zurück
    pub fn collector(&self) -> &MetricsCollector {
        &self.collector
    }
}

/// Metrics Standard Library für Code-Generierung
pub struct MetricsStdlib;

impl MetricsStdlib {
    /// Generiert Rust-Code für metrics.increment()
    pub fn generate_increment_code(name: &str, labels: Option<&str>) -> String {
        if let Some(l) = labels {
            format!("metrics::increment_counter(\"{}\", Some({}))", name, l)
        } else {
            format!("metrics::increment_counter(\"{}\", None)", name)
        }
    }

    /// Generiert Rust-Code für metrics.gauge()
    pub fn generate_gauge_code(name: &str, value: &str, labels: Option<&str>) -> String {
        if let Some(l) = labels {
            format!("metrics::set_gauge(\"{}\", {}, Some({}))", name, value, l)
        } else {
            format!("metrics::set_gauge(\"{}\", {}, None)", name, value)
        }
    }

    /// Generiert Rust-Code für metrics.histogram()
    pub fn generate_histogram_code(name: &str, value: &str, labels: Option<&str>) -> String {
        if let Some(l) = labels {
            format!(
                "metrics::observe_histogram(\"{}\", {}, Some({}))",
                name, value, l
            )
        } else {
            format!("metrics::observe_histogram(\"{}\", {}, None)", name, value)
        }
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Health Check Status
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Health Check
pub struct HealthCheck {
    status: HealthStatus,
    checks: HashMap<String, bool>,
    message: Option<String>,
}

impl HealthCheck {
    /// Erstellt einen neuen Health Check
    pub fn new() -> Self {
        HealthCheck {
            status: HealthStatus::Healthy,
            checks: HashMap::new(),
            message: None,
        }
    }

    /// Setzt den Status einer Komponente
    pub fn set_component_status(&mut self, component: &str, healthy: bool) {
        self.checks.insert(component.to_string(), healthy);
        self.update_overall_status();
    }

    /// Setzt eine Nachricht
    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }

    /// Gibt den aktuellen Status zurück
    pub fn status(&self) -> &HealthStatus {
        &self.status
    }

    /// Gibt alle Checks zurück
    pub fn checks(&self) -> &HashMap<String, bool> {
        &self.checks
    }

    /// Gibt die Nachricht zurück
    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    /// Aktualisiert den Gesamtstatus basierend auf den einzelnen Checks
    fn update_overall_status(&mut self) {
        let all_healthy = self.checks.values().all(|&healthy| healthy);
        let any_unhealthy = self.checks.values().any(|&healthy| !healthy);

        self.status = if all_healthy {
            HealthStatus::Healthy
        } else if any_unhealthy && self.checks.len() > 1 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Unhealthy
        };
    }

    /// Exportiert Health Check als JSON
    pub fn export_json(&self) -> String {
        let status_str = match self.status {
            HealthStatus::Healthy => "healthy",
            HealthStatus::Degraded => "degraded",
            HealthStatus::Unhealthy => "unhealthy",
        };

        format!(
            r#"{{"status":"{}","checks":{},"message":{}}}"#,
            status_str,
            serde_json::to_string(&self.checks).unwrap_or_default(),
            self.message
                .as_ref()
                .map(|m| format!("\"{}\"", m))
                .unwrap_or_else(|| "null".to_string())
        )
    }
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self::new()
    }
}
