use super::*;

#[derive(Clone, Debug)]
pub struct FormatOptions {
    pub indentation: usize,
}

pub fn format_file(file: &File, options: &FormatOptions) -> String {
    let mut w = Writer::with_options(options);
    write_file(&mut w, file, options);
    w.finish()
}

fn write_file(out: &mut Writer, file: &File, options: &FormatOptions) {
    for item in &file.items {
        write_file_item(out, item, options);
        out.push_str("\n\n");
    }
}

fn write_file_item(out: &mut Writer, item: &FileItem, options: &FormatOptions) {
    match item {
        FileItem::Const(const_) => write_const_statement(out, const_, options),
    }
}

fn write_const_statement(out: &mut Writer, const_: &ConstStatement, options: &FormatOptions) {
    out.push_str("const ");
    out.push_str(&const_.name);
    out.push_str(" = ");
    write_expression(out, &const_.value, options);
    out.push_str(";");
}

fn write_expression(out: &mut Writer, expression: &Expression, options: &FormatOptions) {
    match expression {
        Expression::Literal(literal) => write_literal(out, literal),
        Expression::Identifier(identifier) => out.push_str(identifier),
        Expression::Call(call) => write_call(out, call, options),
        Expression::Function(function) => write_simple_function(out, function, options),
        Expression::BinaryOp(binary_op) => write_binary_op(out, binary_op, options),
        Expression::Dot(dot) => write_dot(out, dot, options),
        Expression::Ternary(ternary) => write_ternary(out, ternary, options),
        Expression::Array(array) => write_array(out, array, options),
        Expression::Object(object) => write_object(out, object, options),
    }
}

fn write_literal(out: &mut Writer, literal: &Literal) {
    match literal {
        Literal::Boolean(boolean) => out.push_str(match boolean {
            true => "true",
            false => "false",
        }),
        Literal::Number(number) => out.push_str(&number.to_string()),
        Literal::String { unescaped } => {
            out.push_str("\"");
            out.push_str(&escape_string_contents(unescaped));
            out.push_str("\"");
        }
    }
}

/// This does **not** enclose the string in quotes.
fn escape_string_contents(unescaped: &str) -> String {
    let mut out = String::new();
    for c in unescaped.chars() {
        if c.is_alphanumeric() || " `~!@#$%^&*()-_=+[{]}|;:,<.>/?".contains(c) {
            out.push(c);
        } else if c == '"' {
            out.push_str("\\\"");
        } else if c == '\'' {
            out.push_str("\\'");
        } else if c == '\\' {
            out.push_str("\\\\");
        } else if c == '\n' {
            out.push_str("\\n");
        } else {
            out.push_str(&format!("\\u{:04x}", u32::from(c)));
        }
    }
    out
}

fn write_call(out: &mut Writer, call: &Call, options: &FormatOptions) {
    if let Expression::Identifier(_) = call.callee {
        write_expression(out, &call.callee, options);
    } else {
        out.push_str("(");
        write_expression(out, &call.callee, options);
        out.push_str(")");
    }

    out.push_str("(");
    for (i, arg) in call.args.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        write_expression(out, arg, options);
    }
    out.push_str(")");
}

fn write_simple_function(out: &mut Writer, function: &Function, options: &FormatOptions) {
    out.push_str("function ");
    out.push_str(&function.name);
    out.push_str("(");
    for (i, param) in function.params.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(param);
    }
    out.push_str(") {\n");

    out.increase_indentation_level();
    out.indent();
    for statement in &function.body {
        write_function_statement(out, statement, options);
        out.push_str("\n");
    }
    out.push_str(";\n");
    out.decrease_indentation_level();

    out.indent();
    out.push_str("}");
}

fn write_function_statement(
    out: &mut Writer,
    statement: &FunctionStatement,
    options: &FormatOptions,
) {
    match statement {
        FunctionStatement::Const(const_) => write_const_statement(out, const_, options),
        FunctionStatement::Return(return_value) => {
            write_return_statement(out, return_value, options)
        }
        FunctionStatement::Throw(throw_value) => write_throw_statement(out, throw_value, options),
    }
}

fn write_return_statement(out: &mut Writer, return_value: &Expression, options: &FormatOptions) {
    out.push_str("return ");
    write_expression(out, return_value, options);
    out.push_str(";");
}

fn write_throw_statement(out: &mut Writer, throw_value: &Expression, options: &FormatOptions) {
    out.push_str("throw ");
    write_expression(out, throw_value, options);
    out.push_str(";");
}

fn write_binary_op(out: &mut Writer, binary_op: &BinaryOp, options: &FormatOptions) {
    match binary_op.op {
        BinaryOpKind::TripleEqual => {
            out.push_str("(");
            write_expression(out, &binary_op.left, options);
            out.push_str(" === ");
            write_expression(out, &binary_op.right, options);
            out.push_str(")");
        }
        BinaryOpKind::Index => {
            write_expression(out, &binary_op.left, options);
            out.push_str("[");
            write_expression(out, &binary_op.right, options);
            out.push_str("]");
        }
    }
}

fn write_dot(out: &mut Writer, dot: &Dot, options: &FormatOptions) {
    if let Expression::Identifier(_) = &dot.left {
        write_expression(out, &dot.left, options);
        out.push_str(".");
        out.push_str(&dot.right);
    } else {
        out.push_str("(");
        write_expression(out, &dot.left, options);
        out.push_str(").");
        out.push_str(&dot.right);
    }
}

fn write_ternary(out: &mut Writer, ternary: &Ternary, options: &FormatOptions) {
    out.push_str("(");
    write_expression(out, &ternary.condition, options);
    out.push_str("\n");

    out.increase_indentation_level();
    out.indent();
    out.push_str("? ");
    write_expression(out, &ternary.true_body, options);

    out.push_str("\n");
    out.indent();
    out.push_str(": ");
    write_expression(out, &ternary.false_body, options);
    out.push_str("\n");
    out.decrease_indentation_level();
    out.indent();
    out.push_str(")");
}

fn write_array(out: &mut Writer, array: &Array, options: &FormatOptions) {
    out.push_str("[");
    for (i, item) in array.items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        write_expression(out, item, options);
    }
    out.push_str("]");
}

fn write_object(out: &mut Writer, object: &Object, options: &FormatOptions) {
    out.push_str("{ ");
    for (i, entry) in object.entries.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str("\"");
        out.push_str(&escape_string_contents(&entry.key));
        out.push_str("\": ");
        write_expression(out, &entry.value, options);
    }
    out.push_str(" }");
}

use writer::Writer;
mod writer {
    use super::*;

    /// A thin wrapper around `String` that also tracks indentation state.
    #[derive(Clone, Debug)]
    pub struct Writer<'a> {
        raw: String,
        indentation_level: usize,
        options: &'a FormatOptions,
    }

    impl<'a> Writer<'a> {
        pub fn with_options(options: &'a FormatOptions) -> Writer<'a> {
            Self {
                raw: String::new(),
                indentation_level: 0,
                options,
            }
        }
    }

    impl<'a> std::ops::Deref for Writer<'a> {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.raw
        }
    }

    impl<'a> std::ops::DerefMut for Writer<'a> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.raw
        }
    }

    impl Writer<'_> {
        pub fn increase_indentation_level(&mut self) {
            self.indentation_level += self.options.indentation;
        }

        /// Panics if this would result in the indentation level being negative.
        pub fn decrease_indentation_level(&mut self) {
            self.indentation_level = self
                .indentation_level
                .checked_sub(self.options.indentation)
                .expect("Tried to decrease indentation level below 0.");
        }

        pub fn indent(&mut self) {
            for _ in 0..self.indentation_level {
                self.push(' ');
            }
        }

        pub fn finish(self) -> String {
            self.raw
        }
    }
}
