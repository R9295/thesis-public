use serde::{Deserialize, Serialize};
use thesis::Grammar;
#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum Statement {
    // Basic statements
    Expression(Expression),
    Assignment(AssignmentType, Expression, Expression),
    Return(Option<Expression>),
    Yield(Option<Expression>),
    YieldFrom(Expression),

    // Control pub structures
    Conditional(ConditionalType),
    #[recursive]
    Loop(LoopType),

    // Other statements
    Break(Option<Expression>),
    Continue(Option<Expression>),
    Goto(String),
    Label(String),

    // Declarations
    #[recursive]
    FunctionDeclaration(String, Vec<Parameter>, Body),
    #[recursive]
    ClassDeclaration(String, Option<String>, Vec<String>, Vec<ClassMember>),
    #[recursive]
    InterfaceDeclaration(String, Vec<String>, Vec<InterfaceMember>),
    #[recursive]
    TraitDeclaration(String, Vec<TraitMember>),
    NamespaceDeclaration(String),
    UseDeclaration(Vec<UseElement>),

    // Error handling
    #[recursive]
    Try(Body, Vec<CatchBlock>, Option<Body>),
    Throw(Expression),

    // Other
    Echo(Vec<Expression>),
    Print(Expression),
    Include(Expression),
    IncludeOnce(Expression),
    Require(Expression),
    RequireOnce(Expression),
    Unset(Vec<Expression>),
    Empty,

    // Grouping
    #[recursive]
    StatementGroup(Body),
}
#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum CastType {
    Int,
    Float,
    Null,
    String,
    Array,
    Object,
    Bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum Expression {
    #[recursive]
    ErrorSuppress(Box<Expression>),
    #[recursive]
    PropertyFetch(Box<Expression>, String),
    #[recursive]
    HashMap(Vec<(Expression, Box<Expression>)>),
    #[recursive]
    MethodCall(Box<Expression>, String, Vec<Expression>),
    #[recursive]
    Parenthesized(Box<Expression>),
    #[recursive]
    FunctionCall(Box<Expression>, Vec<Expression>),
    #[recursive]
    Eval(Box<Expression>),
    #[recursive]
    Isset(Vec<Expression>),
    Literal(LiteralValue),
    Variable(String),
    #[recursive]
    Arithmetic(Box<Expression>, ArithmeticOperator, Box<Expression>),
    #[recursive]
    Comparison(Box<Expression>, ComparisonOperator, Box<Expression>),
    #[recursive]
    Logical(Box<Expression>, LogicalOperator, Box<Expression>),
    #[recursive]
    Bitwise(Box<Expression>, BitwiseOperator, Box<Expression>),
    #[recursive]
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
    #[recursive]
    NullCoalescing(Box<Expression>, Box<Expression>),
    #[recursive]
    Instanceof(Box<Expression>, String),
    #[recursive]
    Clone(Box<Expression>),
    #[recursive]
    Array(Vec<(Option<Expression>, Expression)>),
    #[recursive]
    Closure(Vec<Parameter>, Body, Vec<ClosureUse>),
    #[recursive]
    ArrowFunction(Vec<Parameter>, Box<Expression>),
    #[recursive]
    Cast(CastType, Box<Expression>),
    #[recursive]
    New(Box<Expression>, Vec<Expression>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum AssignmentType {
    Simple,
    Compound(CompoundAssignmentOperator),
    ByReference,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum CompoundAssignmentOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Concatenate,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    NullCoalescing,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum ConditionalType {
    If(Expression, Body, Option<Vec<(Expression, Body)>>, Option<Body>),
    Switch(Expression, Vec<SwitchCase>),
    Match(Expression, Vec<MatchArm>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum LoopType {
    For(Vec<Expression>, Expression, Vec<Expression>, Body),
    Foreach(Expression, Expression, Option<Expression>, Body),
    While(Expression, Body),
    DoWhile(Body, Expression),
}

// Additional supporting types
#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub struct Parameter {
    pub name: String,
    pub default_value: Option<Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum ClassMember {
    Property(Visibility, String, Option<Expression>),
    Method(Visibility, String, Vec<Parameter>, Body),
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum InterfaceMember {
    Constant(String, Expression),
    MethodSignature(String, Vec<Parameter>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum TraitMember {
    Constant(String, Expression),
    Method(Visibility, String, Vec<Parameter>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub struct UseElement {
    pub name: String,
    pub alias: Option<String>, // Optional alias for "use ... as ..."
    pub is_namespace: bool, // Indicates if this is a namespace use
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub struct CatchBlock {
    pub exception_type: String,
    pub body: Body,
}


#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub struct ClosureUse {
    pub by_reference: bool,
    pub variable: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub struct SwitchCase {
    pub value: Option<Expression>,
    pub body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub struct MatchArm {
    pub conditions: Vec<Expression>,
    pub body: Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum Visibility {
    Public,
    Protected,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum LiteralValue {
    Integer(i64),
    Hex(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponentiation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    Identical,
    NotIdentical,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Spaceship,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum LogicalOperator {
    And,
    Or,
    Xor,
    Not,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum BitwiseOperator {
    And,
    Or,
    Xor,
    Not,
    LeftShift,
    RightShift,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub struct Body(pub Vec<Statement>);
