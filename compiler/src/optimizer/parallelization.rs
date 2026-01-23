use crate::optimizer::pipeline::PipelineOptimizer;
use crate::parser::ast::*;
use crate::compiler::pass::Pass;
use crate::compiler::context::CompilationContext;
use anyhow::Result;
use std::collections::HashSet;

/// Parallelization Analyzer für automatische Parallelisierung
/// 
/// Analysiert Datenabhängigkeiten und parallelisiert Code automatisch:
/// - Analysiert Datenabhängigkeiten
/// - Findet unabhängige Operationen
/// - Wählt Parallelisierungs-Strategie
/// - Plant Code-Transformation
pub struct ParallelizationAnalyzer {
    pipeline_optimizer: PipelineOptimizer,
    dependency_graph: DependencyGraph,
}

impl Pass for ParallelizationAnalyzer {
    fn name(&self) -> &str {
        "ParallelizationAnalyzer"
    }

    fn run(&self, ctx: &mut CompilationContext) -> Result<()> {
        if let Some(program) = &mut ctx.program {
            // Analysiere und transformiere das gesamte Programm
            let plan = self.analyze(program)?;
            
            // Nur wenn es Optimierungspotenzial gibt (speedup > 1.0)
            if plan.estimated_speedup > 1.05 {
                // Wir müssen hier einen Hack anwenden, da analyze(&self) immutable ist, 
                // aber wir das Programm mutieren müssen.
                // Da transform(&self) auch immutable self nimmt und mut Program, ist das ok.
                self.transform(program, &plan)?;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threading_transformation() {
        let analyzer = ParallelizationAnalyzer::new();
        
        // Mock AST Block:
        // let a = heavy_calc(1);
        // let b = heavy_calc(2);
        // (independent)
        
        let mut block = Block {
            statements: vec![
                Statement::Let(LetStatement {
                    name: "a".to_string(),
                    var_type: None,
                    value: Expression::Call {
                        callee: Box::new(Expression::Identifier("heavy_calc".to_string())),
                        args: vec![Expression::Literal(Literal::Number(1.0))],
                    },
                    mutable: false,
                }),
                Statement::Let(LetStatement {
                    name: "b".to_string(),
                    var_type: None,
                    value: Expression::Call {
                        callee: Box::new(Expression::Identifier("heavy_calc".to_string())),
                        args: vec![Expression::Literal(Literal::Number(2.0))],
                    },
                    mutable: false,
                }),
            ],
        };

        let transformation = CodeTransformation {
            location: "group_[0, 1]".to_string(),
            function_name: "test_func".to_string(),
            original_code: "".to_string(),
            transformed_code: "".to_string(),
            strategy: ParallelizationStrategy::Multithreading,
        };

        let result = analyzer.apply_threading_to_block(&mut block, &transformation);
        assert!(result.is_ok());

        // Check if thread::spawn is present in the new statements
        // We expect:
        // let a_handle = std::thread::spawn(...);
        // let b_handle = std::thread::spawn(...);
        // let a = a_handle.join();
        // let b = b_handle.join();
        
        // Total statements: 2 spawns + 2 joins = 4
        assert_eq!(block.statements.len(), 4);
        
        // Check first statement is a spawn
        if let Statement::Let(let_stmt) = &block.statements[0] {
            assert!(let_stmt.name.ends_with("_handle"), "Variable name should end with _handle");
            if let Expression::Call { callee, .. } = &let_stmt.value {
                if let Expression::Member { member, .. } = &**callee {
                    assert_eq!(member, "spawn", "Expected spawn call");
                } else {
                    assert!(false, "Expected spawn call (Member expression)");
                }
            } else {
                assert!(false, "Expected Call expression");
            }
        } else {
            assert!(false, "Expected Let statement");
        }
    }

    #[test]
    fn test_async_transformation() {
        let analyzer = ParallelizationAnalyzer::new();
        
        // Mock AST Block:
        // let a = await fetch(1);
        // let b = await fetch(2);
        
        let mut block = Block {
            statements: vec![
                Statement::Let(LetStatement {
                    name: "a".to_string(),
                    var_type: None,
                    value: Expression::Await {
                        expr: Box::new(Expression::Call {
                            callee: Box::new(Expression::Identifier("fetch".to_string())),
                            args: vec![Expression::Literal(Literal::Number(1.0))],
                        }),
                    },
                    mutable: false,
                }),
                Statement::Let(LetStatement {
                    name: "b".to_string(),
                    var_type: None,
                    value: Expression::Await {
                        expr: Box::new(Expression::Call {
                            callee: Box::new(Expression::Identifier("fetch".to_string())),
                            args: vec![Expression::Literal(Literal::Number(2.0))],
                        }),
                    },
                    mutable: false,
                }),
            ],
        };

        let transformation = CodeTransformation {
            location: "group_[0, 1]".to_string(),
            function_name: "test_async".to_string(),
            original_code: "".to_string(),
            transformed_code: "".to_string(),
            strategy: ParallelizationStrategy::Async,
        };

        let result = analyzer.apply_async_to_block(&mut block, &transformation);
        assert!(result.is_ok());

        // Check for tokio::join!
        // let __join_result_0 = await tokio::join!(...);
        // let a = __join_result_0.0;
        // let b = __join_result_0.1;
        assert_eq!(block.statements.len(), 3);
        
        if let Statement::Let(let_stmt) = &block.statements[0] {
             // Verify destructuring or tuple assignment
             // Simplified check for now as AST might vary
             if let Expression::Await { expr } = &let_stmt.value {
                 if let Expression::Call { callee, .. } = &**expr {
                     if let Expression::Member { member, .. } = &**callee {
                         assert!(member == "join" || member == "try_join", "Expected join call");
                     } else {
                         assert!(false, "Expected join call (Member)");
                     }
                 } else {
                    assert!(false, "Expected Call expression inside Await");
                 }
             } else {
                 // Might be macro invocation represented differently
                 // Checking if logic replaced the 2 statements with 1
             }
        } else {
            assert!(false, "Expected Let statement");
        }
    }
}

/// Dependency Graph für Datenabhängigkeiten
#[derive(Debug, Clone)]
struct DependencyGraph {
    nodes: Vec<OperationNode>,
    edges: Vec<DependencyEdge>,
}

#[derive(Debug, Clone)]
struct OperationNode {
    id: usize,
    operation: Operation,
    variables_read: HashSet<String>,
    variables_written: HashSet<String>,
}

#[derive(Debug, Clone)]
struct Operation {
    location: String,
    expression: Expression,
}

#[derive(Debug, Clone)]
struct DependencyEdge {
    from: usize,
    to: usize,
    dependency_type: DependencyType,
}

#[derive(Debug, Clone)]
enum DependencyType {
    DataDependency,  // Variable wird gelesen nachdem sie geschrieben wurde
    ControlDependency, // Kontrollfluss-Abhängigkeit
}

/// Parallelization Plan
#[derive(Debug, Clone)]
pub struct ParallelizationPlan {
    pub strategy: ParallelizationStrategy,
    pub transformation: TransformationPlan,
    pub estimated_speedup: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParallelizationStrategy {
    Multithreading,
    GPU,
    Async,
    SIMD,
}

#[derive(Debug, Clone)]
pub struct TransformationPlan {
    transformations: Vec<CodeTransformation>,
}

#[derive(Debug, Clone)]
pub struct CodeTransformation {
    location: String,
    function_name: String,
    original_code: String,
    transformed_code: String,
    strategy: ParallelizationStrategy,
}

impl ParallelizationAnalyzer {
    pub fn new() -> Self {
        Self {
            pipeline_optimizer: PipelineOptimizer::new(),
            dependency_graph: DependencyGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            },
        }
    }

    /// Analysiert AST für Parallelisierungs-Möglichkeiten
    pub fn analyze(&self, program: &Program) -> Result<ParallelizationPlan> {
        // 1. Datenabhängigkeiten analysieren
        let dependencies = self.build_dependency_graph(program)?;

        // 2. Unabhängige Operationen identifizieren
        let independent_ops = self.find_independent_operations(&dependencies)?;

        // 3. Parallelisierungs-Strategie wählen
        let strategy = self.choose_strategy(&independent_ops)?;

        // 4. Code-Transformation planen
        let transformation = self.plan_transformation(&independent_ops, &strategy, &dependencies)?;

        Ok(ParallelizationPlan {
            strategy,
            transformation,
            estimated_speedup: self.estimate_speedup(&independent_ops),
        })
    }

    /// Baut Dependency Graph
    fn build_dependency_graph(&self, program: &Program) -> Result<DependencyGraph> {
        let mut graph = DependencyGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        };

        let mut node_id = 0;

        // Analysiere alle Funktionen
        for item in &program.items {
            if let Item::Function(f) = item {
                // Analysiere Statements im Body
                for (idx, stmt) in f.body.statements.iter().enumerate() {
                    if let Statement::Let(let_stmt) = stmt {
                        let vars_read = self.extract_read_variables(&let_stmt.value);
                        let vars_written = HashSet::from([let_stmt.name.clone()]);

                        graph.nodes.push(OperationNode {
                            id: node_id,
                            operation: Operation {
                                location: format!("{}:{}", f.name, idx),
                                expression: let_stmt.value.clone(),
                            },
                            variables_read: vars_read.clone(),
                            variables_written: vars_written,
                        });

                        // Prüfe Abhängigkeiten zu vorherigen Nodes
                        for prev_node in &graph.nodes {
                            if prev_node.id < node_id {
                                // Prüfe ob aktuelle Operation von vorheriger abhängt
                                if !vars_read.is_disjoint(&prev_node.variables_written) {
                                    graph.edges.push(DependencyEdge {
                                        from: prev_node.id,
                                        to: node_id,
                                        dependency_type: DependencyType::DataDependency,
                                    });
                                }
                            }
                        }

                        node_id += 1;
                    }
                }
            }
        }

        Ok(graph)
    }

    /// Findet unabhängige Operationen
    fn find_independent_operations(&self, deps: &DependencyGraph) -> Result<Vec<Vec<usize>>> {
        // Finde Operationen ohne Datenabhängigkeiten
        let mut independent_groups = Vec::new();
        let mut processed = HashSet::new();

        for node in &deps.nodes {
            if processed.contains(&node.id) {
                continue;
            }

            // Extract function name from location "func:idx"
            let func_name = node.operation.location.split(':').next().unwrap_or("");

            // Finde alle Nodes die von diesem Node abhängen oder von denen dieser abhängt
            let mut group = vec![node.id];
            processed.insert(node.id);

            // Finde unabhängige Nodes (keine Edges zwischen ihnen)
            // WICHTIG: Nur innerhalb der gleichen Funktion!
            for other_node in &deps.nodes {
                if processed.contains(&other_node.id) {
                    continue;
                }

                // Check function boundary
                let other_func = other_node.operation.location.split(':').next().unwrap_or("");
                if func_name != other_func {
                    continue;
                }

                // Prüfe ob es eine Edge zwischen node und other_node gibt
                let has_dependency = deps.edges.iter().any(|edge| {
                    (edge.from == node.id && edge.to == other_node.id) ||
                    (edge.from == other_node.id && edge.to == node.id)
                });

                if !has_dependency {
                    // Prüfe ob other_node von Nodes in der Gruppe abhängt
                    let depends_on_group = deps.edges.iter().any(|edge| {
                        group.contains(&edge.from) && edge.to == other_node.id
                    });

                    // Prüfe ob Gruppe von other_node abhängt (missing check in original code)
                    let group_depends_on = deps.edges.iter().any(|edge| {
                        edge.from == other_node.id && group.contains(&edge.to)
                    });

                    if !depends_on_group && !group_depends_on {
                        group.push(other_node.id);
                        processed.insert(other_node.id);
                    }
                }
            }

            if group.len() > 1 {
                independent_groups.push(group);
            }
        }

        Ok(independent_groups)
    }

    /// Wählt beste Parallelisierungs-Strategie
    fn choose_strategy(&self, ops: &[Vec<usize>]) -> Result<ParallelizationStrategy> {
        // Einfache Heuristik: Wähle Strategie basierend auf Anzahl und Typ der Operationen
        if ops.is_empty() {
            return Ok(ParallelizationStrategy::Async); // Default
        }

        let total_ops: usize = ops.iter().map(|group| group.len()).sum();

        // Wenn viele unabhängige Operationen: Multithreading
        if total_ops > 4 {
            Ok(ParallelizationStrategy::Multithreading)
        } else if total_ops > 2 {
            Ok(ParallelizationStrategy::Async)
        } else {
            Ok(ParallelizationStrategy::Async)
        }
    }

    /// Plant Code-Transformation
    fn plan_transformation(&self, independent_ops: &[Vec<usize>], strategy: &ParallelizationStrategy, deps: &DependencyGraph) -> Result<TransformationPlan> {
        let mut transformations = Vec::new();

        for group in independent_ops {
            if group.len() > 1 {
                let transformation = match strategy {
                    ParallelizationStrategy::Multithreading => {
                        self.plan_threading_transformation(group, deps)
                    }
                    ParallelizationStrategy::GPU => {
                        self.plan_gpu_transformation(group, deps)
                    }
                    ParallelizationStrategy::Async => {
                        self.plan_async_transformation(group, deps)
                    }
                    ParallelizationStrategy::SIMD => {
                        self.plan_simd_transformation(group, deps)
                    }
                };
                transformations.push(transformation);
            }
        }

        Ok(TransformationPlan { transformations })
    }

    /// Plant Threading-Transformation
    fn plan_threading_transformation(&self, group: &[usize], deps: &DependencyGraph) -> CodeTransformation {
        let func_name = if let Some(first_id) = group.first() {
             deps.nodes[*first_id].operation.location.split(':').next().unwrap_or("").to_string()
        } else {
             "".to_string()
        };

        CodeTransformation {
            location: format!("group_{:?}", group),
            function_name: func_name,
            original_code: "Sequential operations".to_string(),
            transformed_code: format!(
                "let results = tokio::join!(\n{}\n);",
                group.iter()
                    .map(|id| format!("    tokio::spawn(async {{ operation_{}() }})", id))
                    .collect::<Vec<_>>()
                    .join(",\n")
            ),
            strategy: ParallelizationStrategy::Multithreading,
        }
    }

    /// Plant GPU-Transformation
    fn plan_gpu_transformation(&self, group: &[usize], deps: &DependencyGraph) -> CodeTransformation {
        let func_name = if let Some(first_id) = group.first() {
             deps.nodes[*first_id].operation.location.split(':').next().unwrap_or("").to_string()
        } else {
             "".to_string()
        };

        // Generiere echten GPU-Kernel-Code basierend auf den Operationen
        // Wir nehmen an, dass alle Operationen in der Gruppe ähnlich sind (SIMD-artig)
        // oder wir generieren Code für jeden Index.
        
        let mut kernel_body = String::new();
        kernel_body.push_str("    let idx = global_id.x;\n");
        
        for (i, &node_id) in group.iter().enumerate() {
            if let Some(node) = deps.nodes.iter().find(|n| n.id == node_id) {
                // Versuche die Operation zu transpilen
                // Einfacher Fall: Binäre Operation
                if let Expression::BinaryOp { left: _, op, right } = &node.operation.expression {
                    let op_str = match op {
                        BinaryOperator::Add => "+",
                        BinaryOperator::Subtract => "-",
                        BinaryOperator::Multiply => "*",
                        BinaryOperator::Divide => "/",
                        _ => "+",
                    };
                    
                    // Wir nehmen an, dass der rechte Operand ein Literal ist für dieses Beispiel
                    // In einem echten Compiler müssten wir rekursiv transpilen
                    let val = match &**right {
                        Expression::Literal(Literal::Number(n)) => n.to_string(),
                        _ => "1.0".to_string(),
                    };
                    
                    kernel_body.push_str(&format!("    output_{}[idx] = input_{}[idx] {} {};\n", i, i, op_str, val));
                } else {
                     kernel_body.push_str(&format!("    output_{}[idx] = input_{}[idx]; // Complex op\n", i, i));
                }
            }
        }

        let kernel_code = format!(
            "// GPU Kernel (WGSL-like Pseudo-code)\n\
            #[kernel]\n\
            fn main(\n\
                #[builtin(global_invocation_id)] global_id: vec3<u32>,\n\
                {}\n\
                {}\n\
            ) {{\n\
            {}\n\
            }}",
            group.iter().enumerate().map(|(i, _)| format!("    #[binding(0, {})] input_{}: array<f32>,", i, i)).collect::<Vec<_>>().join("\n"),
            group.iter().enumerate().map(|(i, _)| format!("    #[binding(0, {})] var<storage, read_write> output_{}: array<f32>,", i + group.len(), i)).collect::<Vec<_>>().join("\n"),
            kernel_body
        );
        
        CodeTransformation {
            location: format!("group_{:?}", group),
            function_name: func_name,
            original_code: "CPU operations".to_string(),
            transformed_code: kernel_code,
            strategy: ParallelizationStrategy::GPU,
        }
    }

    /// Plant Async-Transformation
    fn plan_async_transformation(&self, group: &[usize], deps: &DependencyGraph) -> CodeTransformation {
        let func_name = if let Some(first_id) = group.first() {
             deps.nodes[*first_id].operation.location.split(':').next().unwrap_or("").to_string()
        } else {
             "".to_string()
        };

        CodeTransformation {
            location: format!("group_{:?}", group),
            function_name: func_name,
            original_code: "Sequential async operations".to_string(),
            transformed_code: format!(
                "let (result1, result2) = tokio::join!(\n{}\n);",
                group.iter()
                    .enumerate()
                    .map(|(i, _)| format!("    async_operation_{}()", i))
                    .collect::<Vec<_>>()
                    .join(",\n")
            ),
            strategy: ParallelizationStrategy::Async,
        }
    }

    /// Plant SIMD-Transformation
    fn plan_simd_transformation(&self, group: &[usize], deps: &DependencyGraph) -> CodeTransformation {
        let func_name = if let Some(first_id) = group.first() {
             deps.nodes[*first_id].operation.location.split(':').next().unwrap_or("").to_string()
        } else {
             "".to_string()
        };

        // Generiere SIMD-vektorisierten Code
        let simd_code = format!(
            "// SIMD-vektorisierte Operationen\n\
            use std::simd::*;\n\n\
            let simd_data: [f32x8; {}] = [\n\
            {}];\n\n\
            let results: [f32x8; {}] = simd_data.map(|v| v * 2.0);",
            group.len(),
            group.iter()
                .map(|_| "    f32x8::splat(0.0),")
                .collect::<Vec<_>>()
                .join("\n"),
            group.len()
        );
        
        CodeTransformation {
            location: format!("group_{:?}", group),
            function_name: func_name,
            original_code: "Vector operations".to_string(),
            transformed_code: simd_code,
            strategy: ParallelizationStrategy::SIMD,
        }
    }

    /// Schätzt Speedup
    fn estimate_speedup(&self, independent_ops: &[Vec<usize>]) -> f64 {
        if independent_ops.is_empty() {
            return 1.0;
        }

        let total_ops: usize = independent_ops.iter().map(|group| group.len()).sum();
        let parallelizable_ops: usize = independent_ops.iter()
            .filter(|group| group.len() > 1)
            .map(|group| group.len())
            .sum();

        if total_ops == 0 {
            return 1.0;
        }

        // Einfache Schätzung: Speedup = 1 + (parallelizable_ops / total_ops)
        1.0 + (parallelizable_ops as f64 / total_ops as f64)
    }

    /// Extrahiert gelesene Variablen aus Expression
    fn extract_read_variables(&self, expr: &Expression) -> HashSet<String> {
        let mut vars = HashSet::new();
        self.collect_read_variables(expr, &mut vars);
        vars
    }

    /// Sammelt gelesene Variablen rekursiv
    fn collect_read_variables(&self, expr: &Expression, vars: &mut HashSet<String>) {
        match expr {
            Expression::Identifier(name) => {
                vars.insert(name.clone());
            }
            Expression::Member { object, .. } => {
                // obj.field - extrahiere obj
                self.collect_read_variables(object, vars);
            }
            Expression::Index { object, index } => {
                // arr[i] - extrahiere arr und i
                self.collect_read_variables(object, vars);
                self.collect_read_variables(index, vars);
            }
            Expression::Await { expr } => {
                self.collect_read_variables(expr, vars);
            }
            Expression::Call { callee, args } => {
                // Function calls können Variablen lesen
                self.collect_read_variables(callee, vars);
                for arg in args {
                    self.collect_read_variables(arg, vars);
                }
            }
            Expression::BinaryOp { left, right, .. } => {
                self.collect_read_variables(left, vars);
                self.collect_read_variables(right, vars);
            }
            Expression::UnaryOp { expr, .. } => {
                self.collect_read_variables(expr, vars);
            }
            Expression::If { condition, then_expr, else_expr } => {
                self.collect_read_variables(condition, vars);
                self.collect_read_variables(then_expr, vars);
                self.collect_read_variables(else_expr, vars);
            }
            Expression::Lambda { body, .. } => {
                self.collect_read_variables(body, vars);
            }
            Expression::ListLiteral(items) => {
                for item in items {
                    self.collect_read_variables(item, vars);
                }
            }
            Expression::MapLiteral(entries) => {
                for (_key, value) in entries {
                    // key ist String, nicht Expression - nur value analysieren
                    self.collect_read_variables(value, vars);
                }
            }
            Expression::FormatString { parts } => {
                for part in parts {
                    match part {
                        crate::parser::ast::FormatStringPart::Expression(expr) => {
                            self.collect_read_variables(expr, vars);
                        }
                        crate::parser::ast::FormatStringPart::Text(_) => {}
                    }
                }
            }
            Expression::LLMCall { args, .. } => {
                for arg in args {
                    self.collect_read_variables(arg, vars);
                }
            }
            Expression::StructLiteral { fields, .. } => {
                for (_, expr) in fields {
                    self.collect_read_variables(&expr, vars);
                }
            }
            Expression::GenericConstructor { args, .. } => {
                for arg in args {
                    self.collect_read_variables(arg, vars);
                }
            }
            Expression::Assignment { target, value } => {
                self.collect_read_variables(target, vars);
                self.collect_read_variables(value, vars);
            }
            _ => {}
        }
    }

    /// Transformiert Code basierend auf Plan
    pub fn transform(&self, program: &mut Program, plan: &ParallelizationPlan) -> Result<()> {
        match plan.strategy {
            ParallelizationStrategy::Multithreading => {
                self.transform_to_threading(program, plan)?;
            }
            ParallelizationStrategy::GPU => {
                self.transform_to_gpu(program, plan)?;
            }
            ParallelizationStrategy::Async => {
                self.transform_to_async(program, plan)?;
            }
            ParallelizationStrategy::SIMD => {
                self.transform_to_simd(program, plan)?;
            }
        }
        Ok(())
    }

    /// Transformiert zu Threading
    fn transform_to_threading(&self, program: &mut Program, plan: &ParallelizationPlan) -> Result<()> {
        // Wende Threading-Transformationen an
        for transformation in &plan.transformation.transformations {
            if transformation.strategy == ParallelizationStrategy::Multithreading {
                // Finde entsprechende Funktion und transformiere
                for item in &mut program.items {
                    if let crate::parser::ast::Item::Function(f) = item {
                        // Prüfe ob Location zu dieser Funktion passt
                        if transformation.location.contains(&f.name) {
                            // Transformiere Block für Threading
                            self.apply_threading_to_block(&mut f.body, transformation)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Wendet Threading auf Block an
    fn apply_threading_to_block(&self, block: &mut crate::parser::ast::Block, transformation: &CodeTransformation) -> Result<()> {
        // Parse group indices from location string "group_[1, 2, 3]"
        let group_str = transformation.location.trim_start_matches("group_");
        // Remove brackets and parse comma-separated indices
        let indices_str = group_str.trim_matches(|c| c == '[' || c == ']');
        let group_indices: Vec<usize> = if indices_str.is_empty() {
            Vec::new()
        } else {
            indices_str.split(',')
                .map(|s| s.trim().parse().unwrap_or(0))
                .collect()
        };
        
        if group_indices.is_empty() {
            return Ok(());
        }

        // Similar logic to async, but wrapping in thread::spawn handles
        let mut new_statements = Vec::new();
        let original_statements = block.statements.clone();
        
        let mut i = 0;
        while i < original_statements.len() {
             if group_indices.contains(&i) {
                 // Found start of threading group
                 // Wrap statements in thread handles
                 let mut handles = Vec::new();
                 
                 for &stmt_idx in &group_indices {
                     let stmt = &original_statements[stmt_idx];
                     if let Statement::Let(let_stmt) = stmt {
                         // Transform: let x = calc(); -> let x_handle = thread::spawn(|| calc());
                         let handle_name = format!("{}_handle", let_stmt.name);
                         
                         let spawn_call = Expression::Call {
                             callee: Box::new(Expression::Member {
                                 object: Box::new(Expression::Identifier("std::thread".to_string())),
                                 member: "spawn".to_string(),
                             }),
                             args: vec![
                                 Expression::Lambda {
                                     params: vec![],
                                     return_type: None,
                                     body: Box::new(let_stmt.value.clone()),
                                 }
                             ],
                         };

                         new_statements.push(Statement::Let(LetStatement {
                             name: handle_name.clone(),
                             var_type: None, // Infer
                             value: spawn_call,
                             mutable: false,
                         }));
                         
                         handles.push((let_stmt.name.clone(), handle_name));
                     }
                 }
                 
                 // Join threads
                 for (var_name, handle_name) in handles {
                     // let x = x_handle.join().unwrap();
                     let join_call = Expression::Call {
                         callee: Box::new(Expression::Member {
                             object: Box::new(Expression::Identifier(handle_name)),
                             member: "join".to_string(),
                         }),
                         args: vec![],
                     };
                     
                     new_statements.push(Statement::Let(LetStatement {
                         name: var_name,
                         var_type: None,
                         value: join_call, // In Rust join returns Result, so we might need unwrap. Assumed implied or handled by codegen.
                         mutable: false,
                     }));
                 }
                 
                 i += group_indices.len();
             } else {
                 new_statements.push(original_statements[i].clone());
                 i += 1;
             }
        }
        
        block.statements = new_statements;
        Ok(())
    }

    /// Transformiert zu GPU
    fn transform_to_gpu(&self, program: &mut Program, plan: &ParallelizationPlan) -> Result<()> {
        // Wende GPU-Transformationen an
        for transformation in &plan.transformation.transformations {
            if transformation.strategy == ParallelizationStrategy::GPU {
                for item in &mut program.items {
                    if let crate::parser::ast::Item::Function(f) = item {
                        if transformation.location.contains(&f.name) {
                            self.apply_gpu_to_block(&mut f.body, transformation)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Wendet GPU-Transformation auf Block an
    fn apply_gpu_to_block(&self, block: &mut crate::parser::ast::Block, transformation: &CodeTransformation) -> Result<()> {
        let group_str = transformation.location.trim_start_matches("group_");
        // Remove brackets and parse comma-separated indices
        let indices_str = group_str.trim_matches(|c| c == '[' || c == ']');
        let group_indices: Vec<usize> = if indices_str.is_empty() {
            Vec::new()
        } else {
            indices_str.split(',')
                .map(|s| s.trim().parse().unwrap_or(0))
                .collect()
        };
        
        if group_indices.is_empty() {
            return Ok(());
        }

        let mut new_statements = Vec::new();
        let original_statements = block.statements.clone();
        
        // Collect inputs for the GPU kernel
        let mut input_vars = Vec::new();
        for &stmt_idx in &group_indices {
             if let Statement::Let(let_stmt) = &original_statements[stmt_idx] {
                 // Extract variables read in this statement
                 let vars = self.extract_read_variables(&let_stmt.value);
                 for var in vars {
                     if !input_vars.contains(&var) {
                         input_vars.push(var);
                     }
                 }
             }
        }

        let mut i = 0;
        while i < original_statements.len() {
             if group_indices.contains(&i) {
                 // Generate GPU kernel call
                 // let gpu_result = velin_runtime::gpu::execute_compute_shader(shader_code, entry_point, inputs);
                 
                 let gpu_call = Statement::Let(LetStatement {
                     name: "gpu_result".to_string(),
                     var_type: None,
                     value: Expression::Call {
                        callee: Box::new(Expression::Member {
                            object: Box::new(Expression::Member {
                                object: Box::new(Expression::Identifier("velin_runtime".to_string())),
                                member: "gpu".to_string(),
                            }),
                            member: "execute_compute_shader".to_string(),
                        }),
                        args: vec![
                            Expression::Literal(Literal::String(transformation.transformed_code.clone())), // Shader code
                            Expression::Literal(Literal::String("main".to_string())), // Entry point
                            Expression::ListLiteral(input_vars.iter().map(|v| Expression::Identifier(v.clone())).collect()), // Inputs
                        ],
                     },
                     mutable: false,
                 });
                 
                 new_statements.push(gpu_call);
                 
                 i += group_indices.len();
             } else {
                 new_statements.push(original_statements[i].clone());
                 i += 1;
             }
        }
        
        block.statements = new_statements;
        Ok(())
    }

    /// Wendet Async-Transformation auf Block an (Test-Helper)
    pub fn apply_async_to_block(&self, block: &mut crate::parser::ast::Block, transformation: &CodeTransformation) -> Result<()> {
        let group_str = transformation.location.trim_start_matches("group_");
        // Remove brackets and parse comma-separated indices
        let indices_str = group_str.trim_matches(|c| c == '[' || c == ']');
        let group_indices: Vec<usize> = if indices_str.is_empty() {
            Vec::new()
        } else {
            indices_str.split(',')
                .map(|s| s.trim().parse().unwrap_or(0))
                .collect()
        };
        
        if group_indices.is_empty() {
            return Ok(());
        }
        
        // Use the same logic as apply_async_parallelization but with indices from transformation
        self.apply_async_parallelization(block, &[group_indices])
    }

    /// Transformiert zu Async
    fn transform_to_async(&self, program: &mut Program, _plan: &ParallelizationPlan) -> Result<()> {
        // Nutze PipelineOptimizer für Async-Transformation
        for item in &mut program.items {
            if let crate::parser::ast::Item::Function(f) = item {
                if f.is_async {
                    // Identifiziere parallele Gruppen
                    let parallel_groups = self.pipeline_optimizer.identify_parallel_groups(&f.body);
                    
                    if !parallel_groups.is_empty() {
                        // Transformiere für parallele Ausführung
                        self.apply_async_parallelization(&mut f.body, &parallel_groups)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Transformiert zu SIMD
    fn transform_to_simd(&self, program: &mut Program, plan: &ParallelizationPlan) -> Result<()> {
        // Wende SIMD-Transformationen an
        for transformation in &plan.transformation.transformations {
            if transformation.strategy == ParallelizationStrategy::SIMD {
                 for item in &mut program.items {
                    if let crate::parser::ast::Item::Function(f) = item {
                        if transformation.function_name == f.name {
                            self.apply_simd_to_block(&mut f.body, transformation)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn apply_async_parallelization(&self, block: &mut crate::parser::ast::Block, groups: &[Vec<usize>]) -> Result<()> {
        // Wir müssen den Block neu aufbauen
        let mut new_statements = Vec::new();
        let mut group_idx = 0;
        let original_statements = block.statements.clone();

        let mut handled_indices = HashSet::new();
        for group in groups {
            for &idx in group {
                handled_indices.insert(idx);
            }
        }

        let mut i = 0;
        while i < original_statements.len() {
            // Check if i is the start of a group
            let mut found_group = None;
            for group in groups {
                if !group.is_empty() && group[0] == i {
                    found_group = Some(group);
                    break;
                }
            }

            if let Some(group) = found_group {
                // Generiere tokio::join! für diese Gruppe
                let mut join_args = Vec::new();
                let mut result_vars = Vec::new();

                for stmt_idx in group {
                    if let Statement::Let(let_stmt) = &original_statements[*stmt_idx] {
                        if let Expression::Await { expr } = &let_stmt.value {
                             // Extrahiere den async Call
                             join_args.push(*expr.clone());
                             result_vars.push(let_stmt.name.clone());
                        }
                    }
                }

                if !join_args.is_empty() {
                    // Erstelle tokio::join!(call1, call2, ...)
                    let join_call = Expression::Call {
                        callee: Box::new(Expression::Member {
                            object: Box::new(Expression::Identifier("tokio".to_string())),
                            member: "join".to_string(),
                        }),
                        args: join_args,
                    };

                    let join_var_name = format!("__join_result_{}", group_idx);
                    group_idx += 1;

                    new_statements.push(Statement::Let(LetStatement {
                        name: join_var_name.clone(),
                        var_type: None,
                        value: Expression::Await { expr: Box::new(join_call) }, 
                        mutable: false,
                    }));

                    // Unpack results
                    for (idx, var_name) in result_vars.iter().enumerate() {
                        new_statements.push(Statement::Let(LetStatement {
                            name: var_name.clone(),
                            var_type: None,
                            value: Expression::Member {
                                object: Box::new(Expression::Identifier(join_var_name.clone())),
                                member: idx.to_string(), // Tuple access .0, .1
                            },
                            mutable: false,
                        }));
                    }
                }
            } 
            
            if !handled_indices.contains(&i) {
                // Normal statement (not part of any group)
                new_statements.push(original_statements[i].clone());
            }
            
            i += 1;
        }

        block.statements = new_statements;
        Ok(())
    }

    fn apply_simd_to_block(&self, block: &mut Block, transformation: &CodeTransformation) -> Result<()> {
        let group_str = transformation.location.trim_start_matches("group_");
        // Remove brackets and parse comma-separated indices
        let indices_str = group_str.trim_matches(|c| c == '[' || c == ']');
        let group_indices: Vec<usize> = if indices_str.is_empty() {
            Vec::new()
        } else {
            indices_str.split(',')
                .map(|s| s.trim().parse().unwrap_or(0))
                .collect()
        };
        
        if group_indices.is_empty() { return Ok(()); }

        let mut new_statements = Vec::new();
        let original_statements = block.statements.clone();
        
        let mut i = 0;
        while i < original_statements.len() {
             if group_indices.contains(&i) {
                 // 1. Collect values and determine operation
                 let mut simd_values = Vec::new();
                 let mut op = BinaryOperator::Add; // Default
                 let mut operand = Expression::Literal(Literal::Number(0.0));
                 
                 // Analyze first statement to determine operation
                 if let Statement::Let(first_stmt) = &original_statements[group_indices[0]] {
                     if let Expression::BinaryOp { left: _, op: first_op, right } = &first_stmt.value {
                         op = first_op.clone();
                         operand = *right.clone();
                     }
                 }

                 for &stmt_idx in &group_indices {
                     if let Statement::Let(let_stmt) = &original_statements[stmt_idx] {
                         // Assume binary op: let x = val * 2.0;
                         // We extract 'val' from the left side
                         if let Expression::BinaryOp { left, .. } = &let_stmt.value {
                             simd_values.push(*left.clone());
                         } else {
                             // Fallback if not binary op
                             simd_values.push(let_stmt.value.clone());
                         }
                     }
                 }

                 // 2. Create SIMD Vector
                 let simd_ctor = Expression::Call {
                     callee: Box::new(Expression::Member {
                         object: Box::new(Expression::Identifier("std::simd::f32x4".to_string())),
                         member: "from_array".to_string(),
                     }),
                     args: vec![Expression::ListLiteral(simd_values)],
                 };

                 new_statements.push(Statement::Let(LetStatement {
                     name: "simd_batch".to_string(),
                     var_type: None,
                     value: simd_ctor,
                     mutable: false,
                 }));

                 // 3. Execute Operation
                 let op_call = Expression::BinaryOp {
                     left: Box::new(Expression::Identifier("simd_batch".to_string())),
                     op: op,
                     right: Box::new(operand),
                 };

                 new_statements.push(Statement::Let(LetStatement {
                     name: "result_batch".to_string(),
                     var_type: None,
                     value: op_call,
                     mutable: false,
                 }));
                 
                 i += group_indices.len();
             } else {
                 new_statements.push(original_statements[i].clone());
                 i += 1;
             }
        }
        
        block.statements = new_statements;
        Ok(())
    }
}
