pub use crate::codegen::system_generator::{DeploymentConfig, DeploymentType, Requirements};
use anyhow::Result;

/// Infrastructure-as-Code Generator
/// 
/// Generiert Infrastructure-Code für Deployment:
/// - Dockerfile
/// - docker-compose.yml
/// - Kubernetes Manifests
/// - Helm Charts
/// - Serverless Configs
pub struct InfrastructureGenerator;

impl InfrastructureGenerator {
    pub fn new() -> Self {
        InfrastructureGenerator
    }

    /// Generiert Infrastructure-Code basierend auf Deployment-Plan
    pub fn generate(&self, plan: &DeploymentPlan) -> Result<InfrastructureCode> {
        let mut code = InfrastructureCode::new();

        match plan.deployment_type {
            DeploymentType::Local => {
                // Keine Infrastructure nötig
            }
            DeploymentType::CloudSingle => {
                code.add(self.generate_dockerfile(plan)?);
                code.add(self.generate_docker_compose(plan)?);
            }
            DeploymentType::CloudMulti => {
                code.add(self.generate_kubernetes(plan)?);
                code.add(self.generate_helm_chart(plan)?);
            }
            DeploymentType::Serverless => {
                code.add(self.generate_lambda(plan)?);
                code.add(self.generate_api_gateway(plan)?);
            }
        }

        // Caching-Infrastructure
        if plan.needs_caching {
            code.add(self.generate_redis(plan)?);
        }

        // Database-Infrastructure
        if plan.needs_database {
            code.add(self.generate_database(plan)?);
        }
        
        // Validiere alle generierten Configs
        self.validate_infrastructure_code(&code)?;

        Ok(code)
    }
    
    /// Validiert generierte Infrastructure-Configs
    fn validate_infrastructure_code(&self, code: &InfrastructureCode) -> Result<()> {
        for file in &code.files {
            match file.name.as_str() {
                name if name.ends_with(".yaml") || name.ends_with(".yml") => {
                    // YAML-Validierung
                    let _: serde_yaml::Value = serde_yaml::from_str(&file.content)
                        .map_err(|e| anyhow::anyhow!("Invalid YAML in {}: {}", file.name, e))?;
                }
                name if name.ends_with(".json") => {
                    // JSON-Validierung
                    let _: serde_json::Value = serde_json::from_str(&file.content)
                        .map_err(|e| anyhow::anyhow!("Invalid JSON in {}: {}", file.name, e))?;
                }
                "Dockerfile" => {
                    // Basis-Validierung für Dockerfile
                    if !file.content.contains("FROM") {
                        return Err(anyhow::anyhow!("Dockerfile must contain FROM instruction"));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Generiert Dockerfile
    fn generate_dockerfile(&self, _plan: &DeploymentPlan) -> Result<InfrastructureFile> {
        Ok(InfrastructureFile {
            name: "Dockerfile".to_string(),
            content: r#"FROM rust:1.70 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/velin-compiler /usr/local/bin/velin-compiler
EXPOSE 3000
ENV RUST_LOG=info
CMD ["velin-compiler"]
"#.to_string(),
        })
    }

    /// Generiert docker-compose.yml
    fn generate_docker_compose(&self, plan: &DeploymentPlan) -> Result<InfrastructureFile> {
        let mut compose = r#"version: '3.8'
services:
  api:
    build: .
    ports:
      - "3000:3000"
    environment:
      - RUST_LOG=info
"#.to_string();

        if plan.needs_caching {
            compose.push_str("      - REDIS_URL=redis://redis:6379\n");
            compose.push_str("    depends_on:\n      - redis\n");
        }
        if plan.needs_database {
            compose.push_str("      - DATABASE_URL=${DATABASE_URL}\n");
            compose.push_str("    depends_on:\n      - postgres\n");
        }

        if plan.needs_caching {
            compose.push_str("\n  redis:\n    image: redis:alpine\n    ports:\n      - \"6379:6379\"\n");
        }
        if plan.needs_database {
            compose.push_str("\n  postgres:\n    image: postgres:15\n    environment:\n      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}\n    ports:\n      - \"5432:5432\"\n");
        }

        Ok(InfrastructureFile {
            name: "docker-compose.yml".to_string(),
            content: compose,
        })
    }

    /// Generiert Kubernetes Manifests
    fn generate_kubernetes(&self, plan: &DeploymentPlan) -> Result<InfrastructureFile> {
        let replicas = plan.replicas.unwrap_or(3);
        
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(3000);
        
        let content = format!(
            r#"# Kubernetes Deployment Configuration
# Port: {}
# Replicas: {}

apiVersion: apps/v1
kind: Deployment
metadata:
  name: velin-api
  labels:
    app: velin-api
    version: "3.1.0"
spec:
  replicas: {}
  selector:
    matchLabels:
      app: velin-api
  template:
    metadata:
      labels:
        app: velin-api
        version: "3.1.0"
    spec:
      containers:
      - name: api
        image: velin-api:latest
        imagePullPolicy: Always
        ports:
        - name: http
          containerPort: {}
          protocol: TCP
        - name: metrics
          containerPort: 9090
          protocol: TCP
        env:
        - name: PORT
          value: "{}"
        - name: RUST_LOG
          value: "info"
        # Health Checks
        livenessProbe:
          httpGet:
            path: /health
            port: {}
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
        readinessProbe:
          httpGet:
            path: /health
            port: {}
          initialDelaySeconds: 5
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 3
        # Resource Limits
        resources:
          requests:
            memory: "{}"
            cpu: "{}"
          limits:
            memory: "{}"
            cpu: "{}"
        # Security Context
        securityContext:
          runAsNonRoot: true
          runAsUser: 1000
          allowPrivilegeEscalation: false
---
# Service für Load Balancing
apiVersion: v1
kind: Service
metadata:
  name: velin-api-service
  labels:
    app: velin-api
spec:
  type: LoadBalancer
  selector:
    app: velin-api
  ports:
  - name: http
    port: 80
    targetPort: {}
    protocol: TCP
  - name: metrics
    port: 9090
    targetPort: 9090
    protocol: TCP
  sessionAffinity: ClientIP
  sessionAffinityConfig:
    clientIP:
      timeoutSeconds: 10800
---
# HorizontalPodAutoscaler
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: velin-api-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: velin-api
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
---
# ServiceMonitor für Prometheus (optional, benötigt Prometheus Operator)
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: velin-api-monitor
  labels:
    app: velin-api
spec:
  selector:
    matchLabels:
      app: velin-api
  endpoints:
  - port: metrics
    path: /metrics
    interval: 30s
    scrapeTimeout: 10s
"#,
            port, replicas, replicas, port, port, port, port,
            plan.memory_request.as_deref().unwrap_or("256Mi"),
            plan.cpu_request.as_deref().unwrap_or("100m"),
            plan.memory_limit.as_deref().unwrap_or("512Mi"),
            plan.cpu_limit.as_deref().unwrap_or("500m"),
            port,
        );

        Ok(InfrastructureFile {
            name: "kubernetes.yaml".to_string(),
            content,
        })
    }

    /// Generiert Helm Chart
    fn generate_helm_chart(&self, _plan: &DeploymentPlan) -> Result<InfrastructureFile> {
        let content = r#"apiVersion: v1
kind: ConfigMap
metadata:
  name: velin-api-config
data:
  RUST_LOG: "info"
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: velin-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: velin-api
  template:
    metadata:
      labels:
        app: velin-api
    spec:
      containers:
      - name: api
        image: velin-api:latest
        ports:
        - containerPort: 3000
        envFrom:
        - configMapRef:
            name: velin-api-config
"#.to_string();

        Ok(InfrastructureFile {
            name: "helm-chart.yaml".to_string(),
            content,
        })
    }

    /// Generiert AWS Lambda Config
    fn generate_lambda(&self, _plan: &DeploymentPlan) -> Result<InfrastructureFile> {
        let content = r#"{
  "Runtime": "provided.al2",
  "Handler": "bootstrap",
  "MemorySize": 512,
  "Timeout": 30,
  "Environment": {
    "Variables": {
      "RUST_LOG": "info"
    }
  }
}
"#.to_string();

        Ok(InfrastructureFile {
            name: "lambda-config.json".to_string(),
            content,
        })
    }

    /// Generiert API Gateway Config
    fn generate_api_gateway(&self, _plan: &DeploymentPlan) -> Result<InfrastructureFile> {
        // Ersetze Platzhalter durch Umgebungsvariablen oder Config-Werte
        let account_id = std::env::var("AWS_ACCOUNT_ID")
            .unwrap_or_else(|_| "${AWS_ACCOUNT_ID}".to_string());
        let region = std::env::var("AWS_REGION")
            .unwrap_or_else(|_| "us-east-1".to_string());
        let function_name = std::env::var("LAMBDA_FUNCTION_NAME")
            .unwrap_or_else(|_| "velin-api".to_string());
        
        let content = format!(r#"{{
  "openapi": "3.0.0",
  "info": {{
    "title": "VelinScript API",
    "version": "1.0.0"
  }},
  "paths": {{
    "/api/{{proxy+}}": {{
      "x-amazon-apigateway-any-method": {{
        "x-amazon-apigateway-integration": {{
          "type": "aws_proxy",
          "httpMethod": "POST",
          "uri": "arn:aws:apigateway:{}:lambda:path/2015-03-31/functions/arn:aws:lambda:{}:{}:function:{}/invocations"
        }}
      }}
    }}
  }}
}}
"#, region, region, account_id, function_name);

        Ok(InfrastructureFile {
            name: "api-gateway.json".to_string(),
            content,
        })
    }

    /// Generiert Redis Config
    fn generate_redis(&self, plan: &DeploymentPlan) -> Result<InfrastructureFile> {
        let _ = plan; // Für zukünftige Verwendung
        let content = r#"apiVersion: v1
kind: Service
metadata:
  name: redis
spec:
  ports:
  - port: 6379
    targetPort: 6379
  selector:
    app: redis
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: redis
spec:
  replicas: 1
  selector:
    matchLabels:
      app: redis
  template:
    metadata:
      labels:
        app: redis
    spec:
      containers:
      - name: redis
        image: redis:alpine
        ports:
        - containerPort: 6379
"#.to_string();

        Ok(InfrastructureFile {
            name: "redis.yaml".to_string(),
            content,
        })
    }

    /// Generiert Database Config
    fn generate_database(&self, _plan: &DeploymentPlan) -> Result<InfrastructureFile> {
        let content = r#"apiVersion: v1
kind: Service
metadata:
  name: postgres
spec:
  ports:
  - port: 5432
    targetPort: 5432
  selector:
    app: postgres
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: postgres
spec:
  replicas: 1
  selector:
    matchLabels:
      app: postgres
  template:
    metadata:
      labels:
        app: postgres
    spec:
      containers:
      - name: postgres
        image: postgres:15
        ports:
        - containerPort: 5432
        env:
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef:
              name: postgres-secret
              key: password
"#.to_string();

        Ok(InfrastructureFile {
            name: "postgres.yaml".to_string(),
            content,
        })
    }
}

/// Deployment-Plan für Infrastructure-Generierung
#[derive(Debug, Clone)]
pub struct DeploymentPlan {
    pub deployment_type: DeploymentType,
    pub needs_caching: bool,
    pub needs_database: bool,
    pub replicas: Option<u32>,
    pub memory_request: Option<String>,
    pub cpu_request: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_limit: Option<String>,
}

/// Infrastructure-Code Container
#[derive(Debug, Clone)]
pub struct InfrastructureCode {
    pub files: Vec<InfrastructureFile>,
}

impl InfrastructureCode {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
        }
    }

    pub fn add(&mut self, file: InfrastructureFile) {
        self.files.push(file);
    }
}

/// Infrastructure-Datei
#[derive(Debug, Clone)]
pub struct InfrastructureFile {
    pub name: String,
    pub content: String,
}
