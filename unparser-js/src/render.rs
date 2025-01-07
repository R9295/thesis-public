use crate::ast::{
    BinaryOperator, BitwiseOperator, Body, Expression, Function, LiteralValue, Statement,
    UnaryOperator,
};
use std::fmt::{self, Display};
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(value) => match value {
                LiteralValue::Number(n) => write!(f, "{}", n),
                LiteralValue::Hex(n) => write!(f, "0x{:X}", n),
                LiteralValue::Float(n) => write!(f, "{}", n),
                LiteralValue::String(s) => write!(f, "\"{}\"", s),
                LiteralValue::Boolean(b) => write!(f, "{}", b),
                LiteralValue::Null => write!(f, "null"),
                LiteralValue::Undefined => write!(f, "undefined"),
            },
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::FunctionCall(function_name, arguments) => {
                let mut ret = format!("{}(", function_name);
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        ret.push_str(", ");
                    }
                    ret.push_str(&format!("{}", arg));
                }
                ret.push_str(")");
                write!(f, "{}", ret)
            }
            Expression::ObjectInitializer(properties) => {
                let mut ret = "{".to_string();
                for (i, (key, value)) in properties.iter().enumerate() {
                    if i > 0 {
                        ret.push_str(", ");
                    }
                    ret.push_str(&format!("\"{}\": {}", key, value));
                }
                write!(f, "{} }}", ret)
            }
            Expression::ArrayInitializer(elements) => {
                let mut ret = "[".to_string();
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        ret.push_str(", ");
                    }
                    ret.push_str(&format!("{}", element));
                }
                write!(f, "{}]", ret)
            }
            Expression::PropertyAccess(object, property) => {
                write!(f, "{}.{}", object, property)
            }
            Expression::BinaryOperation(left, operator, right) => {
                write!(f, "({} {} {})", left, operator, right)
            }
            Expression::UnaryOperation(operator, operand) => {
                write!(f, "{}({})", operator, operand)
            }
            Expression::TernaryOperation(condition, true_expr, false_expr) => {
                write!(f, "({} ? {} : {})", condition, true_expr, false_expr)
            }
            Expression::NullishCoalescing(left, right) => {
                write!(f, "{} ?? {}", left, right)
            }
            Expression::Spread(expr) => {
                write!(f, "...{}", expr)
            }
            Expression::Parentheses(expr) => {
                write!(f, "({})", expr)
            }
            Expression::NewExpression(constructor, arguments) => {
                write!(f, "new {}", constructor)?;
                if !arguments.is_empty() {
                    write!(
                        f,
                        "({})",
                        arguments
                            .iter()
                            .map(|arg| arg.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )?;
                }
                Ok(())
            }
            Expression::Await(expr) => {
                write!(f, "await {}", expr)
            }
            Expression::TemplateLiteral(expressions) => {
                let mut result = String::new();
                for (i, expr) in expressions.iter().enumerate() {
                    if i > 0 {
                        result.push_str("${}");
                    }
                    result.push_str(&format!("{}", expr));
                }
                write!(f, "`{}`", result)
            }
            Expression::BitwiseOperation {
                operator,
                left,
                right,
            } => match operator {
                BitwiseOperator::And => write!(f, "({} & {})", left, right.as_ref()),
                BitwiseOperator::Or => write!(f, "({} | {})", left, right.as_ref()),
                BitwiseOperator::Xor => write!(f, "({} ^ {})", left, right.as_ref()),
                BitwiseOperator::Not => write!(f, "(~{})", left),
            },
            Expression::ArrowFunction(parameters, body) => {
                let params = if parameters.len() == 1 {
                    format!("{}", parameters[0]) // No parentheses needed for single parameter
                } else {
                    format!("({})", parameters.join(", ")) // Parentheses for multiple parameters
                };

                write!(f, "{} => {{\n{}\n}}", params, body)
            }
            Expression::OptionalChaining(object, property) => {
                write!(f, "{}?.{}", object, property)
            }
            Expression::InstanceOf(left, right) => {
                write!(f, "{} instanceof {}", left, right)
            }
            Expression::Delete(expr) => {
                write!(f, "delete {}", expr)
            }
            Expression::RegExp(pattern) => {
                write!(f, "/{}/", pattern) // Regular expression literal
            }
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Modulo => write!(f, "%"),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::StrictEqual => write!(f, "==="),
            BinaryOperator::StrictNotEqual => write!(f, "!=="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessThanOrEqual => write!(f, "<="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterThanOrEqual => write!(f, ">="),
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Negate => write!(f, "-"),
            UnaryOperator::Not => write!(f, "!"),
            UnaryOperator::TypeOf => write!(f, "typeof"),
            UnaryOperator::Void => write!(f, "void"),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::If(condition, then_branch, else_if_branches, else_branch) => {
                let mut ret = format!("if ({}) {{\n{}\n}}\n", condition, then_branch);
                if let Some(else_if_branches) = else_if_branches {
                    for (else_if_condition, else_if_branch) in else_if_branches {
                        ret.push_str(&format!(
                            " else if ({}) {{\n{}\n}}\n",
                            else_if_condition, else_if_branch
                        ));
                    }
                }
                if let Some(else_branch) = else_branch {
                    ret.push_str(&format!(" else {{\n{}\n}}\n", else_branch));
                }
                write!(f, "{}\n", ret)
            }
            Statement::While(condition, body) => {
                write!(f, "while ({}) {{\n{}\n}}\n", condition, body)
            }
            Statement::For(initialization, condition, increment, body) => {
                write!(
                    f,
                    "for ({}, {} ; {}) {{\n{}\n}}\n",
                    initialization, condition, increment, body
                )
            }
            Statement::DoWhile(body, condition) => {
                write!(f, "do {{\n{}\n}} while ({});\n", body, condition)
            }
            Statement::Try(try_block, catch_block, finally_block) => {
                let mut ret = format!("try {{\n{}\n}}\n", try_block);
                if let Some(catch_stmt) = catch_block {
                    ret.push_str(&format!(" catch {{\n{}\n}}\n", catch_stmt));
                }
                if let Some(finally_stmt) = finally_block {
                    ret.push_str(&format!(" finally {{\n{}\n}}\n", finally_stmt));
                }
                write!(f, "{}\n", ret)
            }
            Statement::Throw(expression) => {
                write!(f, "throw {};\n", expression)
            }
            Statement::Break => write!(f, "break;\n"),
            Statement::Continue => write!(f, "continue;\n"),
            Statement::Switch(condition, cases, default_case) => {
                let mut ret = format!("switch ({}) {{\n", condition);
                for (case_condition, case_body) in cases {
                    ret.push_str(&format!(
                        "  case {}:\n  {{ {} }}\n",
                        case_condition, case_body
                    ));
                }
                if let Some(default_body) = default_case {
                    ret.push_str(&format!("  default:\n  {{ {} }}\n", default_body));
                }
                write!(f, "{} }}\n", ret)
            }
            Statement::FunctionDeclaration(func) => {
                write!(f, "{}", func)
            }
            Statement::ClassDeclaration(name, super_class, body) => {
                if let Some(super_class_expr) = super_class {
                    write!(f, "class {} extends {} {{\n", name, super_class_expr)?;
                } else {
                    write!(f, "class {} {{\n", name)?;
                }

                for method in body {
                    write!(f, "  {}\n", method)?;
                }

                write!(f, "}}\n")
            }
            Statement::Import(module_name, variables) => {
                let vars = variables.join(", ");
                write!(f, "import {{{}}} from \"{}\";", vars, module_name)
            }
            Statement::Export(variables) => {
                let vars = variables.join(", ");
                write!(f, "export {{{}}};\n", vars)
            }
            Statement::VariableDeclaration(is_re, variable_name, value) => match is_re {
                true => write!(f, "let {} = {};\n", variable_name, value),
                false => write!(f, "{} = {};\n", variable_name ,value),
            },
            Statement::ForOf(variable_name, iterable, body) => {
                write!(
                    f,
                    "for (const {} of {}) {{\n{}\n}}\n",
                    variable_name, iterable, body
                )
            }
            Statement::ForIn(variable_name, object_expression, body) => {
                write!(
                    f,
                    "for (const {} in {}) {{\n{}\n}}\n",
                    variable_name, object_expression, body
                )
            }
            Statement::Label(label_name, body) => {
                write!(f, "{}:\n{};\n", label_name, body)
            }
            Statement::Return(exprs) => {
                let mut ret = format!("return ");
                if let Some(exprs) = exprs {
                    if !exprs.is_empty() {
                        for (i, expr) in exprs.iter().enumerate() {
                            if i > 0 {
                                ret.push_str(", ");
                            }
                            ret.push_str(&format!("{}", expr));
                        }
                    }
                }
                write!(f, "{};\n", ret);
                Ok(())
            }
            Statement::Yield(exprs) => {
                let mut ret = format!("yield ");
                if let Some(exprs) = exprs {
                    if !exprs.is_empty() {
                        for (i, expr) in exprs.iter().enumerate() {
                            if i > 0 {
                                ret.push_str(", ");
                            }
                            ret.push_str(&format!("{}", expr));
                        }
                    }
                }
                write!(f, "{};\n", ret);
                Ok(())
            }
            Statement::YieldStar(exprs) => {
                let mut ret = format!("yield* ");
                if let Some(exprs) = exprs {
                    if !exprs.is_empty() {
                        for (i, expr) in exprs.iter().enumerate() {
                            if i > 0 {
                                ret.push_str(", ");
                            }
                            ret.push_str(&format!("{}", expr));
                        }
                    }
                }
                write!(f, "{};\n", ret);
                Ok(())
            }
        }
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ret = self
            .0
            .iter()
            .map(|i| format!("{}\n", i))
            .collect::<String>();
        write!(f, "{}\n", ret)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = self.parameters.join(", ");
        write!(
            f,
            "function {}({}) {{\n{}\n}}",
            self.name, params, self.body
        )
    }
}
