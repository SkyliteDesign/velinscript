
use crate::parser::ast::*;
use std::collections::HashSet;

pub struct PipelineOptimizer;

impl PipelineOptimizer {
    pub fn new() -> Self {
        PipelineOptimizer
    }

    pub fn analyze_module(&self, module: &mut Module) {
        // Only optimize modules marked with @VelinPipeline
        if !module.items.iter().any(|i| {
            if let Item::Struct(s) = i {
                s.decorators.iter().any(|d| d.name == "VelinPipeline")
            } else if let Item::Function(f) = i {
                 f.decorators.iter().any(|d| d.name == "VelinPipeline")
            } else {
                false
            }
        }) {
            // Check if the module itself has a decorator? AST doesn't support module decorators yet.
            // We'll rely on a top-level comment or assume usage on functions for now.
            // Or better: We assume the user puts @VelinPipeline on the main function orchestrating the flow.
            return;
        }

        // Optimization logic would go here
        // 1. Build dependency graph of function calls
        // 2. Identify independent sub-graphs
        // 3. Mark them for parallel execution
    }

    /// Identifies independent function calls in a block that can be parallelized
    /// Returns a list of groups, where each group is a list of statement indices that can run in parallel
    pub fn identify_parallel_groups(&self, block: &Block) -> Vec<Vec<usize>> {
        let mut parallel_groups = Vec::new();
        let mut current_group = Vec::new();
        let mut defined_vars: HashSet<String> = HashSet::new();
        let mut used_vars: HashSet<String> = HashSet::new();

        for (idx, stmt) in block.statements.iter().enumerate() {
            if let Statement::Let(let_stmt) = stmt {
                // Extract variables used in the expression
                let expr_used_vars = self.extract_used_variables(&let_stmt.value);
                used_vars.extend(expr_used_vars.clone());
                
                // Check if this let statement depends on any currently "pending" parallel group
                // For MVP, we use a simple heuristic:
                // If it's an async call (await), it's a candidate for parallelization
                if let Expression::Await { expr } = &let_stmt.value {
                    if let Expression::Call { .. } = expr.as_ref() {
                        // Check if this statement depends on variables defined in current group
                        let has_dependency = expr_used_vars.iter().any(|var| defined_vars.contains(var));
                        
                        if !has_dependency {
                            // It's an async call without dependencies on current group.
                            current_group.push(idx);
                            // Track the variable being defined
                            defined_vars.insert(let_stmt.name.clone());
                            continue;
                        }
                    }
                }
            }

            // Barrier: If we hit a non-async-call statement, flush the current group
            if !current_group.is_empty() {
                if current_group.len() > 1 {
                    parallel_groups.push(current_group.clone());
                }
                current_group.clear();
                // Clear defined vars when we flush a group
                defined_vars.clear();
            }
        }
        
        if current_group.len() > 1 {
            parallel_groups.push(current_group);
        }

        parallel_groups
    }

    /// Extracts variable names used in an expression
    fn extract_used_variables(&self, expr: &Expression) -> HashSet<String> {
        let mut vars = HashSet::new();
        self.collect_variables(expr, &mut vars);
        vars
    }

    /// Recursively collects variable identifiers from an expression
    fn collect_variables(&self, expr: &Expression, vars: &mut HashSet<String>) {
        match expr {
            Expression::Identifier(name) => {
                vars.insert(name.clone());
            }
            Expression::Await { expr } => {
                self.collect_variables(expr, vars);
            }
            Expression::Call { callee, args } => {
                self.collect_variables(callee, vars);
                for arg in args {
                    self.collect_variables(arg, vars);
                }
            }
            Expression::Member { object, .. } => {
                self.collect_variables(object, vars);
            }
            Expression::BinaryOp { left, right, .. } => {
                self.collect_variables(left, vars);
                self.collect_variables(right, vars);
            }
            Expression::UnaryOp { expr, .. } => {
                self.collect_variables(expr, vars);
            }
            Expression::If { condition, then_expr, else_expr } => {
                self.collect_variables(condition, vars);
                self.collect_variables(then_expr, vars);
                self.collect_variables(else_expr, vars);
            }
            Expression::Index { object, index } => {
                self.collect_variables(object, vars);
                self.collect_variables(index, vars);
            }
            Expression::StructLiteral { fields, .. } => {
                for (_, expr) in fields {
                    self.collect_variables(expr, vars);
                }
            }
            Expression::MapLiteral(fields) => {
                for (_, expr) in fields {
                    self.collect_variables(expr, vars);
                }
            }
            Expression::ListLiteral(items) => {
                for item in items {
                    self.collect_variables(item, vars);
                }
            }
            Expression::FormatString { parts } => {
                for part in parts {
                    if let crate::parser::ast::FormatStringPart::Expression(expr) = part {
                        self.collect_variables(expr, vars);
                    }
                }
            }
            _ => {
                // Other expression types don't contain variables we care about
            }
        }
    }
}
