/// IR Code Generator - Generiert Code aus IR
/// 
/// Dieser Generator konvertiert IR (Intermediate Representation) zu Target-Code
/// (Rust, PHP, Python, etc.).
/// 
/// # Beispiel
/// 
/// ```rust
/// use velin_compiler::codegen::ir_codegen::IRCodeGenerator;
/// use velin_compiler::codegen::traits::TargetLanguage;
/// 
/// let generator = IRCodeGenerator::new(TargetLanguage::Rust);
/// let code = generator.generate(&ir_module);
/// ```

use crate::ir::ir::*;
use crate::codegen::traits::TargetLanguage;
use anyhow::Result;

/// IR Code Generator
pub struct IRCodeGenerator {
    target: TargetLanguage,
}

impl IRCodeGenerator {
    /// Erstellt einen neuen IR Code Generator
    pub fn new(target: TargetLanguage) -> Self {
        IRCodeGenerator { target }
    }
    
    /// Generiert Code aus IR
    pub fn generate(&self, module: &IRModule) -> Result<String> {
        match self.target {
            TargetLanguage::Rust => Ok(self.generate_rust(module)),
            TargetLanguage::Php => Ok(self.generate_php(module)),
            TargetLanguage::Python => Ok(self.generate_python(module)),
            TargetLanguage::JavaScript => Ok(self.generate_javascript(module)),
            TargetLanguage::TypeScript => Ok(self.generate_typescript(module)),
            TargetLanguage::Go => Ok(self.generate_go(module)),
            TargetLanguage::Java => Ok(self.generate_java(module)),
            TargetLanguage::CSharp => Ok(self.generate_csharp(module)),
        }
    }
    
    /// Generiert Rust-Code aus IR
    fn generate_rust(&self, module: &IRModule) -> String {
        let mut output = String::new();
        
        // Imports
        output.push_str("use std::collections::HashMap;\n");
        output.push_str("use serde::{Serialize, Deserialize};\n\n");
        
        // Structs generieren
        for s in &module.structs {
            output.push_str(&self.generate_rust_struct(s));
            output.push_str("\n");
        }
        
        // Enums generieren
        for e in &module.enums {
            output.push_str(&self.generate_rust_enum(e));
            output.push_str("\n");
        }
        
        // Functions generieren
        for f in &module.functions {
            output.push_str(&self.generate_rust_function(f));
            output.push_str("\n");
        }
        
        output
    }
    
    /// Generiert eine Rust-Struct
    fn generate_rust_struct(&self, s: &IRStruct) -> String {
        let mut output = String::new();
        
        let vis = match s.visibility {
            crate::parser::ast::Visibility::Public => "pub ",
            crate::parser::ast::Visibility::Private => "",
        };
        
        output.push_str(&format!("{}struct {} {{\n", vis, s.name));
        
        for field in &s.fields {
            let field_vis = match field.visibility {
                crate::parser::ast::Visibility::Public => "pub ",
                crate::parser::ast::Visibility::Private => "",
            };
            output.push_str(&format!("    {}{}: {},\n", 
                field_vis, 
                field.name, 
                self.ir_type_to_rust(&field.ty)));
        }
        
        output.push_str("}\n");
        output
    }
    
    /// Generiert einen Rust-Enum
    fn generate_rust_enum(&self, e: &IREnum) -> String {
        let mut output = String::new();
        
        let vis = match e.visibility {
            crate::parser::ast::Visibility::Public => "pub ",
            crate::parser::ast::Visibility::Private => "",
        };
        
        output.push_str(&format!("{}enum {} {{\n", vis, e.name));
        
        for variant in &e.variants {
            if let Some(data) = &variant.data {
                output.push_str(&format!("    {}({}),\n", 
                    variant.name,
                    data.iter()
                        .map(|t| self.ir_type_to_rust(t))
                        .collect::<Vec<_>>()
                        .join(", ")));
            } else {
                output.push_str(&format!("    {},\n", variant.name));
            }
        }
        
        output.push_str("}\n");
        output
    }
    
    /// Generiert eine Rust-Funktion
    fn generate_rust_function(&self, func: &IRFunction) -> String {
        let mut output = String::new();
        
        let vis = match func.visibility {
            crate::parser::ast::Visibility::Public => "pub ",
            crate::parser::ast::Visibility::Private => "",
        };
        
        let _async_keyword = if func.is_async { "async " } else { "" };
        
        // Function signature
        output.push_str(&format!("{}async fn {}(", vis, func.name));
        
        // Parameter
        let params: Vec<String> = func.params.iter()
            .map(|p| format!("{}: {}", p.name, self.ir_type_to_rust(&p.ty)))
            .collect();
        output.push_str(&params.join(", "));
        
        output.push_str(") -> ");
        output.push_str(&self.ir_type_to_rust(&func.return_type));
        output.push_str(" {\n");
        
        // Body generieren (aus IR-Instructions)
        output.push_str(&self.generate_rust_block(&func.body));
        
        output.push_str("}\n");
        output
    }
    
    /// Generiert einen Rust-Block
    fn generate_rust_block(&self, block: &IRBlock) -> String {
        let mut output = String::new();
        
        for instruction in &block.instructions {
            output.push_str(&self.generate_rust_instruction(instruction));
        }
        
        output
    }
    
    /// Generiert eine Rust-Instruction
    fn generate_rust_instruction(&self, inst: &IRInstruction) -> String {
        match inst {
            IRInstruction::Add { dest, left, right } => {
                format!("    let {} = {} + {};\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_value_to_rust(left),
                    self.ir_value_to_rust(right))
            }
            IRInstruction::Subtract { dest, left, right } => {
                format!("    let {} = {} - {};\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_value_to_rust(left),
                    self.ir_value_to_rust(right))
            }
            IRInstruction::Multiply { dest, left, right } => {
                format!("    let {} = {} * {};\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_value_to_rust(left),
                    self.ir_value_to_rust(right))
            }
            IRInstruction::Divide { dest, left, right } => {
                format!("    let {} = {} / {};\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_value_to_rust(left),
                    self.ir_value_to_rust(right))
            }
            IRInstruction::Eq { dest, left, right } => {
                format!("    let {} = {} == {};\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_value_to_rust(left),
                    self.ir_value_to_rust(right))
            }
            IRInstruction::NotEq { dest, left, right } => {
                format!("    let {} = {} != {};\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_value_to_rust(left),
                    self.ir_value_to_rust(right))
            }
            IRInstruction::And { dest, left, right } => {
                format!("    let {} = {} && {};\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_value_to_rust(left),
                    self.ir_value_to_rust(right))
            }
            IRInstruction::Or { dest, left, right } => {
                format!("    let {} = {} || {};\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_value_to_rust(left),
                    self.ir_value_to_rust(right))
            }
            IRInstruction::Not { dest, operand } => {
                format!("    let {} = !{};\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_value_to_rust(operand))
            }
            IRInstruction::Store { dest, value } => {
                format!("    {} = {};\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_value_to_rust(value))
            }
            IRInstruction::Alloca { dest, ty } => {
                format!("    let mut {}: {};\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_type_to_rust(ty))
            }
            IRInstruction::Branch { condition, then_block, else_block } => {
                format!("    if {} {{\n        // Block {:?}\n    }} else {{\n        // Block {:?}\n    }}\n", 
                    self.ir_value_to_rust(condition),
                    then_block,
                    else_block)
            }
            IRInstruction::Jump { target } => {
                format!("    // Jump to block {:?}\n", target)
            }
            IRInstruction::Return { value } => {
                if let Some(v) = value {
                    format!("    return {};\n", self.ir_value_to_rust(v))
                } else {
                    "    return;\n".to_string()
                }
            }
            IRInstruction::Call { dest, func, args } => {
                let args_str = args.iter()
                    .map(|a| self.ir_value_to_rust(a))
                    .collect::<Vec<_>>()
                    .join(", ");
                
                if let Some(d) = dest {
                    format!("    let {} = {}({});\n",
                        self.ir_value_to_rust(d),
                        self.ir_value_to_rust(func),
                        args_str)
                } else {
                    format!("    {}({});\n",
                        self.ir_value_to_rust(func),
                        args_str)
                }
            }
            IRInstruction::CallAsync { dest, func, args } => {
                let args_str = args.iter()
                    .map(|a| self.ir_value_to_rust(a))
                    .collect::<Vec<_>>()
                    .join(", ");
                
                if let Some(d) = dest {
                    format!("    let {} = {}({}).await;\n",
                        self.ir_value_to_rust(d),
                        self.ir_value_to_rust(func),
                        args_str)
                } else {
                    format!("    {}({}).await;\n",
                        self.ir_value_to_rust(func),
                        args_str)
                }
            }
            IRInstruction::StructAccess { dest, struct_val, field } => {
                format!("    let {} = {}.{};\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_value_to_rust(struct_val),
                    field)
            }
            IRInstruction::StructConstruct { dest, struct_type, fields } => {
                let fields_str = fields.iter()
                    .map(|(name, val)| format!("{}: {}", name, self.ir_value_to_rust(val)))
                    .collect::<Vec<_>>()
                    .join(", ");
                
                format!("    let {} = {} {{\n        {}\n    }};\n", 
                    self.ir_value_to_rust(dest),
                    match struct_type {
                        IRType::Struct(name) => name.clone(),
                        _ => "Struct".to_string(),
                    },
                    fields_str)
            }
            IRInstruction::ListGet { dest, list, index } => {
                format!("    let {} = {}[{}];\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_value_to_rust(list),
                    self.ir_value_to_rust(index))
            }
            IRInstruction::ListSet { list, index, value } => {
                format!("    {}[{}] = {};\n", 
                    self.ir_value_to_rust(list),
                    self.ir_value_to_rust(index),
                    self.ir_value_to_rust(value))
            }
            IRInstruction::MapGet { dest, map, key } => {
                format!("    let {} = {}.get(&{}).cloned();\n", 
                    self.ir_value_to_rust(dest),
                    self.ir_value_to_rust(map),
                    self.ir_value_to_rust(key))
            }
            IRInstruction::MapSet { map, key, value } => {
                format!("    {}.insert({}, {});\n", 
                    self.ir_value_to_rust(map),
                    self.ir_value_to_rust(key),
                    self.ir_value_to_rust(value))
            }
            IRInstruction::Match { value, arms } => {
                let mut output = format!("    match {} {{\n", self.ir_value_to_rust(value));
                for arm in arms {
                    output.push_str(&format!("        // Pattern: {:?}\n", arm.pattern));
                    if let Some(guard) = &arm.guard {
                        output.push_str(&format!("        if {} {{\n", self.ir_value_to_rust(guard)));
                    }
                    output.push_str(&format!("            // Block {:?}\n", arm.body));
                    if arm.guard.is_some() {
                        output.push_str("        }\n");
                    }
                }
                output.push_str("    }\n");
                output
            }
            IRInstruction::Phi { dest, incoming } => {
                // Phi-Node wird zu let mit if-else
                if incoming.len() == 2 {
                    let (block1, val1) = &incoming[0];
                    let (_block2, val2) = &incoming[1];
                    format!("    let {} = if /* block {:?} */ true {{ {} }} else {{ {} }};\n",
                        self.ir_value_to_rust(dest),
                        block1,
                        self.ir_value_to_rust(val1),
                        self.ir_value_to_rust(val2))
                } else {
                    format!("    let {} = /* phi node */;\n", self.ir_value_to_rust(dest))
                }
            }
            _ => {
                format!("    // Instruction: {:?}\n", inst)
            }
        }
    }
    
    /// Konvertiert IR-Type zu Rust-Type
    fn ir_type_to_rust(&self, ty: &IRType) -> String {
        match ty {
            IRType::Void => "()".to_string(),
            IRType::Bool => "bool".to_string(),
            IRType::Int => "i64".to_string(),
            IRType::Float => "f64".to_string(),
            IRType::String => "String".to_string(),
            IRType::Null => "()".to_string(),
            IRType::Any => "serde_json::Value".to_string(),
            IRType::Pointer(inner) => format!("&{}", self.ir_type_to_rust(inner)),
            IRType::Struct(name) => name.clone(),
            IRType::Enum(name) => name.clone(),
            IRType::Function { params, return_type } => {
                let params_str = params.iter()
                    .map(|p| self.ir_type_to_rust(p))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("fn({}) -> {}", params_str, self.ir_type_to_rust(return_type))
            }
            IRType::List(item) => format!("Vec<{}>", self.ir_type_to_rust(item)),
            IRType::Map { key, value } => {
                format!("HashMap<{}, {}>", self.ir_type_to_rust(key), self.ir_type_to_rust(value))
            }
            IRType::Tuple(types) => {
                let types_str = types.iter()
                    .map(|t| self.ir_type_to_rust(t))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", types_str)
            }
            IRType::Optional(inner) => format!("Option<{}>", self.ir_type_to_rust(inner)),
            IRType::Result { ok, err } => {
                format!("Result<{}, {}>", self.ir_type_to_rust(ok), self.ir_type_to_rust(err))
            }
        }
    }
    
    /// Konvertiert IR-Value zu Rust-Code
    fn ir_value_to_rust(&self, val: &IRValue) -> String {
        match val {
            IRValue::Constant(c) => self.ir_constant_to_rust(c),
            IRValue::Variable(v) => v.name.clone(),
            IRValue::Temporary(t) => format!("tmp_{}", t.0),
        }
    }
    
    /// Konvertiert IR-Constant zu Rust-Code
    fn ir_constant_to_rust(&self, c: &IRConstant) -> String {
        match c {
            IRConstant::String(s) => format!("\"{}\"", s.replace("\"", "\\\"").replace("\n", "\\n")),
            IRConstant::Number(n) => n.to_string(),
            IRConstant::Boolean(b) => b.to_string(),
            IRConstant::Null => "()".to_string(),
        }
    }
    
    /// Generiert PHP-Code aus IR
    fn generate_php(&self, module: &IRModule) -> String {
        let mut output = String::new();
        output.push_str("<?php\n");
        output.push_str("// Generated by VelinScript Compiler\n\n");
        
        // Structs generieren
        for s in &module.structs {
            output.push_str(&format!("class {} {{\n", s.name));
            for field in &s.fields {
                output.push_str(&format!("    public ${};\n", field.name));
            }
            output.push_str(&format!("    public function __construct({}) {{\n", 
                s.fields.iter().map(|f| format!("${}", f.name)).collect::<Vec<_>>().join(", ")));
            for field in &s.fields {
                output.push_str(&format!("        $this->{} = ${};\n", field.name, field.name));
            }
            output.push_str("    }\n");
            output.push_str("}\n\n");
        }
        
        // Functions generieren
        for f in &module.functions {
            let params: Vec<String> = f.params.iter()
                .map(|p| format!("${}", p.name))
                .collect();
            output.push_str(&format!("function {}({}) {{\n", f.name, params.join(", ")));
            output.push_str(&self.generate_php_block(&f.body));
            output.push_str("}\n\n");
        }
        
        output
    }
    
    /// Generiert Python-Code aus IR
    fn generate_python(&self, module: &IRModule) -> String {
        let mut output = String::new();
        
        output.push_str("# Generated by VelinScript Compiler\n");
        output.push_str("from dataclasses import dataclass\n");
        output.push_str("from typing import List, Optional, Dict, Any\n\n");
        
        // Structs generieren
        for s in &module.structs {
            output.push_str(&self.generate_python_struct(s));
            output.push_str("\n");
        }
        
        // Functions generieren
        for f in &module.functions {
            output.push_str(&self.generate_python_function(f));
            output.push_str("\n");
        }
        
        output
    }
    
    fn generate_python_struct(&self, s: &IRStruct) -> String {
        let mut output = String::new();
        output.push_str("@dataclass\n");
        output.push_str(&format!("class {}:\n", s.name));
        
        for field in &s.fields {
            let py_type = self.ir_type_to_python(&field.ty);
            output.push_str(&format!("    {}: {}\n", field.name, py_type));
        }
        
        output
    }
    
    fn generate_python_function(&self, func: &IRFunction) -> String {
        let mut output = String::new();
        
        let async_keyword = if func.is_async { "async " } else { "" };
        let params: Vec<String> = func.params.iter()
            .map(|p| format!("{}: {}", p.name, self.ir_type_to_python(&p.ty)))
            .collect();
        
        output.push_str(&format!("{}def {}({}) -> {}:\n", 
            async_keyword, 
            func.name, 
            params.join(", "),
            self.ir_type_to_python(&func.return_type)));
        
        // Body generieren
        output.push_str(&self.generate_python_block(&func.body));
        
        output
    }
    
    fn ir_type_to_python(&self, ty: &IRType) -> String {
        match ty {
            IRType::Void => "None".to_string(),
            IRType::Bool => "bool".to_string(),
            IRType::Int => "int".to_string(),
            IRType::Float => "float".to_string(),
            IRType::String => "str".to_string(),
            IRType::List(item) => format!("List[{}]", self.ir_type_to_python(item)),
            IRType::Map { key, value } => format!("Dict[{}, {}]", self.ir_type_to_python(key), self.ir_type_to_python(value)),
            IRType::Optional(inner) => format!("Optional[{}]", self.ir_type_to_python(inner)),
            IRType::Struct(name) => name.clone(),
            _ => "Any".to_string(),
        }
    }
    
    /// Generiert JavaScript-Code aus IR
    fn generate_javascript(&self, module: &IRModule) -> String {
        let mut output = String::new();
        
        output.push_str("// Generated by VelinScript Compiler\n\n");
        
        // Structs/Classes generieren
        for s in &module.structs {
            output.push_str(&self.generate_javascript_struct(s));
            output.push_str("\n");
        }
        
        // Functions generieren
        for f in &module.functions {
            output.push_str(&self.generate_javascript_function(f));
            output.push_str("\n");
        }
        
        output
    }
    
    fn generate_javascript_struct(&self, s: &IRStruct) -> String {
        let mut output = String::new();
        output.push_str(&format!("class {} {{\n", s.name));
        
        // Constructor
        let params: Vec<String> = s.fields.iter().map(|f| f.name.clone()).collect();
        output.push_str(&format!("    constructor({}) {{\n", params.join(", ")));
        for field in &s.fields {
            output.push_str(&format!("        this.{} = {};\n", field.name, field.name));
        }
        output.push_str("    }\n");
        output.push_str("}\n");
        
        output
    }
    
    fn generate_javascript_function(&self, func: &IRFunction) -> String {
        let mut output = String::new();
        
        let async_keyword = if func.is_async { "async " } else { "" };
        let params: Vec<String> = func.params.iter()
            .map(|p| p.name.clone())
            .collect();
        
        output.push_str(&format!("{}function {}({}) {{\n", 
            async_keyword, 
            func.name, 
            params.join(", ")));
        
        // Body generieren
        output.push_str(&self.generate_javascript_block(&func.body));
        output.push_str("}\n");
        
        output
    }
    
    /// Generiert TypeScript-Code aus IR
    fn generate_typescript(&self, module: &IRModule) -> String {
        let mut output = String::new();
        output.push_str("// Generated by VelinScript Compiler\n\n");
        
        // Structs/Interfaces generieren
        for s in &module.structs {
            output.push_str(&format!("interface {} {{\n", s.name));
            for field in &s.fields {
                let ts_type = self.ir_type_to_typescript(&field.ty);
                output.push_str(&format!("    {}: {};\n", field.name, ts_type));
            }
            output.push_str("}\n\n");
        }
        
        // Functions generieren
        for f in &module.functions {
            let async_keyword = if f.is_async { "async " } else { "" };
            let params: Vec<String> = f.params.iter()
                .map(|p| format!("{}: {}", p.name, self.ir_type_to_typescript(&p.ty)))
                .collect();
            let ret_type = self.ir_type_to_typescript(&f.return_type);
            output.push_str(&format!("{}function {}({}): Promise<{}> {{\n", 
                async_keyword, f.name, params.join(", "), ret_type));
            output.push_str(&self.generate_typescript_block(&f.body));
            output.push_str("}\n\n");
        }
        
        output
    }
    
    fn ir_type_to_typescript(&self, ty: &IRType) -> String {
        match ty {
            IRType::Void => "void".to_string(),
            IRType::Bool => "boolean".to_string(),
            IRType::Int => "number".to_string(),
            IRType::Float => "number".to_string(),
            IRType::String => "string".to_string(),
            IRType::List(item) => format!("{}[]", self.ir_type_to_typescript(item)),
            IRType::Map { key, value } => format!("Record<{}, {}>", self.ir_type_to_typescript(key), self.ir_type_to_typescript(value)),
            IRType::Optional(inner) => format!("{} | null", self.ir_type_to_typescript(inner)),
            IRType::Struct(name) => name.clone(),
            _ => "any".to_string(),
        }
    }
    
    fn generate_typescript_block(&self, block: &IRBlock) -> String {
        let mut output = String::new();
        for instruction in &block.instructions {
            output.push_str(&self.generate_typescript_instruction(instruction));
        }
        output
    }
    
    fn generate_typescript_instruction(&self, inst: &IRInstruction) -> String {
        match inst {
            IRInstruction::Return { value } => {
                if let Some(v) = value {
                    format!("    return {};\n", self.ir_value_to_typescript(v))
                } else {
                    "    return;\n".to_string()
                }
            }
            IRInstruction::Store { dest, value } => {
                format!("    let {} = {};\n", self.ir_value_to_typescript(dest), self.ir_value_to_typescript(value))
            }
            IRInstruction::Add { dest, left, right } => {
                format!("    let {} = {} + {};\n", self.ir_value_to_typescript(dest), self.ir_value_to_typescript(left), self.ir_value_to_typescript(right))
            }
            IRInstruction::Subtract { dest, left, right } => {
                format!("    let {} = {} - {};\n", self.ir_value_to_typescript(dest), self.ir_value_to_typescript(left), self.ir_value_to_typescript(right))
            }
            IRInstruction::Multiply { dest, left, right } => {
                format!("    let {} = {} * {};\n", self.ir_value_to_typescript(dest), self.ir_value_to_typescript(left), self.ir_value_to_typescript(right))
            }
            IRInstruction::Divide { dest, left, right } => {
                format!("    let {} = {} / {};\n", self.ir_value_to_typescript(dest), self.ir_value_to_typescript(left), self.ir_value_to_typescript(right))
            }
            IRInstruction::Eq { dest, left, right } => {
                format!("    let {} = {} === {};\n", self.ir_value_to_typescript(dest), self.ir_value_to_typescript(left), self.ir_value_to_typescript(right))
            }
            IRInstruction::NotEq { dest, left, right } => {
                format!("    let {} = {} !== {};\n", self.ir_value_to_typescript(dest), self.ir_value_to_typescript(left), self.ir_value_to_typescript(right))
            }
            IRInstruction::Call { dest, func, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.ir_value_to_typescript(a)).collect();
                if let Some(d) = dest {
                    format!("    let {} = {}({});\n", self.ir_value_to_typescript(d), self.ir_value_to_typescript(func), args_str.join(", "))
                } else {
                    format!("    {}({});\n", self.ir_value_to_typescript(func), args_str.join(", "))
                }
            }
            IRInstruction::CallAsync { dest, func, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.ir_value_to_typescript(a)).collect();
                if let Some(d) = dest {
                    format!("    let {} = await {}({});\n", self.ir_value_to_typescript(d), self.ir_value_to_typescript(func), args_str.join(", "))
                } else {
                    format!("    await {}({});\n", self.ir_value_to_typescript(func), args_str.join(", "))
                }
            }
            IRInstruction::Branch { condition, then_block, else_block } => {
                format!("    if ({}) {{\n        // goto block_{}\n    }} else {{\n        // goto block_{}\n    }}\n", 
                    self.ir_value_to_typescript(condition), then_block.0, else_block.0)
            }
            _ => format!("    // {:#?}\n", inst)
        }
    }
    
    fn ir_value_to_typescript(&self, val: &IRValue) -> String {
        match val {
            IRValue::Constant(c) => self.ir_constant_to_typescript(c),
            IRValue::Variable(v) => v.name.clone(),
            IRValue::Temporary(t) => format!("tmp_{}", t.0),
        }
    }
    
    fn ir_constant_to_typescript(&self, c: &IRConstant) -> String {
        match c {
            IRConstant::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            IRConstant::Number(n) => n.to_string(),
            IRConstant::Boolean(b) => if *b { "true".to_string() } else { "false".to_string() },
            IRConstant::Null => "null".to_string(),
        }
    }
    
    /// Generiert Go-Code aus IR
    fn generate_go(&self, module: &IRModule) -> String {
        let mut output = String::new();
        output.push_str("// Generated by VelinScript Compiler\n\n");
        output.push_str("package main\n\n");
        
        // Structs generieren
        for s in &module.structs {
            output.push_str(&format!("type {} struct {{\n", s.name));
            for field in &s.fields {
                let go_type = self.ir_type_to_go(&field.ty);
                output.push_str(&format!("    {} {}\n", 
                    field.name.chars().next().unwrap().to_uppercase().collect::<String>() + &field.name[1..],
                    go_type));
            }
            output.push_str("}\n\n");
        }
        
        // Functions generieren
        for f in &module.functions {
            let params: Vec<String> = f.params.iter()
                .map(|p| format!("{} {}", p.name, self.ir_type_to_go(&p.ty)))
                .collect();
            let ret_type = self.ir_type_to_go(&f.return_type);
            output.push_str(&format!("func {}({}) {} {{\n", f.name, params.join(", "), ret_type));
            output.push_str(&self.generate_go_block(&f.body));
            output.push_str("}\n\n");
        }
        
        output
    }
    
    fn ir_type_to_go(&self, ty: &IRType) -> String {
        match ty {
            IRType::Void => "".to_string(),
            IRType::Bool => "bool".to_string(),
            IRType::Int => "int".to_string(),
            IRType::Float => "float64".to_string(),
            IRType::String => "string".to_string(),
            IRType::List(item) => format!("[]{}", self.ir_type_to_go(item)),
            IRType::Map { key, value } => format!("map[{}]{}", self.ir_type_to_go(key), self.ir_type_to_go(value)),
            IRType::Optional(inner) => format!("*{}", self.ir_type_to_go(inner)),
            IRType::Struct(name) => name.clone(),
            _ => "interface{}".to_string(),
        }
    }
    
    /// Generiert Java-Code aus IR
    fn generate_java(&self, module: &IRModule) -> String {
        let mut output = String::new();
        output.push_str("// Generated by VelinScript Compiler\n\n");
        output.push_str("public class Main {\n\n");
        
        // Structs generieren
        for s in &module.structs {
            output.push_str(&format!("    public static class {} {{\n", s.name));
            for field in &s.fields {
                let java_type = self.ir_type_to_java(&field.ty);
                output.push_str(&format!("        public {} {};\n", java_type, field.name));
            }
            output.push_str("    }\n\n");
        }
        
        // Functions generieren
        for f in &module.functions {
            let params: Vec<String> = f.params.iter()
                .map(|p| format!("{} {}", self.ir_type_to_java(&p.ty), p.name))
                .collect();
            let ret_type = self.ir_type_to_java(&f.return_type);
            output.push_str(&format!("    public static {} {}({}) {{\n", ret_type, f.name, params.join(", ")));
            output.push_str(&self.generate_java_block(&f.body));
            output.push_str("    }\n\n");
        }
        
        output.push_str("}\n");
        output
    }
    
    fn ir_type_to_java(&self, ty: &IRType) -> String {
        match ty {
            IRType::Void => "void".to_string(),
            IRType::Bool => "boolean".to_string(),
            IRType::Int => "int".to_string(),
            IRType::Float => "double".to_string(),
            IRType::String => "String".to_string(),
            IRType::List(item) => format!("List<{}>", self.ir_type_to_java(item)),
            IRType::Map { key, value } => format!("Map<{}, {}>", self.ir_type_to_java(key), self.ir_type_to_java(value)),
            IRType::Optional(inner) => format!("Optional<{}>", self.ir_type_to_java(inner)),
            IRType::Struct(name) => name.clone(),
            _ => "Object".to_string(),
        }
    }
    
    /// Generiert C#-Code aus IR
    fn generate_csharp(&self, module: &IRModule) -> String {
        let mut output = String::new();
        output.push_str("// Generated by VelinScript Compiler\n\n");
        output.push_str("using System;\n");
        output.push_str("using System.Collections.Generic;\n\n");
        
        // Structs/Classes generieren
        for s in &module.structs {
            output.push_str(&format!("public class {} {{\n", s.name));
            for field in &s.fields {
                let cs_type = self.ir_type_to_csharp(&field.ty);
                output.push_str(&format!("    public {} {} {{ get; set; }}\n", cs_type, field.name));
            }
            output.push_str("}\n\n");
        }
        
        // Functions generieren
        for f in &module.functions {
            let async_keyword = if f.is_async { "async " } else { "" };
            let params: Vec<String> = f.params.iter()
                .map(|p| format!("{} {}", self.ir_type_to_csharp(&p.ty), p.name))
                .collect();
            let ret_type = self.ir_type_to_csharp(&f.return_type);
            output.push_str(&format!("    public static {} {} {}({}) {{\n", 
                async_keyword, ret_type, f.name, params.join(", ")));
            output.push_str(&self.generate_csharp_block(&f.body));
            output.push_str("    }\n\n");
        }
        
        output
    }
    
    fn ir_type_to_csharp(&self, ty: &IRType) -> String {
        match ty {
            IRType::Void => "void".to_string(),
            IRType::Bool => "bool".to_string(),
            IRType::Int => "int".to_string(),
            IRType::Float => "double".to_string(),
            IRType::String => "string".to_string(),
            IRType::List(item) => format!("List<{}>", self.ir_type_to_csharp(item)),
            IRType::Map { key, value } => format!("Dictionary<{}, {}>", self.ir_type_to_csharp(key), self.ir_type_to_csharp(value)),
            IRType::Optional(inner) => format!("{}?", self.ir_type_to_csharp(inner)),
            IRType::Struct(name) => name.clone(),
            _ => "object".to_string(),
        }
    }
    
    // Block-Generierung für alle Targets
    fn generate_php_block(&self, block: &IRBlock) -> String {
        let mut output = String::new();
        for instruction in &block.instructions {
            output.push_str(&self.generate_php_instruction(instruction));
        }
        output
    }
    
    fn generate_php_instruction(&self, inst: &IRInstruction) -> String {
        match inst {
            IRInstruction::Return { value } => {
                if let Some(v) = value {
                    format!("    return {};\n", self.ir_value_to_php(v))
                } else {
                    "    return;\n".to_string()
                }
            }
            IRInstruction::Store { dest, value } => {
                format!("    ${} = {};\n", self.ir_value_to_php(dest), self.ir_value_to_php(value))
            }
            IRInstruction::Add { dest, left, right } => {
                format!("    ${} = {} + {};\n", self.ir_value_to_php(dest), self.ir_value_to_php(left), self.ir_value_to_php(right))
            }
            IRInstruction::Subtract { dest, left, right } => {
                format!("    ${} = {} - {};\n", self.ir_value_to_php(dest), self.ir_value_to_php(left), self.ir_value_to_php(right))
            }
            IRInstruction::Multiply { dest, left, right } => {
                format!("    ${} = {} * {};\n", self.ir_value_to_php(dest), self.ir_value_to_php(left), self.ir_value_to_php(right))
            }
            IRInstruction::Divide { dest, left, right } => {
                format!("    ${} = {} / {};\n", self.ir_value_to_php(dest), self.ir_value_to_php(left), self.ir_value_to_php(right))
            }
            IRInstruction::Eq { dest, left, right } => {
                format!("    ${} = {} == {};\n", self.ir_value_to_php(dest), self.ir_value_to_php(left), self.ir_value_to_php(right))
            }
            IRInstruction::NotEq { dest, left, right } => {
                format!("    ${} = {} != {};\n", self.ir_value_to_php(dest), self.ir_value_to_php(left), self.ir_value_to_php(right))
            }
            IRInstruction::Call { dest, func, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.ir_value_to_php(a)).collect();
                if let Some(d) = dest {
                    format!("    ${} = {}({});\n", self.ir_value_to_php(d), self.ir_value_to_php(func), args_str.join(", "))
                } else {
                    format!("    {}({});\n", self.ir_value_to_php(func), args_str.join(", "))
                }
            }
            IRInstruction::Branch { condition, then_block, else_block } => {
                format!("    if ({}) {{\n        // goto block_{}\n    }} else {{\n        // goto block_{}\n    }}\n", 
                    self.ir_value_to_php(condition), then_block.0, else_block.0)
            }
            _ => format!("    // {:#?}\n", inst)
        }
    }
    
    fn ir_value_to_php(&self, val: &IRValue) -> String {
        match val {
            IRValue::Constant(c) => self.ir_constant_to_php(c),
            IRValue::Variable(v) => format!("${}", v.name),
            IRValue::Temporary(t) => format!("$tmp_{}", t.0),
        }
    }
    
    fn ir_constant_to_php(&self, c: &IRConstant) -> String {
        match c {
            IRConstant::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            IRConstant::Number(n) => n.to_string(),
            IRConstant::Boolean(b) => if *b { "true".to_string() } else { "false".to_string() },
            IRConstant::Null => "null".to_string(),
        }
    }
    
    fn generate_python_block(&self, block: &IRBlock) -> String {
        let mut output = String::new();
        for instruction in &block.instructions {
            output.push_str(&self.generate_python_instruction(instruction));
        }
        output
    }
    
    fn generate_python_instruction(&self, inst: &IRInstruction) -> String {
        match inst {
            IRInstruction::Return { value } => {
                if let Some(v) = value {
                    format!("    return {}\n", self.ir_value_to_python(v))
                } else {
                    "    return\n".to_string()
                }
            }
            IRInstruction::Store { dest, value } => {
                format!("    {} = {}\n", self.ir_value_to_python(dest), self.ir_value_to_python(value))
            }
            IRInstruction::Add { dest, left, right } => {
                format!("    {} = {} + {}\n", self.ir_value_to_python(dest), self.ir_value_to_python(left), self.ir_value_to_python(right))
            }
            IRInstruction::Subtract { dest, left, right } => {
                format!("    {} = {} - {}\n", self.ir_value_to_python(dest), self.ir_value_to_python(left), self.ir_value_to_python(right))
            }
            IRInstruction::Multiply { dest, left, right } => {
                format!("    {} = {} * {}\n", self.ir_value_to_python(dest), self.ir_value_to_python(left), self.ir_value_to_python(right))
            }
            IRInstruction::Divide { dest, left, right } => {
                format!("    {} = {} / {}\n", self.ir_value_to_python(dest), self.ir_value_to_python(left), self.ir_value_to_python(right))
            }
            IRInstruction::Eq { dest, left, right } => {
                format!("    {} = {} == {}\n", self.ir_value_to_python(dest), self.ir_value_to_python(left), self.ir_value_to_python(right))
            }
            IRInstruction::NotEq { dest, left, right } => {
                format!("    {} = {} != {}\n", self.ir_value_to_python(dest), self.ir_value_to_python(left), self.ir_value_to_python(right))
            }
            IRInstruction::Call { dest, func, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.ir_value_to_python(a)).collect();
                if let Some(d) = dest {
                    format!("    {} = {}({})\n", self.ir_value_to_python(d), self.ir_value_to_python(func), args_str.join(", "))
                } else {
                    format!("    {}({})\n", self.ir_value_to_python(func), args_str.join(", "))
                }
            }
            IRInstruction::CallAsync { dest, func, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.ir_value_to_python(a)).collect();
                if let Some(d) = dest {
                    format!("    {} = await {}({})\n", self.ir_value_to_python(d), self.ir_value_to_python(func), args_str.join(", "))
                } else {
                    format!("    await {}({})\n", self.ir_value_to_python(func), args_str.join(", "))
                }
            }
            IRInstruction::Branch { condition, then_block, else_block } => {
                format!("    if {}:\n        # goto block_{}\n    else:\n        # goto block_{}\n", 
                    self.ir_value_to_python(condition), then_block.0, else_block.0)
            }
            _ => format!("    # {:#?}\n", inst)
        }
    }
    
    fn ir_value_to_python(&self, val: &IRValue) -> String {
        match val {
            IRValue::Constant(c) => self.ir_constant_to_python(c),
            IRValue::Variable(v) => v.name.clone(),
            IRValue::Temporary(t) => format!("tmp_{}", t.0),
        }
    }
    
    fn ir_constant_to_python(&self, c: &IRConstant) -> String {
        match c {
            IRConstant::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            IRConstant::Number(n) => n.to_string(),
            IRConstant::Boolean(b) => if *b { "True".to_string() } else { "False".to_string() },
            IRConstant::Null => "None".to_string(),
        }
    }
    
    fn generate_javascript_block(&self, block: &IRBlock) -> String {
        let mut output = String::new();
        for instruction in &block.instructions {
            output.push_str(&self.generate_javascript_instruction(instruction));
        }
        output
    }
    
    fn generate_javascript_instruction(&self, inst: &IRInstruction) -> String {
        match inst {
            IRInstruction::Return { value } => {
                if let Some(v) = value {
                    format!("    return {};\n", self.ir_value_to_javascript(v))
                } else {
                    "    return;\n".to_string()
                }
            }
            IRInstruction::Store { dest, value } => {
                format!("    let {} = {};\n", self.ir_value_to_javascript(dest), self.ir_value_to_javascript(value))
            }
            IRInstruction::Add { dest, left, right } => {
                format!("    let {} = {} + {};\n", self.ir_value_to_javascript(dest), self.ir_value_to_javascript(left), self.ir_value_to_javascript(right))
            }
            IRInstruction::Subtract { dest, left, right } => {
                format!("    let {} = {} - {};\n", self.ir_value_to_javascript(dest), self.ir_value_to_javascript(left), self.ir_value_to_javascript(right))
            }
            IRInstruction::Multiply { dest, left, right } => {
                format!("    let {} = {} * {};\n", self.ir_value_to_javascript(dest), self.ir_value_to_javascript(left), self.ir_value_to_javascript(right))
            }
            IRInstruction::Divide { dest, left, right } => {
                format!("    let {} = {} / {};\n", self.ir_value_to_javascript(dest), self.ir_value_to_javascript(left), self.ir_value_to_javascript(right))
            }
            IRInstruction::Eq { dest, left, right } => {
                format!("    let {} = {} === {};\n", self.ir_value_to_javascript(dest), self.ir_value_to_javascript(left), self.ir_value_to_javascript(right))
            }
            IRInstruction::NotEq { dest, left, right } => {
                format!("    let {} = {} !== {};\n", self.ir_value_to_javascript(dest), self.ir_value_to_javascript(left), self.ir_value_to_javascript(right))
            }
            IRInstruction::Call { dest, func, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.ir_value_to_javascript(a)).collect();
                if let Some(d) = dest {
                    format!("    let {} = {}({});\n", self.ir_value_to_javascript(d), self.ir_value_to_javascript(func), args_str.join(", "))
                } else {
                    format!("    {}({});\n", self.ir_value_to_javascript(func), args_str.join(", "))
                }
            }
            IRInstruction::CallAsync { dest, func, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.ir_value_to_javascript(a)).collect();
                if let Some(d) = dest {
                    format!("    let {} = await {}({});\n", self.ir_value_to_javascript(d), self.ir_value_to_javascript(func), args_str.join(", "))
                } else {
                    format!("    await {}({});\n", self.ir_value_to_javascript(func), args_str.join(", "))
                }
            }
            IRInstruction::Branch { condition, then_block, else_block } => {
                format!("    if ({}) {{\n        // goto block_{}\n    }} else {{\n        // goto block_{}\n    }}\n", 
                    self.ir_value_to_javascript(condition), then_block.0, else_block.0)
            }
            _ => format!("    // {:#?}\n", inst)
        }
    }
    
    fn ir_value_to_javascript(&self, val: &IRValue) -> String {
        match val {
            IRValue::Constant(c) => self.ir_constant_to_javascript(c),
            IRValue::Variable(v) => v.name.clone(),
            IRValue::Temporary(t) => format!("tmp_{}", t.0),
        }
    }
    
    fn ir_constant_to_javascript(&self, c: &IRConstant) -> String {
        match c {
            IRConstant::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            IRConstant::Number(n) => n.to_string(),
            IRConstant::Boolean(b) => if *b { "true".to_string() } else { "false".to_string() },
            IRConstant::Null => "null".to_string(),
        }
    }
    
    fn generate_go_block(&self, block: &IRBlock) -> String {
        let mut output = String::new();
        for instruction in &block.instructions {
            output.push_str(&self.generate_go_instruction(instruction));
        }
        output
    }
    
    fn generate_go_instruction(&self, inst: &IRInstruction) -> String {
        match inst {
            IRInstruction::Return { value } => {
                if let Some(v) = value {
                    format!("    return {}\n", self.ir_value_to_go(v))
                } else {
                    "    return\n".to_string()
                }
            }
            IRInstruction::Store { dest, value } => {
                format!("    {} := {}\n", self.ir_value_to_go(dest), self.ir_value_to_go(value))
            }
            IRInstruction::Add { dest, left, right } => {
                format!("    {} := {} + {}\n", self.ir_value_to_go(dest), self.ir_value_to_go(left), self.ir_value_to_go(right))
            }
            IRInstruction::Subtract { dest, left, right } => {
                format!("    {} := {} - {}\n", self.ir_value_to_go(dest), self.ir_value_to_go(left), self.ir_value_to_go(right))
            }
            IRInstruction::Multiply { dest, left, right } => {
                format!("    {} := {} * {}\n", self.ir_value_to_go(dest), self.ir_value_to_go(left), self.ir_value_to_go(right))
            }
            IRInstruction::Divide { dest, left, right } => {
                format!("    {} := {} / {}\n", self.ir_value_to_go(dest), self.ir_value_to_go(left), self.ir_value_to_go(right))
            }
            IRInstruction::Eq { dest, left, right } => {
                format!("    {} := {} == {}\n", self.ir_value_to_go(dest), self.ir_value_to_go(left), self.ir_value_to_go(right))
            }
            IRInstruction::NotEq { dest, left, right } => {
                format!("    {} := {} != {}\n", self.ir_value_to_go(dest), self.ir_value_to_go(left), self.ir_value_to_go(right))
            }
            IRInstruction::Call { dest, func, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.ir_value_to_go(a)).collect();
                if let Some(d) = dest {
                    format!("    {} := {}({})\n", self.ir_value_to_go(d), self.ir_value_to_go(func), args_str.join(", "))
                } else {
                    format!("    {}({})\n", self.ir_value_to_go(func), args_str.join(", "))
                }
            }
            IRInstruction::Branch { condition, then_block, else_block } => {
                format!("    if {} {{\n        // goto block_{}\n    }} else {{\n        // goto block_{}\n    }}\n", 
                    self.ir_value_to_go(condition), then_block.0, else_block.0)
            }
            _ => format!("    // {:#?}\n", inst)
        }
    }
    
    fn ir_value_to_go(&self, val: &IRValue) -> String {
        match val {
            IRValue::Constant(c) => self.ir_constant_to_go(c),
            IRValue::Variable(v) => v.name.clone(),
            IRValue::Temporary(t) => format!("tmp_{}", t.0),
        }
    }
    
    fn ir_constant_to_go(&self, c: &IRConstant) -> String {
        match c {
            IRConstant::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            IRConstant::Number(n) => n.to_string(),
            IRConstant::Boolean(b) => if *b { "true".to_string() } else { "false".to_string() },
            IRConstant::Null => "nil".to_string(),
        }
    }
    
    fn generate_java_block(&self, block: &IRBlock) -> String {
        let mut output = String::new();
        for instruction in &block.instructions {
            output.push_str(&self.generate_java_instruction(instruction));
        }
        output
    }
    
    fn generate_java_instruction(&self, inst: &IRInstruction) -> String {
        match inst {
            IRInstruction::Return { value } => {
                if let Some(v) = value {
                    format!("        return {};\n", self.ir_value_to_java(v))
                } else {
                    "        return;\n".to_string()
                }
            }
            IRInstruction::Store { dest, value } => {
                format!("        {} = {};\n", self.ir_value_to_java(dest), self.ir_value_to_java(value))
            }
            IRInstruction::Add { dest, left, right } => {
                format!("        {} = {} + {};\n", self.ir_value_to_java(dest), self.ir_value_to_java(left), self.ir_value_to_java(right))
            }
            IRInstruction::Subtract { dest, left, right } => {
                format!("        {} = {} - {};\n", self.ir_value_to_java(dest), self.ir_value_to_java(left), self.ir_value_to_java(right))
            }
            IRInstruction::Multiply { dest, left, right } => {
                format!("        {} = {} * {};\n", self.ir_value_to_java(dest), self.ir_value_to_java(left), self.ir_value_to_java(right))
            }
            IRInstruction::Divide { dest, left, right } => {
                format!("        {} = {} / {};\n", self.ir_value_to_java(dest), self.ir_value_to_java(left), self.ir_value_to_java(right))
            }
            IRInstruction::Eq { dest, left, right } => {
                format!("        {} = {} == {};\n", self.ir_value_to_java(dest), self.ir_value_to_java(left), self.ir_value_to_java(right))
            }
            IRInstruction::NotEq { dest, left, right } => {
                format!("        {} = {} != {};\n", self.ir_value_to_java(dest), self.ir_value_to_java(left), self.ir_value_to_java(right))
            }
            IRInstruction::Call { dest, func, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.ir_value_to_java(a)).collect();
                if let Some(d) = dest {
                    format!("        {} = {}({});\n", self.ir_value_to_java(d), self.ir_value_to_java(func), args_str.join(", "))
                } else {
                    format!("        {}({});\n", self.ir_value_to_java(func), args_str.join(", "))
                }
            }
            IRInstruction::Branch { condition, then_block, else_block } => {
                format!("        if ({}) {{\n            // goto block_{}\n        }} else {{\n            // goto block_{}\n        }}\n", 
                    self.ir_value_to_java(condition), then_block.0, else_block.0)
            }
            _ => format!("        // {:#?}\n", inst)
        }
    }
    
    fn ir_value_to_java(&self, val: &IRValue) -> String {
        match val {
            IRValue::Constant(c) => self.ir_constant_to_java(c),
            IRValue::Variable(v) => v.name.clone(),
            IRValue::Temporary(t) => format!("tmp_{}", t.0),
        }
    }
    
    fn ir_constant_to_java(&self, c: &IRConstant) -> String {
        match c {
            IRConstant::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            IRConstant::Number(n) => n.to_string(),
            IRConstant::Boolean(b) => if *b { "true".to_string() } else { "false".to_string() },
            IRConstant::Null => "null".to_string(),
        }
    }
    
    fn generate_csharp_block(&self, block: &IRBlock) -> String {
        let mut output = String::new();
        for instruction in &block.instructions {
            output.push_str(&self.generate_csharp_instruction(instruction));
        }
        output
    }
    
    fn generate_csharp_instruction(&self, inst: &IRInstruction) -> String {
        match inst {
            IRInstruction::Return { value } => {
                if let Some(v) = value {
                    format!("        return {};\n", self.ir_value_to_csharp(v))
                } else {
                    "        return;\n".to_string()
                }
            }
            IRInstruction::Store { dest, value } => {
                format!("        {} = {};\n", self.ir_value_to_csharp(dest), self.ir_value_to_csharp(value))
            }
            _ => format!("        // {:#?}\n", inst)
        }
    }
    
    fn ir_value_to_csharp(&self, val: &IRValue) -> String {
        match val {
            IRValue::Constant(c) => self.ir_constant_to_csharp(c),
            IRValue::Variable(v) => v.name.clone(),
            IRValue::Temporary(t) => format!("tmp_{}", t.0),
        }
    }
    
    fn ir_constant_to_csharp(&self, c: &IRConstant) -> String {
        match c {
            IRConstant::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            IRConstant::Number(n) => n.to_string(),
            IRConstant::Boolean(b) => if *b { "true".to_string() } else { "false".to_string() },
            IRConstant::Null => "null".to_string(),
        }
    }
}
