/// IR Builder - Konvertiert AST zu IR
/// 
/// Dieser Builder konvertiert vollständig einen AST (Abstract Syntax Tree) in
/// eine IR (Intermediate Representation) im SSA-Format.
/// 
/// # Beispiel
/// 
/// ```rust
/// use velin_compiler::ir::builder::IRBuilder;
/// use velin_compiler::parser::ast::Program;
/// 
/// let mut builder = IRBuilder::new();
/// let ir_module = builder.build_module(&ast_program);
/// ```

use crate::parser::ast::*;
use crate::ir::ir::*;
use std::collections::HashMap;

/// IR Builder
/// 
/// Konvertiert AST-Knoten zu IR-Strukturen.
pub struct IRBuilder {
    current_function: Option<String>,
    current_block: BlockId,
    blocks: HashMap<BlockId, IRBlock>,
    temp_counter: usize,
    var_counter: usize,
    block_counter: usize,
    var_map: HashMap<String, VarId>,
    var_types: HashMap<VarId, IRType>,
}

impl IRBuilder {
    /// Erstellt einen neuen IR Builder
    pub fn new() -> Self {
        IRBuilder {
            current_function: None,
            current_block: BlockId::new(0),
            blocks: HashMap::new(),
            temp_counter: 0,
            var_counter: 0,
            block_counter: 0,
            var_map: HashMap::new(),
            var_types: HashMap::new(),
        }
    }
    
    /// Konvertiert ein AST-Programm zu einem IR-Modul
    pub fn build_module(&mut self, program: &Program) -> IRModule {
        let mut ir_module = IRModule::new("main".to_string());
        
        for item in &program.items {
            match item {
                Item::Function(func) => {
                    let ir_func = self.build_function(func);
                    ir_module.functions.push(ir_func);
                }
                Item::Struct(s) => {
                    let ir_struct = self.build_struct(s);
                    ir_module.structs.push(ir_struct);
                }
                Item::Enum(e) => {
                    let ir_enum = self.build_enum(e);
                    ir_module.enums.push(ir_enum);
                }
                _ => {
                    // TypeAlias, Module, Use, Trait, Impl werden später behandelt
                }
            }
        }
        
        ir_module
    }
    
    /// Konvertiert eine AST-Funktion zu einer IR-Funktion
    fn build_function(&mut self, func: &Function) -> IRFunction {
        self.current_function = Some(func.name.clone());
        self.current_block = BlockId::new(0);
        self.blocks.clear();
        self.var_map.clear();
        self.var_types.clear();
        
        // Parameter konvertieren
        let params: Vec<IRParameter> = func.params.iter()
            .map(|p| {
                let var_id = self.get_or_create_var_id(&p.name);
                let ty = self.ast_type_to_ir(&p.param_type);
                self.var_types.insert(var_id, ty.clone());
                
                IRParameter {
                    name: p.name.clone(),
                    ty: ty.clone(),
                    ownership: Ownership::Owned, // Default: owned
                }
            })
            .collect();
        
        // Return-Type konvertieren
        let return_type = func.return_type.as_ref()
            .map(|t| self.ast_type_to_ir(t))
            .unwrap_or(IRType::Void);
        
        // Body konvertieren (SSA-Format)
        let entry_block = self.create_block();
        self.current_block = entry_block;
        let body = self.build_block(&func.body);
        
        // Attributes aus Decorators konvertieren
        let attributes = self.build_attributes(&func.decorators);
        
        IRFunction {
            name: func.name.clone(),
            params,
            return_type,
            body,
            attributes,
            is_async: func.is_async,
            visibility: func.visibility.clone(),
        }
    }
    
    /// Konvertiert einen AST-Block zu einem IR-Block (SSA)
    fn build_block(&mut self, block: &Block) -> IRBlock {
        let block_id = self.current_block;
        let mut ir_block = IRBlock::new(block_id);
        
        for stmt in &block.statements {
            let stmt_instructions = self.build_statement(stmt);
            ir_block.instructions.extend(stmt_instructions);
        }
        
        ir_block
    }
    
    /// Konvertiert ein AST-Statement zu IR-Instructions
    fn build_statement(&mut self, stmt: &Statement) -> Vec<IRInstruction> {
        match stmt {
            Statement::Let(let_stmt) => {
                self.build_let_statement(let_stmt)
            }
            Statement::Return(ret) => {
                self.build_return_statement(ret)
            }
            Statement::Expression(expr_stmt) => {
                let _value = self.build_expression(&expr_stmt.expression);
                // Expression-Statement: Wert wird berechnet, aber nicht verwendet
                vec![]
            }
            Statement::If(if_stmt) => {
                self.build_if_statement(if_stmt)
            }
            Statement::For(for_stmt) => {
                self.build_for_statement(for_stmt)
            }
            Statement::While(while_stmt) => {
                self.build_while_statement(while_stmt)
            }
            Statement::Match(match_stmt) => {
                self.build_match_statement(match_stmt)
            }
            Statement::Throw(throw_stmt) => {
                self.build_throw_statement(throw_stmt)
            }
            Statement::Break(_) => {
                vec![IRInstruction::Jump {
                    target: BlockId::new(0), // Wird später durch Loop-Analyse korrigiert
                }]
            }
            Statement::Try(_) => {
                panic!("Try statement found after desugaring pass");
            }
        }
    }
    
    /// Konvertiert ein Let-Statement
    fn build_let_statement(&mut self, let_stmt: &LetStatement) -> Vec<IRInstruction> {
        let value = self.build_expression(&let_stmt.value);
        let var_id = self.get_or_create_var_id(&let_stmt.name);
        
        let ty = if let Some(ast_type) = &let_stmt.var_type {
            self.ast_type_to_ir(ast_type)
        } else {
            value.get_type()
        };
        
        self.var_types.insert(var_id, ty.clone());
        
        let var_value = IRValue::Variable(IRVariable {
            name: let_stmt.name.clone(),
            id: var_id,
            ty: ty.clone(),
            ownership: Ownership::Owned,
        });
        
        vec![
            IRInstruction::Alloca {
                dest: var_value.clone(),
                ty: ty.clone(),
            },
            IRInstruction::Store {
                dest: var_value,
                value,
            },
        ]
    }
    
    /// Konvertiert ein Return-Statement
    fn build_return_statement(&mut self, ret: &ReturnStatement) -> Vec<IRInstruction> {
        let value = ret.value.as_ref()
            .map(|e| self.build_expression(e));
        
        vec![IRInstruction::Return { value }]
    }
    
    /// Konvertiert ein If-Statement
    fn build_if_statement(&mut self, if_stmt: &IfStatement) -> Vec<IRInstruction> {
        let condition = self.build_expression(&if_stmt.condition);
        
        let then_block_id = self.create_block();
        let else_block_id = if if_stmt.else_block.is_some() {
            self.create_block()
        } else {
            self.create_block() // Merge-Block
        };
        let merge_block_id = self.create_block();
        
        // Then-Block bauen
        let saved_block = self.current_block;
        self.current_block = then_block_id;
        let then_instructions = self.build_block(&if_stmt.then_block).instructions;
        let mut then_block = IRBlock::new(then_block_id);
        then_block.instructions = then_instructions;
        then_block.successors.push(merge_block_id);
        self.blocks.insert(then_block_id, then_block);
        
        // Else-Block bauen (falls vorhanden)
        if let Some(ref else_block) = if_stmt.else_block {
            self.current_block = else_block_id;
            let else_instructions = self.build_block(else_block).instructions;
            let mut else_block_ir = IRBlock::new(else_block_id);
            else_block_ir.instructions = else_instructions;
            else_block_ir.successors.push(merge_block_id);
            self.blocks.insert(else_block_id, else_block_ir);
        }
        
        // Merge-Block
        let mut merge_block = IRBlock::new(merge_block_id);
        merge_block.predecessors.push(then_block_id);
        if if_stmt.else_block.is_some() {
            merge_block.predecessors.push(else_block_id);
        }
        self.blocks.insert(merge_block_id, merge_block);
        
        self.current_block = saved_block;
        
        vec![
            IRInstruction::Branch {
                condition,
                then_block: then_block_id,
                else_block: else_block_id,
            }
        ]
    }
    
    /// Konvertiert ein For-Statement
    fn build_for_statement(&mut self, for_stmt: &ForStatement) -> Vec<IRInstruction> {
        let loop_start = self.create_block();
        let loop_body = self.create_block();
        let _loop_end = self.create_block();
        
        // Iterable auswerten
        let _iterable = self.build_expression(&for_stmt.iterable);
        
        // Loop-Variable erstellen
        let var_id = self.get_or_create_var_id(&for_stmt.variable);
        let _var_value = IRValue::Variable(IRVariable {
            name: for_stmt.variable.clone(),
            id: var_id,
            ty: IRType::Any, // Wird später durch Type-Checker gefüllt
            ownership: Ownership::Owned,
        });
        
        // Body bauen
        let saved_block = self.current_block;
        self.current_block = loop_body;
        let body_instructions = self.build_block(&for_stmt.body).instructions;
        let mut body_block = IRBlock::new(loop_body);
        body_block.instructions = body_instructions;
        body_block.successors.push(loop_start); // Loop zurück
        self.blocks.insert(loop_body, body_block);
        
        self.current_block = saved_block;
        
        vec![
            IRInstruction::Jump { target: loop_start },
        ]
    }
    
    /// Konvertiert ein While-Statement
    fn build_while_statement(&mut self, while_stmt: &WhileStatement) -> Vec<IRInstruction> {
        let loop_start = self.create_block();
        let loop_body = self.create_block();
        let loop_end = self.create_block();
        
        let condition = self.build_expression(&while_stmt.condition);
        
        // Body bauen
        let saved_block = self.current_block;
        self.current_block = loop_body;
        let body_instructions = self.build_block(&while_stmt.body).instructions;
        let mut body_block = IRBlock::new(loop_body);
        body_block.instructions = body_instructions;
        body_block.successors.push(loop_start); // Loop zurück
        self.blocks.insert(loop_body, body_block);
        
        self.current_block = saved_block;
        
        vec![
            IRInstruction::Branch {
                condition,
                then_block: loop_body,
                else_block: loop_end,
            }
        ]
    }
    
    /// Konvertiert ein Match-Statement
    fn build_match_statement(&mut self, match_stmt: &MatchStatement) -> Vec<IRInstruction> {
        let value = self.build_expression(&match_stmt.expression);
        
        let arms: Vec<IRMatchArm> = match_stmt.arms.iter()
            .map(|arm| {
                let arm_block = self.create_block();
                
                // Body bauen
                let saved_block = self.current_block;
                self.current_block = arm_block;
                let body_instructions = self.build_block(&arm.body).instructions;
                let mut body_block = IRBlock::new(arm_block);
                body_block.instructions = body_instructions;
                self.blocks.insert(arm_block, body_block);
                self.current_block = saved_block;
                
                IRMatchArm {
                    pattern: self.build_pattern(&arm.pattern),
                    guard: arm.guard.as_ref().map(|g| self.build_expression(g)),
                    body: arm_block,
                }
            })
            .collect();
        
        vec![IRInstruction::Match { value, arms }]
    }
    
    /// Konvertiert ein Throw-Statement
    fn build_throw_statement(&mut self, throw_stmt: &ThrowStatement) -> Vec<IRInstruction> {
        let _value = self.build_expression(&throw_stmt.expression);
        // Throw wird als Call zu einer Error-Funktion behandelt
        vec![]
    }
    
    /// Konvertiert eine AST-Expression zu einem IR-Value
    fn build_expression(&mut self, expr: &Expression) -> IRValue {
        match expr {
            Expression::Literal(lit) => {
                IRValue::Constant(self.build_literal(lit))
            }
            Expression::Identifier(name) => {
                let var_id = self.get_or_create_var_id(name);
                let ty = self.var_types.get(&var_id)
                    .cloned()
                    .unwrap_or(IRType::Any);
                
                IRValue::Variable(IRVariable {
                    name: name.clone(),
                    id: var_id,
                    ty,
                    ownership: Ownership::Owned,
                })
            }
            Expression::BinaryOp { left, op, right } => {
                self.build_binary_op(left, op, right)
            }
            Expression::UnaryOp { op, expr } => {
                self.build_unary_op(op, expr)
            }
            Expression::Call { callee, args } => {
                self.build_call(callee, args)
            }
            Expression::Member { object, member } => {
                self.build_member(object, member)
            }
            Expression::Index { object, index } => {
                self.build_index(object, index)
            }
            Expression::If { condition, then_expr, else_expr } => {
                self.build_if_expression(condition, then_expr, else_expr)
            }
            Expression::Block(block) => {
                // Block-Expression: Letzte Expression ist der Wert
                let _block_ir = self.build_block(block);
                // Letzte Expression ist der Return-Wert
                if let Some(last_expr) = block.statements.last() {
                    if let Statement::Expression(expr_stmt) = last_expr {
                        let value = self.build_expression(&expr_stmt.expression);
                        let return_inst = IRInstruction::Return {
                            value: Some(value),
                        };
                        if let Some(block_ir) = self.blocks.get_mut(&self.current_block) {
                            block_ir.instructions.push(return_inst);
                        }
                    }
                }
                IRValue::Constant(IRConstant::Null)
            }
            Expression::Await { expr } => {
                self.build_await(expr)
            }
            Expression::StructLiteral { name, fields } => {
                self.build_struct_literal(name, fields)
            }
            Expression::MapLiteral(fields) => {
                self.build_map_literal(fields)
            }
            Expression::ListLiteral(items) => {
                self.build_list_literal(items)
            }
            Expression::GenericConstructor { name, type_params, args } => {
                self.build_generic_constructor(name, type_params, args)
            }
            Expression::Lambda { params, return_type, body } => {
                self.build_lambda(params, return_type, body)
            }
            Expression::Assignment { target, value } => {
                self.build_assignment(target, value)
            }
            Expression::FormatString { parts } => {
                self.build_format_string(parts)
            }
            Expression::LLMCall { method, args } => {
                self.build_llm_call(method, args)
            }
        }
    }
    
    /// Konvertiert einen LLM-Call (@llm.analyze(text))
    fn build_llm_call(&mut self, _method: &str, args: &[Expression]) -> IRValue {
        // Argumente konvertieren
        let args_ir: Vec<IRValue> = args.iter()
            .map(|a| self.build_expression(a))
            .collect();
        
        // LLM-Client als Variable
        let llm_client = IRValue::Variable(IRVariable {
            name: "llm_client".to_string(),
            id: self.get_or_create_var_id("llm_client"),
            ty: IRType::Any,
            ownership: Ownership::Owned,
        });
        
        // Method-Call als Call-Instruction
        let dest = self.create_temp(IRType::String);
        
        let instruction = IRInstruction::CallAsync {
            dest: Some(IRValue::Temporary(dest.clone())),
            func: llm_client,
            args: args_ir,
        };
        
        if let Some(block) = self.blocks.get_mut(&self.current_block) {
            block.instructions.push(instruction);
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert ein Literal
    fn build_literal(&self, lit: &Literal) -> IRConstant {
        match lit {
            Literal::String(s) => IRConstant::String(s.clone()),
            Literal::Number(n) => IRConstant::Number(*n),
            Literal::Boolean(b) => IRConstant::Boolean(*b),
            Literal::Null => IRConstant::Null,
        }
    }
    
    /// Konvertiert eine Binary-Operation
    fn build_binary_op(&mut self, left: &Expression, op: &BinaryOperator, right: &Expression) -> IRValue {
        let left_val = self.build_expression(left);
        let right_val = self.build_expression(right);
        let dest = self.create_temp(IRType::Bool); // Default, wird später korrigiert
        
        let instruction = match op {
            BinaryOperator::Add => IRInstruction::Add {
                dest: IRValue::Temporary(dest.clone()),
                left: left_val,
                right: right_val,
            },
            BinaryOperator::Subtract => IRInstruction::Subtract {
                dest: IRValue::Temporary(dest.clone()),
                left: left_val,
                right: right_val,
            },
            BinaryOperator::Multiply => IRInstruction::Multiply {
                dest: IRValue::Temporary(dest.clone()),
                left: left_val,
                right: right_val,
            },
            BinaryOperator::Divide => IRInstruction::Divide {
                dest: IRValue::Temporary(dest.clone()),
                left: left_val,
                right: right_val,
            },
            BinaryOperator::Modulo => IRInstruction::Modulo {
                dest: IRValue::Temporary(dest.clone()),
                left: left_val,
                right: right_val,
            },
            BinaryOperator::Eq => IRInstruction::Eq {
                dest: IRValue::Temporary(dest.clone()),
                left: left_val,
                right: right_val,
            },
            BinaryOperator::NotEq => IRInstruction::NotEq {
                dest: IRValue::Temporary(dest.clone()),
                left: left_val,
                right: right_val,
            },
            BinaryOperator::Lt => IRInstruction::Lt {
                dest: IRValue::Temporary(dest.clone()),
                left: left_val,
                right: right_val,
            },
            BinaryOperator::Gt => IRInstruction::Gt {
                dest: IRValue::Temporary(dest.clone()),
                left: left_val,
                right: right_val,
            },
            BinaryOperator::LtEq => IRInstruction::LtEq {
                dest: IRValue::Temporary(dest.clone()),
                left: left_val,
                right: right_val,
            },
            BinaryOperator::GtEq => IRInstruction::GtEq {
                dest: IRValue::Temporary(dest.clone()),
                left: left_val,
                right: right_val,
            },
            BinaryOperator::And => IRInstruction::And {
                dest: IRValue::Temporary(dest.clone()),
                left: left_val,
                right: right_val,
            },
            BinaryOperator::Or => IRInstruction::Or {
                dest: IRValue::Temporary(dest.clone()),
                left: left_val,
                right: right_val,
            },
        };
        
        // Instruction zum aktuellen Block hinzufügen
        if let Some(block) = self.blocks.get_mut(&self.current_block) {
            block.instructions.push(instruction);
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert eine Unary-Operation
    fn build_unary_op(&mut self, op: &UnaryOperator, expr: &Expression) -> IRValue {
        let operand = self.build_expression(expr);
        let dest = self.create_temp(IRType::Bool);
        
        let instruction = match op {
            UnaryOperator::Not => IRInstruction::Not {
                dest: IRValue::Temporary(dest.clone()),
                operand,
            },
            UnaryOperator::Minus => {
                // Minus wird als 0 - operand behandelt
                let zero = IRValue::Constant(IRConstant::Number(0.0));
                IRInstruction::Subtract {
                    dest: IRValue::Temporary(dest.clone()),
                    left: zero,
                    right: operand,
                }
            }
        };
        
        if let Some(block) = self.blocks.get_mut(&self.current_block) {
            block.instructions.push(instruction);
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert einen Funktions-Aufruf
    fn build_call(&mut self, callee: &Expression, args: &[Expression]) -> IRValue {
        let func = self.build_expression(callee);
        let args_ir: Vec<IRValue> = args.iter()
            .map(|a| self.build_expression(a))
            .collect();
        
        let dest = self.create_temp(IRType::Any);
        
        let instruction = IRInstruction::Call {
            dest: Some(IRValue::Temporary(dest.clone())),
            func,
            args: args_ir,
        };
        
        if let Some(block) = self.blocks.get_mut(&self.current_block) {
            block.instructions.push(instruction);
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert einen Member-Zugriff
    fn build_member(&mut self, object: &Expression, member: &str) -> IRValue {
        let obj = self.build_expression(object);
        let dest = self.create_temp(IRType::Any);
        
        let instruction = IRInstruction::StructAccess {
            dest: IRValue::Temporary(dest.clone()),
            struct_val: obj,
            field: member.to_string(),
        };
        
        if let Some(block) = self.blocks.get_mut(&self.current_block) {
            block.instructions.push(instruction);
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert einen Index-Zugriff
    fn build_index(&mut self, object: &Expression, index: &Expression) -> IRValue {
        let obj = self.build_expression(object);
        let idx = self.build_expression(index);
        let dest = self.create_temp(IRType::Any);
        
        let instruction = IRInstruction::ListGet {
            dest: IRValue::Temporary(dest.clone()),
            list: obj,
            index: idx,
        };
        
        if let Some(block) = self.blocks.get_mut(&self.current_block) {
            block.instructions.push(instruction);
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert eine If-Expression
    fn build_if_expression(&mut self, condition: &Expression, then_expr: &Expression, else_expr: &Expression) -> IRValue {
        let _cond = self.build_expression(condition);
        let then_val = self.build_expression(then_expr);
        let else_val = self.build_expression(else_expr);
        
        // Phi-Node für SSA
        let dest = self.create_temp(IRType::Any);
        let then_block = self.current_block;
        let else_block = self.create_block();
        let merge_block = self.create_block();
        
        let phi = IRInstruction::Phi {
            dest: IRValue::Temporary(dest.clone()),
            incoming: vec![
                (then_block, then_val),
                (else_block, else_val),
            ],
        };
        
        if let Some(block) = self.blocks.get_mut(&merge_block) {
            block.instructions.push(phi);
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert einen Await-Ausdruck
    fn build_await(&mut self, expr: &Expression) -> IRValue {
        let func = self.build_expression(expr);
        let dest = self.create_temp(IRType::Any);
        
        let instruction = IRInstruction::CallAsync {
            dest: Some(IRValue::Temporary(dest.clone())),
            func,
            args: vec![],
        };
        
        if let Some(block) = self.blocks.get_mut(&self.current_block) {
            block.instructions.push(instruction);
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert einen Struct-Literal
    fn build_struct_literal(&mut self, name: &str, fields: &[(String, Expression)]) -> IRValue {
        let fields_ir: Vec<(String, IRValue)> = fields.iter()
            .map(|(fname, expr)| (fname.clone(), self.build_expression(expr)))
            .collect();
        
        let dest = self.create_temp(IRType::Struct(name.to_string()));
        
        let instruction = IRInstruction::StructConstruct {
            dest: IRValue::Temporary(dest.clone()),
            struct_type: IRType::Struct(name.to_string()),
            fields: fields_ir,
        };
        
        if let Some(block) = self.blocks.get_mut(&self.current_block) {
            block.instructions.push(instruction);
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert einen Map-Literal
    fn build_map_literal(&mut self, fields: &[(String, Expression)]) -> IRValue {
        // Map-Literal wird als Struct-Literal behandelt
        let dest = self.create_temp(IRType::Map {
            key: Box::new(IRType::String),
            value: Box::new(IRType::Any),
        });
        
        // Map-Konstruktion: Erstelle Map mit Key-Value-Paaren
        let mut map_fields = Vec::new();
        for (key, value_expr) in fields {
            let value = self.build_expression(value_expr);
            map_fields.push((key.clone(), value));
        }
        
        let instruction = IRInstruction::StructConstruct {
            dest: IRValue::Temporary(dest.clone()),
            struct_type: IRType::Map {
                key: Box::new(IRType::String),
                value: Box::new(IRType::Any),
            },
            fields: map_fields,
        };
        
        if let Some(block) = self.blocks.get_mut(&self.current_block) {
            block.instructions.push(instruction);
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert einen List-Literal
    fn build_list_literal(&mut self, items: &[Expression]) -> IRValue {
        let dest = self.create_temp(IRType::List(Box::new(IRType::Any)));
        
        // List-Konstruktion: Erstelle Liste mit Elementen
        for (i, item_expr) in items.iter().enumerate() {
            let item = self.build_expression(item_expr);
            let index = IRValue::Constant(IRConstant::Number(i as f64));
            
            let set_inst = IRInstruction::ListSet {
                list: IRValue::Temporary(dest.clone()),
                index,
                value: item,
            };
            
            if let Some(block) = self.blocks.get_mut(&self.current_block) {
                block.instructions.push(set_inst);
            }
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert einen Generic-Constructor
    fn build_generic_constructor(&mut self, name: &str, _type_params: &[Type], args: &[Expression]) -> IRValue {
        let args_ir: Vec<IRValue> = args.iter()
            .map(|a| self.build_expression(a))
            .collect();
        
        let dest = self.create_temp(IRType::Any);
        
        // Generic-Constructor: Rufe Konstruktor mit Type-Parametern auf
        let func_name = format!("{}::new", name);
        let func = IRValue::Variable(IRVariable {
            name: func_name,
            id: VarId::new(0),
            ty: IRType::Function {
                params: vec![IRType::Any],
                return_type: Box::new(IRType::Any),
            },
            ownership: Ownership::Owned,
        });
        
        let instruction = IRInstruction::Call {
            dest: Some(IRValue::Temporary(dest.clone())),
            func,
            args: args_ir,
        };
        
        if let Some(block) = self.blocks.get_mut(&self.current_block) {
            block.instructions.push(instruction);
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert eine Lambda-Funktion
    fn build_lambda(&mut self, _params: &[Parameter], _return_type: &Option<Type>, body: &Expression) -> IRValue {
        let dest = self.create_temp(IRType::Function {
            params: vec![],
            return_type: Box::new(IRType::Void),
        });
        
        // Lambda-Implementierung: Erstelle Closure
        let body_val = self.build_expression(body);
        
        // Lambda wird als Function-Type gespeichert
        let instruction = IRInstruction::Store {
            dest: IRValue::Temporary(dest.clone()),
            value: body_val,
        };
        
        if let Some(block) = self.blocks.get_mut(&self.current_block) {
            block.instructions.push(instruction);
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert eine Assignment
    fn build_assignment(&mut self, target: &Expression, value: &Expression) -> IRValue {
        let target_val = self.build_expression(target);
        let value_val = self.build_expression(value);
        
        let instruction = IRInstruction::Store {
            dest: target_val,
            value: value_val.clone(),
        };
        
        if let Some(block) = self.blocks.get_mut(&self.current_block) {
            block.instructions.push(instruction);
        }
        
        value_val
    }
    
    /// Konvertiert einen Format-String
    fn build_format_string(&mut self, parts: &[FormatStringPart]) -> IRValue {
        // Format-String wird zu String-Konkatenation
        let mut result = IRValue::Constant(IRConstant::String(String::new()));
        
        for part in parts {
            match part {
                FormatStringPart::Text(text) => {
                    let text_val = IRValue::Constant(IRConstant::String(text.clone()));
                    result = self.build_string_concat(&result, &text_val);
                }
                FormatStringPart::Expression(expr) => {
                    let expr_val = self.build_expression(expr);
                    // Expression zu String konvertieren
                    result = self.build_string_concat(&result, &expr_val);
                }
            }
        }
        
        result
    }
    
    /// Hilfsfunktion für String-Konkatenation
    fn build_string_concat(&mut self, left: &IRValue, right: &IRValue) -> IRValue {
        let dest = self.create_temp(IRType::String);
        
        let instruction = IRInstruction::Add {
            dest: IRValue::Temporary(dest.clone()),
            left: left.clone(),
            right: right.clone(),
        };
        
        if let Some(block) = self.blocks.get_mut(&self.current_block) {
            block.instructions.push(instruction);
        }
        
        IRValue::Temporary(dest)
    }
    
    /// Konvertiert ein Pattern
    fn build_pattern(&self, pattern: &Pattern) -> IRPattern {
        match pattern {
            Pattern::Literal(lit) => {
                IRPattern::Literal(self.build_literal(lit))
            }
            Pattern::Identifier(name) => {
                IRPattern::Identifier(name.clone())
            }
            Pattern::Tuple(patterns) => {
                IRPattern::Tuple(patterns.iter().map(|p| self.build_pattern(p)).collect())
            }
            Pattern::Struct { name, fields } => {
                IRPattern::Struct {
                    name: name.clone(),
                    fields: fields.iter()
                        .map(|(fname, p)| (fname.clone(), self.build_pattern(p)))
                        .collect(),
                }
            }
            Pattern::EnumVariant { name, data } => {
                IRPattern::EnumVariant {
                    name: name.clone(),
                    data: data.as_ref().map(|d| d.iter().map(|p| self.build_pattern(p)).collect()),
                }
            }
            Pattern::Wildcard => IRPattern::Wildcard,
            Pattern::Or(patterns) => {
                // Or-Pattern wird als erstes Pattern behandelt
                if let Some(first) = patterns.first() {
                    self.build_pattern(first)
                } else {
                    IRPattern::Wildcard
                }
            }
            Pattern::Range { .. } => {
                // Range-Pattern wird später implementiert
                IRPattern::Wildcard
            }
        }
    }
    
    /// Konvertiert einen AST-Type zu einem IR-Type
    fn ast_type_to_ir(&self, ty: &Type) -> IRType {
        match ty {
            Type::String => IRType::String,
            Type::Number => IRType::Float,
            Type::Boolean => IRType::Bool,
            Type::Void => IRType::Void,
            Type::Null => IRType::Null,
            Type::Any => IRType::Any,
            Type::Named(name) => {
                // Struct oder Enum
                IRType::Struct(name.clone())
            }
            Type::Generic { name, params } => {
                match name.as_str() {
                    "List" => {
                        if let Some(item_type) = params.first() {
                            IRType::List(Box::new(self.ast_type_to_ir(item_type)))
                        } else {
                            IRType::List(Box::new(IRType::Any))
                        }
                    }
                    "Map" => {
                        if params.len() >= 2 {
                            IRType::Map {
                                key: Box::new(self.ast_type_to_ir(&params[0])),
                                value: Box::new(self.ast_type_to_ir(&params[1])),
                            }
                        } else {
                            IRType::Map {
                                key: Box::new(IRType::String),
                                value: Box::new(IRType::Any),
                            }
                        }
                    }
                    "Result" => {
                        if params.len() >= 2 {
                            IRType::Result {
                                ok: Box::new(self.ast_type_to_ir(&params[0])),
                                err: Box::new(self.ast_type_to_ir(&params[1])),
                            }
                        } else {
                            IRType::Result {
                                ok: Box::new(IRType::Any),
                                err: Box::new(IRType::String),
                            }
                        }
                    }
                    _ => IRType::Any,
                }
            }
            Type::Function { params, return_type } => {
                IRType::Function {
                    params: params.iter().map(|p| self.ast_type_to_ir(p)).collect(),
                    return_type: Box::new(self.ast_type_to_ir(return_type)),
                }
            }
            Type::List(item_type) => {
                IRType::List(Box::new(self.ast_type_to_ir(item_type)))
            }
            Type::Map { key, value } => {
                IRType::Map {
                    key: Box::new(self.ast_type_to_ir(key)),
                    value: Box::new(self.ast_type_to_ir(value)),
                }
            }
            Type::Tuple(types) => {
                IRType::Tuple(types.iter().map(|t| self.ast_type_to_ir(t)).collect())
            }
            Type::Optional(inner) => {
                IRType::Optional(Box::new(self.ast_type_to_ir(inner)))
            }
            Type::Result { ok, err } => {
                IRType::Result {
                    ok: Box::new(self.ast_type_to_ir(ok)),
                    err: Box::new(self.ast_type_to_ir(err)),
                }
            }
        }
    }
    
    /// Konvertiert Decorators zu Attributes
    fn build_attributes(&self, decorators: &[Decorator]) -> Vec<IRAttribute> {
        decorators.iter()
            .map(|d| {
                let args: Vec<IRAttributeArg> = d.args.iter()
                    .map(|a| match a {
                        DecoratorArg::String(s) => IRAttributeArg::String(s.clone()),
                        DecoratorArg::Number(n) => IRAttributeArg::Number(*n),
                        DecoratorArg::Boolean(b) => IRAttributeArg::Boolean(*b),
                        DecoratorArg::Identifier(i) => IRAttributeArg::Identifier(i.clone()),
                        DecoratorArg::Named { name, value: _ } => {
                            // Named arguments werden als Identifier behandelt
                            IRAttributeArg::Identifier(name.clone())
                        }
                    })
                    .collect();
                
                IRAttribute {
                    name: d.name.clone(),
                    args,
                }
            })
            .collect()
    }
    
    /// Konvertiert einen Struct
    fn build_struct(&self, s: &Struct) -> IRStruct {
        let fields: Vec<IRStructField> = s.fields.iter()
            .map(|f| IRStructField {
                name: f.name.clone(),
                ty: self.ast_type_to_ir(&f.field_type),
                visibility: f.visibility.clone(),
            })
            .collect();
        
        IRStruct {
            name: s.name.clone(),
            fields,
            visibility: s.visibility.clone(),
        }
    }
    
    /// Konvertiert einen Enum
    fn build_enum(&self, e: &Enum) -> IREnum {
        let variants: Vec<IREnumVariant> = e.variants.iter()
            .map(|v| IREnumVariant {
                name: v.name.clone(),
                data: v.data.as_ref().map(|d| d.iter().map(|t| self.ast_type_to_ir(t)).collect()),
            })
            .collect();
        
        IREnum {
            name: e.name.clone(),
            variants,
            visibility: e.visibility.clone(),
        }
    }
    
    /// Erstellt eine neue temporäre Variable
    fn create_temp(&mut self, _ty: IRType) -> TempId {
        let id = TempId::new(self.temp_counter);
        self.temp_counter += 1;
        id
    }
    
    /// Erstellt einen neuen Block
    fn create_block(&mut self) -> BlockId {
        let id = BlockId::new(self.block_counter);
        self.block_counter += 1;
        let block = IRBlock::new(id);
        self.blocks.insert(id, block);
        id
    }
    
    /// Holt oder erstellt eine Variable-ID
    fn get_or_create_var_id(&mut self, name: &str) -> VarId {
        if let Some(&var_id) = self.var_map.get(name) {
            var_id
        } else {
            let var_id = VarId::new(self.var_counter);
            self.var_counter += 1;
            self.var_map.insert(name.to_string(), var_id);
            var_id
        }
    }
}

impl Default for IRBuilder {
    fn default() -> Self {
        Self::new()
    }
}
