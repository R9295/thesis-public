use crate::ast::*;
use std::fmt;

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Statement::Assignment(lhs, rhs) => format!("{} = {}", lhs, rhs),
            Statement::GlobalAssignment(name, value) => format!("{} = {}", name, value),
            Statement::Conditional(cond, then_branch, else_branch) => {
                let else_part = else_branch
                    .as_ref()
                    .map_or(String::new(), |e| format!(" else\n{}", e));
                format!("if {}\n{}{}\nend", cond, then_branch, else_part)
            }
            Statement::Loop(loop_type, block) => {
                let keyword = match loop_type {
                    LoopType::While => "while",
                    LoopType::Until => "until",
                    LoopType::For => "for",
                    LoopType::Loop => "loop do",
                };
                format!("{}\n{}\nend", keyword, block)
            }
            Statement::MethodCall(receiver, method, args) => {
                let receiver_str = receiver
                    .as_ref()
                    .map_or(String::new(), |r| format!("{}.", r));
                let args_str = args
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}{}({})", receiver_str, method, args_str)
            }
            Statement::Return(value) => format!(
                "return {}",
                value.as_ref().map_or(String::new(), |v| v.to_string())
            ),
            Statement::Break(value) => format!(
                "break {}",
                value.as_ref().map_or(String::new(), |v| v.to_string())
            ),
            Statement::Next(value) => format!(
                "next {}",
                value.as_ref().map_or(String::new(), |v| v.to_string())
            ),
            Statement::Redo => "redo".to_string(),
            Statement::Retry => "retry".to_string(),
            Statement::Begin(block, rescues, ensure) => {
                let rescues_str = rescues
                    .iter()
                    .map(|(e, b)| {
                        let exception = e.as_ref().map_or(String::new(), |ex| format!(" {}", ex));
                        format!("rescue{}\n{}", exception, b)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                let ensure_str = ensure
                    .as_ref()
                    .map_or(String::new(), |e| format!("ensure\n{}", e));
                format!("begin\n{}\n{}\n{}\nend", block, rescues_str, ensure_str)
            }
            Statement::ClassDefinition(name, superclass, block) => {
                let superclass_str = superclass
                    .as_ref()
                    .map_or(String::new(), |s| format!(" < {}", s));
                format!("class {}{}\n{}\nend", name, superclass_str, block)
            }
            Statement::ModuleDefinition(name, block) => {
                format!("module {}\n{}\nend", name, block)
            }
            Statement::MethodDefinition(name, params, block) => {
                let params_str = params.join(", ");
                format!("def {}({})\n{}\nend", name, params_str, block)
            }
            Statement::Yield(args) => {
                let args_str = args
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("yield {}", args_str)
            }
            Statement::Raise(exception) => format!(
                "raise {}",
                exception.as_ref().map_or(String::new(), |e| e.to_string())
            ),
            Statement::Rescue(exception, block) => {
                let exception_str = exception
                    .as_ref()
                    .map_or(String::new(), |e| format!(" {}", e));
                format!("rescue{}\n{}", exception_str, block)
            }
            Statement::Ensure(block) => format!("ensure\n{}", block),
            Statement::Case(expr, whens, else_block) => {
                let whens_str = whens
                    .iter()
                    .map(|(cond, block)| format!("when {}\n{}", cond, block))
                    .collect::<Vec<_>>()
                    .join("\n");
                let else_str = else_block
                    .as_ref()
                    .map_or(String::new(), |e| format!("else\n{}", e));
                format!("case {}\n{}\n{}\nend", expr, whens_str, else_str)
            }
            Statement::Unless(cond, block, else_block) => {
                let else_str = else_block
                    .as_ref()
                    .map_or(String::new(), |e| format!("else\n{}", e));
                format!("unless {}\n{}\n{}\nend", cond, block, else_str)
            }
            Statement::Until(cond, block) => format!("until {}\n{}\nend", cond, block),
            Statement::For(var, collection, block) => {
                format!("for {} in {}\n{}\nend", var, collection, block)
            }
            Statement::Lambda(params, block) => {
                let params_str = params.join(", ");
                format!("lambda {{ |{}| {} }}", params_str, block)
            }
            Statement::Alias(new, old) => format!("alias {} {}", new, old),
            Statement::Undef(methods) => format!(
                "undef {}",
                methods
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Statement::Include(module_name) => format!("include {}", module_name),
            Statement::Extend(module_name) => format!("extend {}", module_name),
            Statement::Require(file) => format!("require {}", file),
            Statement::RequireRelative(file) => format!("require_relative {}", file),
        };

        write!(f, "{}\n", output)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Expression::Literal(value) => value.to_string(),
            Expression::Variable(name) => name.clone(),
            Expression::GlobalVariable(name) => format!("${}", name),
            Expression::ConstantAccess(parts) => parts.join("::"),
            Expression::MethodCall(receiver, method, args) => {
                let receiver_str = receiver
                    .as_ref()
                    .map_or(String::new(), |r| format!("{}.", r));
                let args_str = args
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}{}({})", receiver_str, method, args_str)
            }
            Expression::Undef(methods) => {
                format!(
                    "undef {}",
                    methods
                        .iter()
                        .map(|m| m.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Expression::Super(args) => {
                if args.is_none() {
                    "super".to_string()
                } else {
                    let args_str = args
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("super({})", args_str)
                }
            }
            Expression::BinaryOperation(lhs, op, rhs) => format!("({} {} {})", lhs, op, rhs),
            Expression::UnaryOperation(op, expr) => format!("{}({})", op, expr),
            Expression::Assignment(lhs, rhs) => format!("{} = {}", lhs, rhs),
            Expression::Conditional(cond, then_expr, else_expr) => {
                let else_part = else_expr
                    .as_ref()
                    .map_or(String::new(), |e| format!(" : {}", e));
                format!("{} ? {}{}", cond, then_expr, else_part)
            }
            Expression::Block(exprs) => format!(
                "{{ {} }}",
                exprs
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
            Expression::Lambda(params, body) => {
                let params_str = params.join(", ");
                format!("lambda {{ |{}| {} }}", params_str, body)
            }
            Expression::YieldExpression(args) => {
                let args_str = args
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("yield {}", args_str)
            }
            Expression::Range(start, end, inclusive) => {
                let op = if *inclusive { ".." } else { "..." };
                format!("{}{}{}", start, op, end)
            }
            Expression::ArrayLiteral(elements) => {
                let elements_str = elements
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", elements_str)
            }
            Expression::HashLiteral(pairs) => {
                let pairs_str = pairs
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", pairs_str)
            }
            Expression::Self_ => "self".to_string(),
            Expression::Interpolation(parts) => {
                let parts_str = parts
                    .iter()
                    .map(|p| match p {
                        InterpolationPart::String(s) => s.clone(),
                        InterpolationPart::Expression(e) => format!("#{{{}}}", e),
                    })
                    .collect::<Vec<_>>()
                    .join("");
                format!("\"{}\"", parts_str)
            }
            Expression::FlipFlop(start, end, flip_flop_type) => {
                let op = match flip_flop_type {
                    FlipFlopType::Inclusive => "..",
                    FlipFlopType::Exclusive => "...",
                };
                format!("{}{}{}", start, op, end)
            }
            Expression::Ternary(cond, then_expr, else_expr) => {
                format!("{} ? {} : {}", cond, then_expr, else_expr)
            }
            Expression::IndexAccess(expr, index) => format!("{}[{}]", expr, index),
            Expression::Splat(expr) => format!("*{}", expr),
            Expression::DoubleSplat(expr) => format!("**{}", expr),
            Expression::Defined(expr) => format!("defined?({})", expr),
            Expression::Pin(expr) => format!("^{}", expr),
        };

        write!(f, "{}", output)
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::LessThan => "<",
            BinaryOperator::LessThanOrEqual => "<=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::GreaterThanOrEqual => ">=",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
            BinaryOperator::BitwiseAnd => "&",
            BinaryOperator::BitwiseOr => "|",
            BinaryOperator::BitwiseXor => "^",
            BinaryOperator::LeftShift => "<<",
            BinaryOperator::RightShift => ">>",
        };
        write!(f, "{}", output)
    }
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            LiteralValue::Integer(i) => i.to_string(),
            LiteralValue::Float(fl) => fl.to_string(),
            LiteralValue::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")), // Escape quotes in strings
            LiteralValue::Symbol(s) => format!(":{}", s),
            LiteralValue::Boolean(b) => b.to_string(),
            LiteralValue::Nil => "nil".to_string(),
        };
        write!(f, "{}", output)
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            UnaryOperator::Negative => "-",
            UnaryOperator::Not => "!",
            UnaryOperator::BitwiseNot => "~",
        };
        write!(f, "{}", output)
    }
}

impl fmt::Display for InterpolationPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpolationPart::String(s) => write!(f, "{}", s),
            InterpolationPart::Expression(expr) => write!(f, "#{{{}}}", expr),
        }
    }
}

impl fmt::Display for FlipFlopType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            FlipFlopType::Inclusive => "..",
            FlipFlopType::Exclusive => "...",
        };
        write!(f, "{}", output)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Join the statements in the block with newlines and format them
        let statements_str = self
            .0
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        // Output the formatted block
        write!(f, "{}\n", statements_str)
    }
}
