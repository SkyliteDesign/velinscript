use super::{Template, TemplateConfig};
use anyhow::Result;

/// Deployment Template
/// 
/// Generiert Dockerfile, docker-compose.yml
/// Kubernetes Manifests, etc.
pub struct DeploymentTemplate;

impl Template for DeploymentTemplate {
    fn generate(&self, config: &TemplateConfig) -> Result<String, String> {
        let deployment_type = config.options
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("docker");

        match deployment_type {
            "docker" => Ok(self.generate_dockerfile()),
            "docker-compose" => Ok(self.generate_docker_compose()),
            "kubernetes" => Ok(self.generate_kubernetes()),
            "serverless" => Ok(self.generate_serverless()),
            _ => Ok(self.generate_dockerfile()),
        }
    }
}

impl DeploymentTemplate {
    fn generate_dockerfile(&self) -> String {
        r#"FROM rust:1.70 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/velin-compiler /usr/local/bin/velin-compiler
EXPOSE 3000
CMD ["velin-compiler"]
"#.to_string()
    }

    fn generate_docker_compose(&self) -> String {
        r#"version: '3.8'
services:
  api:
    build: .
    ports:
      - "3000:3000"
    environment:
      - RUST_LOG=info
    depends_on:
      - redis
      - postgres

  redis:
    image: redis:alpine
    ports:
      - "6379:6379"

  postgres:
    image: postgres:15
    environment:
      - POSTGRES_PASSWORD=postgres
    ports:
      - "5432:5432"
"#.to_string()
    }

    fn generate_kubernetes(&self) -> String {
        r#"apiVersion: apps/v1
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
        env:
        - name: RUST_LOG
          value: "info"
---
apiVersion: v1
kind: Service
metadata:
  name: velin-api-service
spec:
  selector:
    app: velin-api
  ports:
  - port: 80
    targetPort: 3000
  type: LoadBalancer
"#.to_string()
    }

    fn generate_serverless(&self) -> String {
        r#"service: velin-api
frameworkVersion: '3'

provider:
  name: aws
  runtime: provided.al2
  architecture: arm64
  memorySize: 512
  timeout: 30
  region: us-east-1
  environment:
    RUST_LOG: info

functions:
  api:
    handler: bootstrap
    events:
      - httpApi: '*'

plugins:
  - serverless-rust

custom:
  rust:
    cargoFlags: '--release'
"#.to_string()
    }
}
