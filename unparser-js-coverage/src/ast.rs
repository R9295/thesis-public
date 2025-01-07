use serde::{Deserialize, Serialize};
use thesis::{Grammar, ToNautilus};

#[derive(Debug, Clone, Serialize, Deserialize, Grammar, ToNautilus)]
pub enum Expression {
    Literal(LiteralValue),
    Variable(String),
    #[recursive]
    FunctionCall(String, Vec<Expression>),
    #[recursive]
    ObjectInitializer(Vec<(Expression, Expression)>),
    #[recursive]
    ArrayInitializer(Vec<Expression>),
    #[recursive]
    PropertyAccess(Box<Expression>, Box<Expression>),
    #[recursive]
    BinaryOperation(Box<Expression>, BinaryOperator, Box<Expression>),
    #[recursive]
    UnaryOperation(UnaryOperator, Box<Expression>),
    #[recursive]
    TernaryOperation(Box<Expression>, Box<Expression>, Box<Expression>),
    #[recursive]
    NullishCoalescing(Box<Expression>, Box<Expression>),
    #[recursive]
    NewExpression(
        Box<Expression>,      // The constructor to call
        Vec<Expression>, // Arguments for the constructor
    ),
    #[recursive]
    Spread(Box<Expression>),
    #[recursive]
    Parentheses(Box<Expression>),
    #[recursive]
    Await(Box<Expression>), // Await expression (for async functions)
    #[recursive]
    TemplateLiteral(Vec<Expression>), // Template literals
    #[recursive]
    BitwiseOperation {
        operator: BitwiseOperator,
        left: Box<Expression>,
        right: Box<Expression>, // Right operand is optional for NOT
    },
    ArrowFunction(Vec<String>, Body),
    #[recursive]
    OptionalChaining(Box<Expression>, Box<Expression>), // The property being accessed
    // Other expressions you might want to add:
    #[recursive]
    InstanceOf(Box<Expression>, String),
    #[recursive]
    Delete(Box<Expression>),
    #[recursive]
    RegExp(Box<Expression>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar, ToNautilus)]
pub enum LiteralValue {
    Number(i64),
    Float(i64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,
    Hex(i64),
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar, ToNautilus)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar, ToNautilus)]
pub enum UnaryOperator {
    Negate,
    Not,
    TypeOf,
    Void,
}
#[derive(Debug, Clone, Serialize, Deserialize, Grammar, ToNautilus)]
pub enum BitwiseOperator {
    And,
    Or,
    Xor,
    Not,
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar, ToNautilus)]
pub enum Statement {
    #[recursive]
    If(
        Expression,                      // The condition for the if statement
        Body,                                 // The statement to execute if the condition is true
        Option<Vec<(Expression, Body)>>, // Optional else if branches
        Option<Body>,                         // Optional else branch
    ),
    #[recursive]
    While(
        Expression, // The condition for the while loop
        Box<Statement>,  // The body of the loop
    ),
    #[recursive]
    For(
        Box<Statement>,  // Initialization statement (e.g., let i = 0)
        Expression, // Loop continuation condition
        Box<Statement>,  // Increment statement (e.g., i++)
        Body,  // The body of the loop
    ),
    #[recursive]
    DoWhile(
        Body,  // The body of the do-while loop
        Expression, // The condition for the do-while loop
    ),
    #[recursive]
    Try(
        Box<Statement>, // The block of code to try
        Option<Body>,   // Optional catch block
        Option<Body>,   // Optional finally block
    ),
    Throw(Expression), // Throw statement for exceptions
    Break,    // Break statement to exit loops
    Continue, // Continue statement to skip to the next iteration of a loop
    #[recursive]
    Switch(
        Expression,              // The expression to switch on
        Vec<(Expression, Body)>, // Case branches (expression and corresponding statement)
        Option<Body>,                 // Optional default case
    ),
    #[recursive]
    FunctionDeclaration(Function),

    #[recursive]
    ClassDeclaration(
        String,              // Name of the class
        Option<String>, // Optional superclass for inheritance
        Vec<Function>,       // Body of the class containing methods and properties
    ),

    Return(Option<Vec<Expression>>), // Return statement (with optional value)
    Yield(Option<Vec<Expression>>),  // Return statement (with optional value)
    YieldStar(Option<Vec<Expression>>), // Return statement (with optional value)

    Import(
        String,      // Module name to import from
        Vec<String>, // Variables to import from the module
    ),

    Export(
        Vec<String>, // Variables to export from the module
    ),

    VariableDeclaration(
        // Variable declaration (e.g., let x = 5)
        String,
        Option<Expression>,
    ),

    #[recursive]
    ForOf(
        // For-of loop for iterating over iterable objects (e.g., arrays)
        String,
        Expression,
        Box<Statement>,
    ),

    #[recursive]
    ForIn(
        // For-in loop for iterating over object properties
        String,
        Expression,
        Box<Statement>,
    ),

    #[recursive]
    Label(
        // Label statement for break/continue targeting
        String,
        Box<Statement>,
    ),
}

#[derive(Debug, Clone, Serialize, Deserialize, Grammar, ToNautilus)]
pub struct Body(pub Vec<Statement>);

#[derive(Debug, Clone, Serialize, Deserialize, Grammar, ToNautilus)]
pub struct Function {
    pub name: String,            // Name of the function
    pub parameters: Vec<String>, // Parameters for the function
    pub body: Body,              // Body of the function (which can be a block or other statements)
}
