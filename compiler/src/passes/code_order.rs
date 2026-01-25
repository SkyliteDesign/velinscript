use crate::compiler::context::CompilationContext;
use crate::compiler::pass::Pass;
use crate::parser::ast::*;
use anyhow::Result;
use indexmap::IndexMap;
use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet};

pub struct CodeOrderingPass;

impl CodeOrderingPass {
    pub fn new() -> Self {
        Self
    }

    fn order_program(&self, program: &mut Program) -> Result<()> {
        // Build dependency graph
        let (graph, _node_map, item_map) = self.build_dependency_graph(&program.items)?;

        // Perform topological sort
        let sorted_indices = match toposort(&graph, None) {
            Ok(indices) => indices,
            Err(cycle) => {
                // Circular dependency detected
                let cycle_node = graph[cycle.node_id()].clone();
                return Err(anyhow::anyhow!(
                    "Circular dependency detected involving: {:?}",
                    cycle_node
                ));
            }
        };

        // Reorder items based on topological sort
        let mut ordered_items = Vec::new();
        let mut item_map_mut = item_map;
        for idx in sorted_indices {
            if let Some(item) = item_map_mut.remove(&idx) {
                ordered_items.push(item);
            }
        }

        // Add any items that weren't in the graph (shouldn't happen, but safety check)
        let graph_item_names: HashSet<String> = item_map_mut
            .values()
            .filter_map(|item| self.get_item_name(item))
            .collect();

        for item in &program.items {
            if let Some(name) = self.get_item_name(item) {
                if !graph_item_names.contains(&name) {
                    ordered_items.push(item.clone());
                }
            }
        }

        program.items = ordered_items;
        Ok(())
    }

    fn build_dependency_graph(
        &self,
        items: &[Item],
    ) -> Result<(
        DiGraph<String, ()>,
        IndexMap<String, NodeIndex>,
        HashMap<NodeIndex, Item>,
    )> {
        let mut graph = DiGraph::new();
        let mut node_map = IndexMap::new();
        let mut item_map = HashMap::new();

        // First pass: Add all items as nodes
        for item in items {
            if let Some(name) = self.get_item_name(item) {
                if !node_map.contains_key(&name) {
                    let idx = graph.add_node(name.clone());
                    node_map.insert(name.clone(), idx);
                    item_map.insert(idx, item.clone());
                }
            }
        }

        // Second pass: Add edges based on dependencies
        for item in items {
            if let Some(item_name) = self.get_item_name(item) {
                let item_idx = *node_map.get(&item_name).unwrap();
                let dependencies = self.extract_dependencies(item);

                for dep in dependencies {
                    if let Some(dep_idx) = node_map.get(&dep) {
                        // Add edge: dependency -> item (item depends on dependency)
                        graph.add_edge(*dep_idx, item_idx, ());
                    }
                }
            }
        }

        Ok((graph, node_map, item_map))
    }

    fn get_item_name(&self, item: &Item) -> Option<String> {
        match item {
            Item::Function(f) => Some(f.name.clone()),
            Item::Struct(s) => Some(s.name.clone()),
            Item::Enum(e) => Some(e.name.clone()),
            Item::TypeAlias(ta) => Some(ta.name.clone()),
            Item::Trait(t) => Some(t.name.clone()),
            Item::Module(m) => Some(m.name.clone()),
            Item::Use(_) => None, // Use statements don't have names for ordering
            Item::Impl(i) => Some(format!("impl_{}", i.trait_name)),
            Item::TopLevelCode(_) => None, // Top-level code doesn't have names
        }
    }

    fn extract_dependencies(&self, item: &Item) -> Vec<String> {
        let mut deps = Vec::new();

        match item {
            Item::Function(f) => {
                // Dependencies from parameters
                for param in &f.params {
                    deps.extend(self.extract_type_dependencies(&param.param_type));
                }

                // Dependencies from return type
                if let Some(ref return_type) = f.return_type {
                    deps.extend(self.extract_type_dependencies(return_type));
                }

                // Dependencies from function body (called functions, used types)
                self.extract_expression_dependencies(&Expression::Block(f.body.clone()), &mut deps);
            }
            Item::Struct(s) => {
                // Dependencies from fields
                for field in &s.fields {
                    deps.extend(self.extract_type_dependencies(&field.field_type));
                }

                // Dependencies from generic parameters
                for _type_param in &s.type_params {
                    // Type parameters themselves don't create dependencies
                    // But constraints might
                }
            }
            Item::Enum(e) => {
                // Dependencies from variants
                for variant in &e.variants {
                    if let Some(ref variant_types) = variant.data {
                        for variant_type in variant_types {
                            deps.extend(self.extract_type_dependencies(variant_type));
                        }
                    }
                }
            }
            Item::TypeAlias(ta) => {
                // Dependencies from aliased type
                deps.extend(self.extract_type_dependencies(&ta.aliased_type));
            }
            Item::Trait(t) => {
                // Dependencies from method signatures
                for method in &t.methods {
                    for param in &method.params {
                        deps.extend(self.extract_type_dependencies(&param.param_type));
                    }
                    if let Some(ref return_type) = method.return_type {
                        deps.extend(self.extract_type_dependencies(return_type));
                    }
                }
            }
            Item::Impl(i) => {
                // Dependencies from trait name
                deps.push(i.trait_name.clone());

                // Dependencies from methods
                for method in &i.methods {
                    for param in &method.params {
                        deps.extend(self.extract_type_dependencies(&param.param_type));
                    }
                    if let Some(ref return_type) = method.return_type {
                        deps.extend(self.extract_type_dependencies(return_type));
                    }
                    self.extract_expression_dependencies(
                        &Expression::Block(method.body.clone()),
                        &mut deps,
                    );
                }
            }
            Item::Module(m) => {
                // Dependencies from module items (recursive)
                for sub_item in &m.items {
                    deps.extend(self.extract_dependencies(sub_item));
                }
            }
            Item::Use(u) => {
                // Use statements create dependencies on modules/types
                // path is Vec<String>, so we add all path components
                for path_component in &u.path {
                    deps.push(path_component.clone());
                }
            }
            Item::TopLevelCode(expr_stmt) => {
                // Dependencies from top-level expressions
                self.extract_expression_dependencies(&expr_stmt.expression, &mut deps);
            }
        }

        deps
    }

    fn extract_type_dependencies(&self, ty: &Type) -> Vec<String> {
        let mut deps = Vec::new();

        match ty {
            Type::Named(name) => {
                deps.push(name.clone());
            }
            Type::Generic { name, params } => {
                deps.push(name.clone());
                for param in params {
                    deps.extend(self.extract_type_dependencies(param));
                }
            }
            Type::List(item_type) => {
                deps.extend(self.extract_type_dependencies(item_type));
            }
            Type::Map { key, value } => {
                deps.extend(self.extract_type_dependencies(key));
                deps.extend(self.extract_type_dependencies(value));
            }
            Type::Optional(inner) => {
                deps.extend(self.extract_type_dependencies(inner));
            }
            Type::Result { ok, err } => {
                deps.extend(self.extract_type_dependencies(ok));
                deps.extend(self.extract_type_dependencies(err));
            }
            Type::Tuple(types) => {
                for t in types {
                    deps.extend(self.extract_type_dependencies(t));
                }
            }
            Type::Function {
                params,
                return_type,
            } => {
                for param in params {
                    deps.extend(self.extract_type_dependencies(param));
                }
                deps.extend(self.extract_type_dependencies(return_type));
            }
            _ => {} // Basic types don't create dependencies
        }

        deps
    }

    fn extract_expression_dependencies(&self, expr: &Expression, deps: &mut Vec<String>) {
        match expr {
            Expression::Identifier(name) => {
                // Could be a function call or type reference
                deps.push(name.clone());
            }
            Expression::Call { callee, args } => {
                // Extract function name from callee
                if let Expression::Identifier(name) = callee.as_ref() {
                    deps.push(name.clone());
                } else if let Expression::Member { object, member: _ } = callee.as_ref() {
                    // Method call - extract object type
                    self.extract_expression_dependencies(object, deps);
                }

                // Extract dependencies from arguments
                for arg in args {
                    self.extract_expression_dependencies(arg, deps);
                }
            }
            Expression::Member { object, member: _ } => {
                self.extract_expression_dependencies(object, deps);
            }
            Expression::Index { object, index } => {
                self.extract_expression_dependencies(object, deps);
                self.extract_expression_dependencies(index, deps);
            }
            Expression::BinaryOp { left, right, .. } => {
                self.extract_expression_dependencies(left, deps);
                self.extract_expression_dependencies(right, deps);
            }
            Expression::UnaryOp { expr, .. } => {
                self.extract_expression_dependencies(expr, deps);
            }
            Expression::If {
                condition,
                then_expr,
                else_expr,
            } => {
                self.extract_expression_dependencies(condition, deps);
                self.extract_expression_dependencies(then_expr, deps);
                self.extract_expression_dependencies(else_expr, deps);
            }
            Expression::Block(block) => {
                for stmt in &block.statements {
                    self.extract_statement_dependencies(stmt, deps);
                }
            }
            Expression::Lambda {
                params,
                return_type,
                body,
            } => {
                for param in params {
                    deps.extend(self.extract_type_dependencies(&param.param_type));
                }
                if let Some(ref ret_type) = return_type {
                    deps.extend(self.extract_type_dependencies(ret_type));
                }
                self.extract_expression_dependencies(body, deps);
            }
            Expression::StructLiteral { name, fields } => {
                deps.push(name.clone());
                for (_field_name, value) in fields {
                    self.extract_expression_dependencies(value, deps);
                }
            }
            Expression::GenericConstructor {
                name,
                type_params,
                args,
            } => {
                deps.push(name.clone());
                for type_param in type_params {
                    deps.extend(self.extract_type_dependencies(type_param));
                }
                for arg in args {
                    self.extract_expression_dependencies(arg, deps);
                }
            }
            Expression::ListLiteral(elements) => {
                for elem in elements {
                    self.extract_expression_dependencies(elem, deps);
                }
            }
            Expression::MapLiteral(entries) => {
                for (_key, value) in entries {
                    self.extract_expression_dependencies(value, deps);
                }
            }
            Expression::Assignment { target, value } => {
                self.extract_expression_dependencies(target, deps);
                self.extract_expression_dependencies(value, deps);
            }
            Expression::Await { expr } => {
                self.extract_expression_dependencies(expr, deps);
            }
            Expression::LLMCall { args, .. } => {
                for arg in args {
                    self.extract_expression_dependencies(arg, deps);
                }
            }
            Expression::FormatString { parts } => {
                for part in parts {
                    if let FormatStringPart::Expression(expr) = part {
                        self.extract_expression_dependencies(expr, deps);
                    }
                }
            }
            _ => {} // Literals don't create dependencies
        }
    }

    fn extract_statement_dependencies(&self, stmt: &Statement, deps: &mut Vec<String>) {
        match stmt {
            Statement::Let(let_stmt) => {
                if let Some(ref var_type) = let_stmt.var_type {
                    deps.extend(self.extract_type_dependencies(var_type));
                }
                self.extract_expression_dependencies(&let_stmt.value, deps);
            }
            Statement::Return(ret_stmt) => {
                if let Some(ref value) = ret_stmt.value {
                    self.extract_expression_dependencies(value, deps);
                }
            }
            Statement::Expression(expr_stmt) => {
                self.extract_expression_dependencies(&expr_stmt.expression, deps);
            }
            Statement::If(if_stmt) => {
                self.extract_expression_dependencies(&if_stmt.condition, deps);
                self.extract_statement_dependencies(
                    &Statement::Expression(ExpressionStatement {
                        expression: Expression::Block(if_stmt.then_block.clone()),
                    }),
                    deps,
                );
                if let Some(ref else_block) = if_stmt.else_block {
                    self.extract_statement_dependencies(
                        &Statement::Expression(ExpressionStatement {
                            expression: Expression::Block(else_block.clone()),
                        }),
                        deps,
                    );
                }
            }
            Statement::For(for_stmt) => {
                self.extract_expression_dependencies(&for_stmt.iterable, deps);
                self.extract_statement_dependencies(
                    &Statement::Expression(ExpressionStatement {
                        expression: Expression::Block(for_stmt.body.clone()),
                    }),
                    deps,
                );
            }
            Statement::While(while_stmt) => {
                self.extract_expression_dependencies(&while_stmt.condition, deps);
                self.extract_statement_dependencies(
                    &Statement::Expression(ExpressionStatement {
                        expression: Expression::Block(while_stmt.body.clone()),
                    }),
                    deps,
                );
            }
            Statement::Match(match_stmt) => {
                self.extract_expression_dependencies(&match_stmt.expression, deps);
                for arm in &match_stmt.arms {
                    self.extract_statement_dependencies(
                        &Statement::Expression(ExpressionStatement {
                            expression: Expression::Block(arm.body.clone()),
                        }),
                        deps,
                    );
                }
            }
            Statement::Throw(throw_stmt) => {
                self.extract_expression_dependencies(&throw_stmt.expression, deps);
            }
            Statement::Try(try_stmt) => {
                self.extract_statement_dependencies(
                    &Statement::Expression(ExpressionStatement {
                        expression: Expression::Block(try_stmt.try_block.clone()),
                    }),
                    deps,
                );
                for catch_block in &try_stmt.catch_blocks {
                    self.extract_statement_dependencies(
                        &Statement::Expression(ExpressionStatement {
                            expression: Expression::Block(catch_block.body.clone()),
                        }),
                        deps,
                    );
                }
                if let Some(ref finally_block) = try_stmt.finally_block {
                    self.extract_statement_dependencies(
                        &Statement::Expression(ExpressionStatement {
                            expression: Expression::Block(finally_block.clone()),
                        }),
                        deps,
                    );
                }
            }
            Statement::Break(_) => {}
        }
    }
}

impl Pass for CodeOrderingPass {
    fn name(&self) -> &str {
        "CodeOrdering"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        if let Some(ref mut program) = context.program {
            self.order_program(program)?;
        }
        Ok(())
    }
}
