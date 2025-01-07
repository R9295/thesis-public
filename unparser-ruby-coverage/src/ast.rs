use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thesis::Grammar;

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum Statement {
    Assignment(Expression, Expression),
    GlobalAssignment(String, Expression),
    #[recursive]
    Conditional(Expression, Block, Option<Block>),
    #[recursive]
    Loop(LoopType, Block),
    MethodCall(Option<Expression>, String, Vec<Expression>),
    Return(Option<Expression>),
    Break(Option<Expression>),
    Next(Option<Expression>),
    Redo,
    Retry,
    #[recursive]
    Begin(Block, Vec<(Option<Expression>, Block)>, Option<Block>),
    #[recursive]
    ClassDefinition(String, Option<Expression>, Block),
    #[recursive]
    ModuleDefinition(String, Block),
    #[recursive]
    MethodDefinition(String, Vec<String>, Block),
    Yield(Vec<Expression>),
    Raise(Option<Expression>),
    #[recursive]
    Rescue(Option<Expression>, Block),
    #[recursive]
    Ensure(Block),
    #[recursive]
    Case(Expression, Vec<(Expression, Block)>, Option<Block>),
    #[recursive]
    Unless(Expression, Block, Option<Block>),
    #[recursive]
    Until(Expression, Block),
    #[recursive]
    For(String, Expression, Block),
    #[recursive]
    Lambda(Vec<String>, Block),
    Alias(Expression, Expression),
    Undef(Vec<Expression>),
    Include(Expression),
    Extend(Expression),
    Require(Expression),
    RequireRelative(Expression),
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum Expression {
    #[recursive]
    Super(Option<Vec<Expression>>),
    Self_,
    Literal(LiteralValue),
    Variable(String),
    GlobalVariable(String),
    ConstantAccess(Vec<String>),
    #[recursive]
    MethodCall(Option<Box<Expression>>, String, Vec<Expression>),
    #[recursive]
    BinaryOperation(Box<Expression>, BinaryOperator, Box<Expression>),
    #[recursive]
    UnaryOperation(UnaryOperator, Box<Expression>),
    #[recursive]
    Assignment(Box<Expression>, Box<Expression>),
    #[recursive]
    Conditional(Box<Expression>, Box<Expression>, Option<Box<Expression>>),
    #[recursive]
    Block(Vec<Expression>),
    #[recursive]
    Lambda(Vec<String>, Box<Expression>),
    #[recursive]
    YieldExpression(Vec<Expression>),
    #[recursive]
    Range(Box<Expression>, Box<Expression>, bool),
    #[recursive]
    ArrayLiteral(Vec<Expression>),
    #[recursive]
    HashLiteral(Vec<(Expression, Expression)>),
    Interpolation(Vec<InterpolationPart>),
    #[recursive]
    FlipFlop(Box<Expression>, Box<Expression>, FlipFlopType),
    #[recursive]
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
    #[recursive]
    IndexAccess(Box<Expression>, Box<Expression>),
    #[recursive]
    Splat(Box<Expression>),
    #[recursive]
    DoubleSplat(Box<Expression>),
    #[recursive]
    Defined(Box<Expression>),
    #[recursive]
    Pin(Box<Expression>),
    Undef(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum LoopType {
    While,
    Until,
    For,
    Loop,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Symbol(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum UnaryOperator {
    Negative,
    Not,
    BitwiseNot,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum InterpolationPart {
    String(String),
    Expression(Box<Expression>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar)]
pub enum FlipFlopType {
    Inclusive,
    Exclusive,
}
