use crate::ast::*;
use std::fmt;
impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Statement::Expression(expr) => format!("{};", expr),
            Statement::Assignment(typ, left, right) => {
                let op = match typ {
                    AssignmentType::Simple => "=".to_string(),
                    AssignmentType::Compound(op) => op.to_string(),
                    AssignmentType::ByReference => "=&".to_string(),
                };
                format!("{} {} {};", left, op, right)
            }
            Statement::Return(expr) => match expr {
                Some(e) => format!("return {};", e),
                None => "return;".to_string(),
            },
            Statement::Yield(expr) => match expr {
                Some(e) => format!("yield {};", e),
                None => "yield;".to_string(),
            },
            Statement::YieldFrom(expr) => format!("yield from {};", expr),
            Statement::Conditional(cond) => cond.to_string(),
            Statement::Loop(loop_type) => loop_type.to_string(),
            Statement::Break(expr) => match expr {
                Some(e) => format!("break {};", e),
                None => "break;".to_string(),
            },
            Statement::Continue(expr) => match expr {
                Some(e) => format!("continue {};", e),
                None => "continue;".to_string(),
            },
                Statement::Empty => "".to_string(),
            Statement::Goto(label) => format!("goto {};", label),
            Statement::Label(label) => format!("{}:", label),
            Statement::FunctionDeclaration(name, params, body) => {
                let params_str = params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("function {}({}) {{\n{}\n}}\n", name, params_str, body)
            }
            Statement::ClassDeclaration(name, parent, interfaces, members) => {
                let parent_str = parent
                    .as_ref()
                    .map_or("".to_string(), |p| format!(" extends {}", p));
                let interfaces_str = if interfaces.is_empty() {
                    "".to_string()
                } else {
                    format!(" implements {}", interfaces.join(", "))
                };
                let members_str = members
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "class {}{}{}  {{\n{}\n}} }}\n",
                    name, parent_str, interfaces_str, members_str
                )
            }
            Statement::InterfaceDeclaration(name, extends, members) => {
                let extends_str = if extends.is_empty() {
                    "".to_string()
                } else {
                    format!(" extends {}", extends.join(", "))
                };
                let members_str = members
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "interface {}{} {{\n{}\n}}\n",
                    name, extends_str, members_str
                )
            }
            Statement::TraitDeclaration(name, members) => {
                let members_str = members
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("trait {}  \n{{\n {}\n }}", name, members_str)
            }
            Statement::NamespaceDeclaration(name) => format!("namespace {};", name),
            Statement::UseDeclaration(uses) => format!(
                "use {};",
                uses.iter()
                    .map(|u| u.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Statement::Try(try_block, catch_blocks, finally_block) => {
                let catch_str = catch_blocks
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                let finally_str = finally_block
                    .as_ref()
                    .map_or("".to_string(), |f| format!("finally\n {{ \n{}\n }}", f,));
                format!("try\n {{\n {} }} {} {}", try_block, catch_str, finally_str)
            }
            Statement::Throw(expr) => format!("throw {};", expr),
            Statement::Echo(exprs) => format!(
                "echo {};",
                exprs
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Statement::Print(expr) => format!("print {};", expr),
            Statement::Include(expr) => format!("include {};", expr),
            Statement::IncludeOnce(expr) => format!("include_once {};", expr),
            Statement::Require(expr) => format!("require {};", expr),
            Statement::RequireOnce(expr) => format!("require_once {};", expr),
            Statement::Unset(exprs) => format!(
                "unset({});",
                exprs
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Statement::StatementGroup(body) => format!("{}", body),
        };
        write!(f, "\n{}\n", code)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Expression::Literal(value) => value.to_string(),
            Expression::Variable(name) => format!("${}", name),
            Expression::Arithmetic(left, op, right) => format!("({} {} {})", left, op, right),
            Expression::Comparison(left, op, right) => format!("({} {} {})", left, op, right),
            Expression::Logical(left, op, right) => format!("({} {} {})", left, op, right),
            Expression::Bitwise(left, op, right) => format!("({} {} {})", left, op, right),
            Expression::Ternary(cond, then, else_) => format!("({} ? {} : {})", cond, then, else_),
            Expression::NullCoalescing(left, right) => format!("({} ?? {})", left, right),
            Expression::Instanceof(expr, class) => format!("{} instanceof {}", expr, class),
            Expression::Cast (cast_type, expression)  => {
                format!("({}) {}", cast_type, expression) // Casting representation (e.g., (int) $var)
            },
            Expression::Parenthesized(expr) => {
                format!("({})", expr) // Displaying a parenthesized expression
            }
            Expression::Eval(expr) => {
                format!("eval({})", expr) // Displaying an eval expression
            }
            Expression::Isset(expressions) => {
                let exprs: Vec<String> = expressions
                    .iter()
                    .map(|expr| format!("{}", expr)) // Formatting each expression in isset
                    .collect();
                format!("isset({})", exprs.join(", ")) // Joining expressions with commas for isset
            }
            Expression::FunctionCall(name, args) => format!(
                "{}({})",
                name,
                args.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expression::MethodCall(obj, method, args) => format!(
                "{}->{}({})",
                obj,
                method,
                args.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expression::ErrorSuppress(expr) => {
                format!("@{}", expr) // Error suppression is represented by @ followed by the expression
            }
            Expression::PropertyFetch(object, property) => {
                format!("{}.{}", object, property) // Accessing a property of an object
            }
            Expression::HashMap(pairs) => {
                let entries: Vec<String> = pairs
                    .iter()
                    .map(|(key, value)| format!("{}: {}", key, value)) // Formatting key-value pairs
                    .collect();
                format!("{{{}}}", entries.join(", ")) // Joining pairs with commas
            }
            Expression::New(class, args) => {
                let args_str = args
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("new {}({})", class, args_str)
            }
            Expression::Clone(expr) => format!("clone {}", expr),
            Expression::Array(items) => {
                let items_str = items
                    .iter()
                    .map(|(key, value)| match key {
                        Some(k) => format!("{} => {}", k, value),
                        None => value.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", items_str)
            }
            Expression::Closure(params, body, uses) => {
                let params_str = params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                let uses_str = if uses.is_empty() {
                    "".to_string()
                } else {
                    format!(
                        " use ({})",
                        uses.iter()
                            .map(|u| u.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                };
                format!(
                    "function({}){}
{{
{}
}}",
                    params_str, uses_str, body
                )
            }
            Expression::ArrowFunction(params, expr) => {
                let params_str = params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("fn({}) => {}", params_str, expr)
            }
        };
        write!(f, "{}", code)
    }
}

impl fmt::Display for AssignmentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssignmentType::Simple => write!(f, "="),
            AssignmentType::Compound(op) => write!(f, "{}", op),
            AssignmentType::ByReference => write!(f, "=&"),
        }
    }
}

impl fmt::Display for CompoundAssignmentOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            CompoundAssignmentOperator::Add => "+=",
            CompoundAssignmentOperator::Subtract => "-=",
            CompoundAssignmentOperator::Multiply => "*=",
            CompoundAssignmentOperator::Divide => "/=",
            CompoundAssignmentOperator::Modulo => "%=",
            CompoundAssignmentOperator::Concatenate => ".=",
            CompoundAssignmentOperator::BitwiseAnd => "&=",
            CompoundAssignmentOperator::BitwiseOr => "|=",
            CompoundAssignmentOperator::BitwiseXor => "^=",
            CompoundAssignmentOperator::LeftShift => "<<=",
            CompoundAssignmentOperator::RightShift => ">>=",
            CompoundAssignmentOperator::NullCoalescing => "??=",
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for ConditionalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            ConditionalType::If(condition, if_block, else_ifs, else_block) => {
                let if_str = format!("if ({}) {{\n{}\n}}", condition, if_block,);
                let else_if_str = else_ifs.as_ref().map_or("".to_string(), |ei| {
                    ei.iter()
                        .map(|ei| format!("elseif ({}) {{\n {}\n }}\n", ei.0, ei.1))
                        .collect::<Vec<_>>()
                        .join("\n")
                });
                let else_str = else_block
                    .as_ref()
                    .map_or("".to_string(), |eb| format!("else {{\n{}\n}}", eb));
                format!("{}\n{}\n{}", if_str, else_if_str, else_str)
            }
            ConditionalType::Switch(expr, cases) => {
                let cases_str = cases
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("switch ({}) {{\n{}\n}}", expr, cases_str)
            }
            ConditionalType::Match(expr, arms) => {
                let arms_str = arms
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(",\n");
                format!("match ({}) {{\n{}\n}}", expr, arms_str)
            }
        };
        write!(f, "{}", code)
    }
}

impl fmt::Display for LoopType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            LoopType::For(init, condition, increment, body) => {
                let init_str = init
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                let increment_str = increment
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "for ({};{};{}) {{\n{}\n}}",
                    init_str, condition, increment_str, body
                )
            }
            LoopType::Foreach(iterable, value, key, body) => {
                let key_str = key
                    .as_ref()
                    .map_or("".to_string(), |k| format!("{} => ", k));
                format!(
                    "foreach ({} as {}{}) {{\n{}\n}}",
                    iterable, key_str, value, body
                )
            }
            LoopType::While(condition, body) => {
                format!("while ({}) {{\n{}\n}}", condition, body)
            }
            LoopType::DoWhile(body, condition) => {
                format!("do {{\n{}\n}} while ({});", body, condition)
            }
        };
        write!(f, "{}\n", code)
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let default_value = self
            .default_value
            .as_ref()
            .map_or("".to_string(), |v| format!(" = {}", v));
        write!(f, "${}{}", self.name, default_value)
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl fmt::Display for ClassMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassMember::Property(visibility, name, default_value) => {
                let visibility_str = match visibility {
                    Visibility::Public => "public",
                    Visibility::Protected => "protected",
                    Visibility::Private => "private",
                };
                let default_str = default_value
                    .as_ref()
                    .map_or("".to_string(), |v| format!(" = {}", v));
                write!(f, "{} ${}{};", visibility_str, name, default_str)
            }
            ClassMember::Method(visibility, name, params, body) => {
                let visibility_str = match visibility {
                    Visibility::Public => "public",
                    Visibility::Protected => "protected",
                    Visibility::Private => "private",
                };
                let params_str = params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(
                    f,
                    "{} function {}({})\n{{\n{}\n}}\n",
                    visibility_str, name, params_str, body
                )
            }
        }
    }
}

impl fmt::Display for MatchArm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let conditions_str = self
            .conditions
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{} => {{ {} }}\n", conditions_str, self.body)
    }
}
impl fmt::Display for SwitchCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(expr) => write!(f, "case {}:\n{}\nbreak;", expr, self.body),
            None => write!(f, "default:\n{}", self.body),
        }
    }
}
impl fmt::Display for InterfaceMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterfaceMember::Constant(name, value) => {
                write!(f, "const {} = {};\n", name, value)
            }
            InterfaceMember::MethodSignature(name, params) => {
                let params_str = params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "public function {}({});\n", name, params_str)
            }
        }
    }
}

impl fmt::Display for ClosureUse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reference = if self.by_reference { "&" } else { "" };
        write!(f, "{}{}", reference, self.variable)
    }
}

impl fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            ComparisonOperator::Equal => "==",
            ComparisonOperator::NotEqual => "!=",
            ComparisonOperator::Identical => "===",
            ComparisonOperator::NotIdentical => "!==",
            ComparisonOperator::LessThan => "<",
            ComparisonOperator::GreaterThan => ">",
            ComparisonOperator::LessThanOrEqual => "<=",
            ComparisonOperator::GreaterThanOrEqual => ">=",
            ComparisonOperator::Spaceship => "<=>",
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for BitwiseOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            BitwiseOperator::And => "&",
            BitwiseOperator::Or => "|",
            BitwiseOperator::Xor => "^",
            BitwiseOperator::Not => "~",
            BitwiseOperator::LeftShift => "<<",
            BitwiseOperator::RightShift => ">>",
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            LogicalOperator::And => "&&",
            LogicalOperator::Or => "||",
            LogicalOperator::Xor => "xor",
            LogicalOperator::Not => "!",
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for ArithmeticOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            ArithmeticOperator::Add => "+",
            ArithmeticOperator::Subtract => "-",
            ArithmeticOperator::Multiply => "*",
            ArithmeticOperator::Divide => "/",
            ArithmeticOperator::Modulo => "%",
            ArithmeticOperator::Exponentiation => "**",
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Integer(i) => write!(f, "{}", i),
            LiteralValue::Hex(i) => write!(f, "0x{:X}", i),
            LiteralValue::Float(fl) => write!(f, "{}", fl),
            LiteralValue::String(s) => write!(f, "\"{}\"", s.replace("\"", "\\\"")),
            LiteralValue::Boolean(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            LiteralValue::Null => write!(f, "null"),
        }
    }
}

impl fmt::Display for TraitMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraitMember::Method(visibility, name, params) => {
                let params_str = params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{} function {}({});\n", visibility, name, params_str)
            }
            TraitMember::Constant(name, value) => {
                write!(f, "const {} = {};\n", name, value)
            }
        }
    }
}

impl fmt::Display for UseElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_namespace {
            write!(f, "use namespace {};", self.name)
        } else {
            match &self.alias {
                Some(alias) => write!(f, "use {} as {};\n", self.name, alias),
                None => write!(f, "use {};\n", self.name),
            }
        }
    }
}

impl fmt::Display for CatchBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "catch ({}) {{\n{}\n}}", self.exception_type, self.body)
    }
}
impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let visibility_str = match self {
            Visibility::Public => "public",
            Visibility::Protected => "protected",
            Visibility::Private => "private",
        };
        write!(f, "{}", visibility_str)
    }
}
impl fmt::Display for CastType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CastType::Int => write!(f, "int"),
            CastType::Float => write!(f, "float"),
            CastType::String => write!(f, "string"),
            CastType::Bool => write!(f, "bool"),
            CastType::Array => write!(f, "array"),
            CastType::Object => write!(f, "object"),
            CastType::Null => write!(f, "null"),
        }
    }
}

