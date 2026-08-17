
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    TypeAlias(TypeAlias),
    Module(Module),
    Use(Use),
    Trait(Trait),
    Impl(Impl),
    TopLevelCode(ExpressionStatement), // Top-level expression statements like init();
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub decorators: Vec<Decorator>,
    pub visibility: Visibility,
    pub name: String,
    pub type_params: Vec<GenericParam>, // Generic parameters with constraints
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub is_async: bool,
    pub is_const: bool,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Decorator {
    pub name: String,
    pub args: Vec<DecoratorArg>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DecoratorArg {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
    Named { name: String, value: Box<DecoratorArg> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub default: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    If(IfStatement),
    For(ForStatement),
    While(WhileStatement),
    Match(MatchStatement),
    Throw(ThrowStatement),
    Break(BreakStatement),
    Try(TryStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowStatement {
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct TryStatement {
    pub try_block: Block,
    pub catch_blocks: Vec<CatchBlock>,
    pub finally_block: Option<Block>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchBlock {
    pub error_var: Option<String>,
    pub error_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub name: String,
    pub var_type: Option<Type>,
    pub value: Expression,
    pub mutable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub variable: String,
    pub iterable: Expression,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchStatement {
    pub expression: Expression,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>, // Pattern guard: `if condition`
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal(Literal),
    Identifier(String),
    Tuple(Vec<Pattern>),
    Struct { name: String, fields: Vec<(String, Pattern)> },
    EnumVariant { name: String, data: Option<Vec<Pattern>> },
    Range { start: Box<Expression>, end: Box<Expression>, inclusive: bool }, // 0..10 or 0..=10
    Wildcard, // _
    Or(Vec<Pattern>), // pattern1 | pattern2
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        args: Vec<Expression>,
    },
    Member {
        object: Box<Expression>,
        member: String,
    },
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
    },
    Block(Block),
    Await {
        expr: Box<Expression>,
    },
    StructLiteral {
        name: String,
        fields: Vec<(String, Expression)>,
    },
    MapLiteral(Vec<(String, Expression)>),
    ListLiteral(Vec<Expression>),
    GenericConstructor {
        name: String,
        type_params: Vec<Type>,
        args: Vec<Expression>,
    },
    Lambda {
        params: Vec<Parameter>,
        return_type: Option<Type>,
        body: Box<Expression>, // Can be a Block or a single expression
    },
    Assignment {
        target: Box<Expression>,
        value: Box<Expression>,
    },
    FormatString {
        parts: Vec<FormatStringPart>,
    },
    /// LLM-Call: @llm.analyze(text)
    LLMCall {
        method: String, // "analyze", "summarize", "extract", etc.
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormatStringPart {
    Text(String),
    Expression(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %
    Eq,       // ==
    NotEq,    // !=
    Lt,       // <
    Gt,       // >
    LtEq,     // <=
    GtEq,     // >=
    And,      // &&
    Or,       // ||
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not,   // !
    Minus, // -
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub name: String,
    pub type_params: Vec<String>,
    pub fields: Vec<StructField>,
    pub visibility: Visibility,
    pub decorators: Vec<Decorator>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: String,
    pub field_type: Type,
    pub visibility: Visibility,
    pub decorators: Vec<Decorator>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub visibility: Visibility,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub data: Option<Vec<Type>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAlias {
    pub name: String,
    pub aliased_type: Type,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: String,
    pub items: Vec<Item>,
    pub visibility: Visibility,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Use {
    pub path: Vec<String>,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Trait {
    pub name: String,
    pub type_params: Vec<String>,
    pub methods: Vec<TraitMethod>,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitMethod {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Impl {
    pub trait_name: String,
    pub for_type: Type,
    pub type_params: Vec<String>,
    pub methods: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericParam {
    pub name: String,
    pub constraints: Vec<GenericConstraint>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenericConstraint {
    Trait(String),
    Multiple(Vec<String>), // T: Trait1 & Trait2
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // Basic types
    String,
    Number,
    Boolean,
    Void,
    Null,
    Any,
    
    // Named types
    Named(String),
    
    // Generic types
    Generic {
        name: String,
        params: Vec<Type>,
    },
    
    // Function types
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    
    // Collection types
    List(Box<Type>),
    Map {
        key: Box<Type>,
        value: Box<Type>,
    },
    
    // Tuple types
    Tuple(Vec<Type>),
    
    // Optional types
    Optional(Box<Type>),
    
    // Result type
    Result {
        ok: Box<Type>,
        err: Box<Type>,
    },
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::String => "string".to_string(),
            Type::Number => "number".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::Void => "void".to_string(),
            Type::Null => "null".to_string(),
            Type::Any => "any".to_string(),
            Type::Named(name) => name.clone(),
            Type::Generic { name, params } => {
                let params_str = params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}<{}>", name, params_str)
            }
            Type::Function { params, return_type } => {
                let params_str = params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("fn({}) -> {}", params_str, return_type.to_string())
            }
            Type::List(item_type) => format!("List<{}>", item_type.to_string()),
            Type::Map { key, value } => {
                format!("Map<{}, {}>", key.to_string(), value.to_string())
            }
            Type::Tuple(types) => {
                let types_str = types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", types_str)
            }
            Type::Optional(inner) => format!("{}?", inner.to_string()),
            Type::Result { ok, err } => {
                format!("Result<{}, {}>", ok.to_string(), err.to_string())
            }
        }
    }
}
