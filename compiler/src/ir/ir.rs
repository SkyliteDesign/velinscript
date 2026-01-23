/// IR-Strukturen für VelinScript
/// 
/// Dieses Modul definiert alle IR-Strukturen für die Intermediate Representation.
/// Die IR verwendet SSA (Single Static Assignment) Format.

use crate::parser::ast::Visibility;

/// Eindeutige ID für einen Block
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);

impl BlockId {
    pub fn new(id: usize) -> Self {
        BlockId(id)
    }
}

/// Eindeutige ID für eine temporäre Variable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TempId(pub usize);

impl TempId {
    pub fn new(id: usize) -> Self {
        TempId(id)
    }
}

/// Eindeutige ID für eine Variable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarId(pub usize);

impl VarId {
    pub fn new(id: usize) -> Self {
        VarId(id)
    }
}

/// Haupt-IR-Modul
/// 
/// Repräsentiert ein vollständiges VelinScript-Modul in IR-Format.
#[derive(Debug, Clone)]
pub struct IRModule {
    pub name: String,
    pub functions: Vec<IRFunction>,
    pub structs: Vec<IRStruct>,
    pub enums: Vec<IREnum>,
    pub constants: Vec<IRConstant>,
}

impl IRModule {
    pub fn new(name: String) -> Self {
        IRModule {
            name,
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            constants: Vec::new(),
        }
    }
}

/// IR-Funktion
/// 
/// Repräsentiert eine Funktion in IR-Format mit SSA-Body.
#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub params: Vec<IRParameter>,
    pub return_type: IRType,
    pub body: IRBlock,
    pub attributes: Vec<IRAttribute>,
    pub is_async: bool,
    pub visibility: Visibility,
}

/// IR-Parameter
/// 
/// Repräsentiert einen Funktionsparameter mit Typ und Ownership-Information.
#[derive(Debug, Clone)]
pub struct IRParameter {
    pub name: String,
    pub ty: IRType,
    pub ownership: Ownership,
}

/// IR-Attribut (aus Decorator)
/// 
/// Konvertiert aus AST-Decorator zu IR-Attribut.
#[derive(Debug, Clone)]
pub struct IRAttribute {
    pub name: String,
    pub args: Vec<IRAttributeArg>,
}

/// IR-Attribut-Argument
#[derive(Debug, Clone)]
pub enum IRAttributeArg {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
}

/// IR-Block (SSA - Single Static Assignment)
/// 
/// Ein Block enthält eine Liste von Instructions und Informationen über
/// Control Flow (Predecessors, Successors).
#[derive(Debug, Clone)]
pub struct IRBlock {
    pub id: BlockId,
    pub instructions: Vec<IRInstruction>,
    pub predecessors: Vec<BlockId>,
    pub successors: Vec<BlockId>,
}

impl IRBlock {
    pub fn new(id: BlockId) -> Self {
        IRBlock {
            id,
            instructions: Vec::new(),
            predecessors: Vec::new(),
            successors: Vec::new(),
        }
    }
}

/// IR-Instruction (SSA-Format)
/// 
/// Jede Instruction produziert höchstens einen Wert (SSA-Prinzip).
#[derive(Debug, Clone)]
pub enum IRInstruction {
    // Arithmetik
    Add {
        dest: IRValue,
        left: IRValue,
        right: IRValue,
    },
    Subtract {
        dest: IRValue,
        left: IRValue,
        right: IRValue,
    },
    Multiply {
        dest: IRValue,
        left: IRValue,
        right: IRValue,
    },
    Divide {
        dest: IRValue,
        left: IRValue,
        right: IRValue,
    },
    Modulo {
        dest: IRValue,
        left: IRValue,
        right: IRValue,
    },
    
    // Vergleichs-Operationen
    Eq {
        dest: IRValue,
        left: IRValue,
        right: IRValue,
    },
    NotEq {
        dest: IRValue,
        left: IRValue,
        right: IRValue,
    },
    Lt {
        dest: IRValue,
        left: IRValue,
        right: IRValue,
    },
    Gt {
        dest: IRValue,
        left: IRValue,
        right: IRValue,
    },
    LtEq {
        dest: IRValue,
        left: IRValue,
        right: IRValue,
    },
    GtEq {
        dest: IRValue,
        left: IRValue,
        right: IRValue,
    },
    
    // Logische Operationen
    And {
        dest: IRValue,
        left: IRValue,
        right: IRValue,
    },
    Or {
        dest: IRValue,
        left: IRValue,
        right: IRValue,
    },
    Not {
        dest: IRValue,
        operand: IRValue,
    },
    
    // Speicher-Operationen
    Load {
        dest: IRValue,
        source: IRValue,
    },
    Store {
        dest: IRValue,
        value: IRValue,
    },
    Alloca {
        dest: IRValue,
        ty: IRType,
    },
    
    // Kontrollfluss
    Branch {
        condition: IRValue,
        then_block: BlockId,
        else_block: BlockId,
    },
    Jump {
        target: BlockId,
    },
    Return {
        value: Option<IRValue>,
    },
    
    // Funktions-Aufrufe
    Call {
        dest: Option<IRValue>,
        func: IRValue,
        args: Vec<IRValue>,
    },
    CallAsync {
        dest: Option<IRValue>,
        func: IRValue,
        args: Vec<IRValue>,
    },
    
    // Struct/Enum Operationen
    StructAccess {
        dest: IRValue,
        struct_val: IRValue,
        field: String,
    },
    StructConstruct {
        dest: IRValue,
        struct_type: IRType,
        fields: Vec<(String, IRValue)>,
    },
    EnumConstruct {
        dest: IRValue,
        enum_type: IRType,
        variant: String,
        data: Option<IRValue>,
    },
    
    // Pattern Matching
    Match {
        value: IRValue,
        arms: Vec<IRMatchArm>,
    },
    
    // Collections
    ListGet {
        dest: IRValue,
        list: IRValue,
        index: IRValue,
    },
    ListSet {
        list: IRValue,
        index: IRValue,
        value: IRValue,
    },
    MapGet {
        dest: IRValue,
        map: IRValue,
        key: IRValue,
    },
    MapSet {
        map: IRValue,
        key: IRValue,
        value: IRValue,
    },
    
    // Phi-Node (für SSA bei Control Flow)
    Phi {
        dest: IRValue,
        incoming: Vec<(BlockId, IRValue)>,
    },
}

/// IR-Match-Arm
/// 
/// Repräsentiert einen Match-Arm mit Pattern und Body.
#[derive(Debug, Clone)]
pub struct IRMatchArm {
    pub pattern: IRPattern,
    pub guard: Option<IRValue>,
    pub body: BlockId,
}

/// IR-Pattern
/// 
/// Repräsentiert ein Pattern für Pattern Matching.
#[derive(Debug, Clone)]
pub enum IRPattern {
    Literal(IRConstant),
    Identifier(String),
    Tuple(Vec<IRPattern>),
    Struct {
        name: String,
        fields: Vec<(String, IRPattern)>,
    },
    EnumVariant {
        name: String,
        data: Option<Vec<IRPattern>>,
    },
    Wildcard,
}

/// IR-Wert (SSA)
/// 
/// Ein Wert in SSA-Format kann eine Konstante, Variable oder temporäre Variable sein.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IRValue {
    Constant(IRConstant),
    Variable(IRVariable),
    Temporary(TempId),
}

impl IRValue {
    pub fn get_type(&self) -> IRType {
        match self {
            IRValue::Constant(c) => c.get_type(),
            IRValue::Variable(v) => v.ty.clone(),
            IRValue::Temporary(_) => IRType::Any, // Wird später durch Type-Checker gefüllt
        }
    }
}

/// IR-Konstante
/// 
/// Repräsentiert einen konstanten Wert.
#[derive(Debug, Clone)]
pub enum IRConstant {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl PartialEq for IRConstant {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (IRConstant::String(a), IRConstant::String(b)) => a == b,
            (IRConstant::Number(a), IRConstant::Number(b)) => a.to_bits() == b.to_bits(),
            (IRConstant::Boolean(a), IRConstant::Boolean(b)) => a == b,
            (IRConstant::Null, IRConstant::Null) => true,
            _ => false,
        }
    }
}

impl Eq for IRConstant {}

impl std::hash::Hash for IRConstant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            IRConstant::String(s) => {
                0u8.hash(state);
                s.hash(state);
            }
            IRConstant::Number(f) => {
                1u8.hash(state);
                f.to_bits().hash(state);
            }
            IRConstant::Boolean(b) => {
                2u8.hash(state);
                b.hash(state);
            }
            IRConstant::Null => {
                3u8.hash(state);
            }
        }
    }
}

impl IRConstant {
    pub fn get_type(&self) -> IRType {
        match self {
            IRConstant::String(_) => IRType::String,
            IRConstant::Number(_) => IRType::Float,
            IRConstant::Boolean(_) => IRType::Bool,
            IRConstant::Null => IRType::Null,
        }
    }
}

/// IR-Variable (mit Ownership-Information)
/// 
/// Repräsentiert eine Variable mit Typ und Ownership-Information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IRVariable {
    pub name: String,
    pub id: VarId,
    pub ty: IRType,
    pub ownership: Ownership,
}

/// Ownership-Information
/// 
/// Definiert die Ownership-Semantik für eine Variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ownership {
    /// Variable besitzt den Wert (move semantics)
    Owned,
    /// Variable ist eine immutable Referenz (&T)
    Borrowed,
    /// Variable ist eine mutable Referenz (&mut T)
    BorrowedMut,
    /// Shared ownership (Arc<T> / Rc<T>)
    Shared,
    /// Copy-Semantik (primitive types)
    Copy,
}

/// Lifetime-Information
/// 
/// Definiert die Lifetime einer Referenz.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lifetime {
    pub id: LifetimeId,
    pub scope: ScopeId,
}

/// Lifetime-ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LifetimeId(pub usize);

impl LifetimeId {
    pub fn new(id: usize) -> Self {
        LifetimeId(id)
    }
}

/// Scope-ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub usize);

impl ScopeId {
    pub fn new(id: usize) -> Self {
        ScopeId(id)
    }
}

/// IR-Typ
/// 
/// Repräsentiert einen Typ in der IR.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IRType {
    Void,
    Bool,
    Int,
    Float,
    String,
    Null,
    Any,
    Pointer(Box<IRType>),
    Struct(String),
    Enum(String),
    Function {
        params: Vec<IRType>,
        return_type: Box<IRType>,
    },
    List(Box<IRType>),
    Map {
        key: Box<IRType>,
        value: Box<IRType>,
    },
    Tuple(Vec<IRType>),
    Optional(Box<IRType>),
    Result {
        ok: Box<IRType>,
        err: Box<IRType>,
    },
}

impl IRType {
    pub fn to_string(&self) -> String {
        match self {
            IRType::Void => "void".to_string(),
            IRType::Bool => "bool".to_string(),
            IRType::Int => "i64".to_string(),
            IRType::Float => "f64".to_string(),
            IRType::String => "string".to_string(),
            IRType::Null => "null".to_string(),
            IRType::Any => "any".to_string(),
            IRType::Pointer(inner) => format!("&{}", inner.to_string()),
            IRType::Struct(name) => name.clone(),
            IRType::Enum(name) => name.clone(),
            IRType::Function { params, return_type } => {
                let params_str = params.iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("fn({}) -> {}", params_str, return_type.to_string())
            }
            IRType::List(item) => format!("List<{}>", item.to_string()),
            IRType::Map { key, value } => {
                format!("Map<{}, {}>", key.to_string(), value.to_string())
            }
            IRType::Tuple(types) => {
                let types_str = types.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", types_str)
            }
            IRType::Optional(inner) => format!("{}?", inner.to_string()),
            IRType::Result { ok, err } => {
                format!("Result<{}, {}>", ok.to_string(), err.to_string())
            }
        }
    }
}

/// IR-Struct
/// 
/// Repräsentiert einen Struct in IR-Format.
#[derive(Debug, Clone)]
pub struct IRStruct {
    pub name: String,
    pub fields: Vec<IRStructField>,
    pub visibility: Visibility,
}

/// IR-Struct-Field
#[derive(Debug, Clone)]
pub struct IRStructField {
    pub name: String,
    pub ty: IRType,
    pub visibility: Visibility,
}

/// IR-Enum
/// 
/// Repräsentiert einen Enum in IR-Format.
#[derive(Debug, Clone)]
pub struct IREnum {
    pub name: String,
    pub variants: Vec<IREnumVariant>,
    pub visibility: Visibility,
}

/// IR-Enum-Variant
#[derive(Debug, Clone)]
pub struct IREnumVariant {
    pub name: String,
    pub data: Option<Vec<IRType>>,
}
