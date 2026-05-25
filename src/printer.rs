use std::fmt::Write;

use crate::parser::Node;

#[derive(Default)]
pub struct Printer {
    output: String,
    indent_level: usize,
    indent: &'static str,
    line_end: &'static str,
    first_block: bool,
    in_jsx: bool,
}

impl Printer {
    pub fn new(indent: &'static str, line_end: &'static str) -> Self {
        Self {
            indent,
            line_end,
            first_block: true,
            in_jsx: false,
            ..Default::default()
        }
    }

    pub fn code(self) -> String {
        self.output
    }

    fn write(&mut self, s: &str) {
        let _ = self.output.write_str(s);
    }

    fn newline(&mut self) {
        self.write(self.line_end);
    }

    fn indent(&mut self) {
        self.write(&self.indent.repeat(self.indent_level));
    }

    fn indent_inc(&mut self) {
        self.indent_level += 1;
    }

    fn indent_dec(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    pub fn print<'a>(&mut self, node: &Node<'a>) {
        match node {
            Node::Function((name, params, body)) => self.print_function(name, params, body),
            Node::Block(statements) => self.print_block(statements),
            Node::If((test, consequent, alternate)) => self.print_if(test, consequent, alternate),
            Node::While((test, body)) => self.print_while(test, body),
            Node::Do((body, test)) => self.print_do_while(body, test),
            Node::Switch((discriminant, cases)) => self.print_switch(discriminant, cases),
            Node::For((init, body)) => self.print_for(init, body),
            Node::With((object, body)) => self.print_with(object, body),
            Node::Declaration((kind, declarations)) => self.print_declaration(kind, declarations),
            Node::Return(expr) => self.print_return(expr),
            Node::Throw(expr) => self.print_throw(expr),
            Node::Continue(label) => self.print_continue(label),
            Node::Break(label) => self.print_break(label),
            Node::Blank => self.print_blank(),
            Node::Str(s) => self.print_str(s),
            Node::Interpolation(s) => self.print_interpolation(s),
            Node::Ident(name) => self.print_ident(name),
            Node::Double(n) => self.print_double(*n),
            Node::Octal(n) => self.print_octal(*n),
            Node::Hexadecimal(n) => self.print_hexadecimal(*n),
            Node::BinaryNum(n) => self.print_binary_number(*n),
            Node::Idents(names) => self.print_idents(names),
            Node::Regex((pattern, flags)) => self.print_regex(pattern, flags),
            Node::List(elements) => self.print_list(elements),
            Node::Object(properties) => self.print_object(properties),
            Node::Paren(expr) => self.print_paren(expr),
            Node::Closure((params, body)) => self.print_closure(params, body),
            Node::Shorthand((key, params, body)) => self.print_shorthand(key, params, body),
            Node::Setter(body) => self.print_setter(body),
            Node::Getter(body) => self.print_getter(body),
            Node::Static(body) => self.print_static(body),
            Node::Generator((name, params, body)) => self.print_generator(name, params, body),
            Node::Class((name, super_class, body)) => self.print_class(name, super_class, body),
            Node::Field((key, value)) => self.print_field(key, value),
            Node::Unary(op, expr) => self.print_unary(op, expr),
            Node::Binary(op, left, right) => self.print_binary(op, left, right),
            Node::Ternary(test, consequent, alternate) => {
                self.print_ternary(test, consequent, alternate)
            }
            Node::Import((spec)) => self.print_import(spec),
            Node::Export(expr) => self.print_export(expr),
            Node::Default(expr) => self.print_default(expr),
            Node::Try((block, catch, finally)) => self.print_try(block, catch, finally),
            Node::Catch((param, body)) => self.print_catch(param, body),
            Node::Label((label, stmt)) => self.print_label(label, stmt),
            Node::Case((test, consequent)) => self.print_case(test, consequent),
            Node::Args(args) => self.print_args(args),
            Node::ListPattern(elements) => self.print_list_pattern(elements),
            Node::ObjPattern(properties) => self.print_obj_pattern(properties),
            Node::Splat(expr) => self.print_splat(expr),
            Node::KeyValue((key, value)) => self.print_key_value(key, value),
            Node::Param((param, default)) => self.print_param(param, default),
            Node::ForTrio(trio) => self.print_for_trio(trio),
            Node::ForOf((left, right)) => self.print_for_of(left, right),
            Node::ForIn((left, right)) => self.print_for_in(left, right),
            Node::Variable((kind, expr)) => self.print_variable(kind, expr),
            Node::JSXElement((tag, attrs, children)) => {
                self.print_jsx_element(tag, attrs, children)
            }
            Node::JSXFragment(children) => {
                self.print_jsx_fragment(children)
            }
            Node::JSXSpreadAttribute(expr) => {
                self.print_jsx_spread_attribute(expr)
            }
            Node::JSXMemberExpression((obj, prop)) => {
                self.print_jsx_member_expression(obj, prop)
            }
            Node::JSXNamespacedName((prefix, local)) => {
                self.print_jsx_namespaced_name(prefix, local)
            }
            Node::JSXText(text) => {
                self.print_jsx_text(text)
            }
            _ => {}
        }
    }

    fn print_function<'a>(&mut self, name: &Option<&str>, params: &[Node<'a>], body: &Node<'a>) {
        self.write("function");
        if let Some(n) = name {
            self.write(" ");
            self.write(n);
        }
        self.print_params(params);
        self.write(" ");

        // 强制函数体使用大括号包裹，即使是第一个block
        let prev_first_block = self.first_block;
        self.first_block = false;
        self.print(body);
        self.first_block = prev_first_block;
    }

    fn print_block<'a>(&mut self, statements: &[Node<'a>]) {
        let is_first = self.first_block;

        if !is_first {
            self.write("{");
        }

        let prev_indent = self.indent_level;

        if !is_first {
            self.indent_inc();
        } else {
            self.indent_level = 0;
        }

        if !statements.is_empty() {
            self.newline();
            for stmt in statements {
                if !is_first {
                    self.indent();
                }
                self.print(stmt);
                self.newline();
            }
        }

        if is_first {
            self.indent_level = prev_indent;
        } else {
            self.indent_dec();
            self.indent();
            self.write("}");
        }

        if is_first {
            self.first_block = false;
        }
    }

    fn print_jsx_element<'a>(&mut self, tag: &Node<'a>, attrs: &[Node<'a>], children: &[Node<'a>]) {
        let prev_in_jsx = self.in_jsx;
        self.in_jsx = true;

        self.write("<");
        self.print(tag);

        for attr in attrs {
            self.write(" ");
            self.print(attr);
        }

        if children.is_empty() {
            self.write(" />");
        } else {
            self.write(">");
            
            // 判断是否需要格式化输出
            let needs_formatting = children.len() > 1 || 
                children.iter().any(|c| matches!(c, Node::JSXElement(_) | Node::JSXFragment(_)));
            
            if needs_formatting {
                self.indent_inc();
                for child in children {
                    self.newline();
                    self.indent();
                    self.print(child);
                }
                self.indent_dec();
                self.newline();
                self.indent();
            } else {
                for child in children {
                    self.print(child);
                }
            }
            
            self.write("</");
            self.print(tag);
            self.write(">");
        }

        self.in_jsx = prev_in_jsx;
    }

    fn print_jsx_fragment<'a>(&mut self, children: &[Node<'a>]) {
        let prev_in_jsx = self.in_jsx;
        self.in_jsx = true;

        self.write("<>");
        
        let needs_formatting = children.len() > 1 || 
            children.iter().any(|c| matches!(c, Node::JSXElement(_) | Node::JSXFragment(_)));
        
        if needs_formatting {
            self.indent_inc();
            for child in children {
                self.newline();
                self.indent();
                self.print(child);
            }
            self.indent_dec();
            self.newline();
            self.indent();
        } else {
            for child in children {
                self.print(child);
            }
        }
        
        self.write("</>");

        self.in_jsx = prev_in_jsx;
    }

    fn print_jsx_spread_attribute<'a>(&mut self, expr: &Node<'a>) {
        self.write("{...");
        self.print(expr);
        self.write("}");
    }

    fn print_jsx_member_expression<'a>(&mut self, obj: &Node<'a>, prop: &Node<'a>) {
        self.print(obj);
        self.write(".");
        self.print(prop);
    }

    fn print_jsx_namespaced_name<'a>(&mut self, prefix: &Node<'a>, local: &Node<'a>) {
        self.print(prefix);
        self.write(":");
        self.print(local);
    }

    fn print_jsx_text(&mut self, text: &str) {
        // JSX 文本直接输出，不需要引号
        self.write(text);
    }

    fn print_key_value<'a>(&mut self, key: &Node<'a>, value: &Node<'a>) {
        self.print(key);

        if self.in_jsx {
            self.write("=");
        } else {
            self.write(": ");
        }

        self.print(value);
    }

    fn print_str(&mut self, s: &str) {
        // 在 JSX 上下文中，字符串作为属性值需要引号
        // 但作为文本节点时不应该有引号（由 print_jsx_text 处理）
        self.write(&format!("\"{}\"", s.replace("\\", "\\\\").replace("\"", "\\\"")));
    }

    fn print_binary<'a>(&mut self, op: &str, left: &Node<'a>, right: &Node<'a>) {
        if op == "(" {
            self.print(left);
            self.write("(");
            if let Node::Args(args) = right {
                self.print_comma_separated(args);
            } else {
                self.print(right);
            }
            self.write(")");
            return;
        }

        if op == "." {
            self.print(left);
            self.write(".");
            self.print(right);
            return;
        }

        self.print(left);
        self.write(&format!(" {} ", op));
        self.print(right);
    }

    fn print_if<'a>(
        &mut self,
        test: &Node<'a>,
        consequent: &Node<'a>,
        alternate: &Option<Box<Node<'a>>>,
    ) {
        self.write("if (");
        self.print(test);
        self.write(") ");
        self.print(consequent);

        if let Some(alt) = alternate {
            self.write(" else ");
            self.print(alt);
        }
    }

    fn print_while<'a>(&mut self, test: &Node<'a>, body: &Node<'a>) {
        self.write("while (");
        self.print(test);
        self.write(") ");
        self.print(body);
    }

    fn print_do_while<'a>(&mut self, body: &Node<'a>, test: &Node<'a>) {
        self.write("do ");
        self.print(body);
        self.write(" while (");
        self.print(test);
        self.write(");");
    }

    fn print_switch<'a>(&mut self, discriminant: &Node<'a>, cases: &Node<'a>) {
        self.write("switch (");
        self.print(discriminant);
        self.write(") {");
        self.indent_inc();
        self.newline();
        self.print(cases);
        self.indent_dec();
        self.indent();
        self.write("}");
    }

    fn print_for<'a>(&mut self, init: &Node<'a>, body: &Node<'a>) {
        self.write("for (");
        self.print(init);
        self.write(") ");
        self.print(body);
    }

    fn print_with<'a>(&mut self, object: &Node<'a>, body: &Node<'a>) {
        self.write("with (");
        self.print(object);
        self.write(") ");
        self.print(body);
    }

    fn print_declaration<'a>(&mut self, kind: &str, declarations: &[Node<'a>]) {
        self.write(kind);
        self.write(" ");
        self.print_comma_separated(declarations);
        self.write(";");
    }

    fn print_return<'a>(&mut self, expr: &Option<Box<Node<'a>>>) {
        self.write("return");
        if let Some(e) = expr {
            self.write(" ");
            self.print(e);
        }
        self.write(";");
    }

    fn print_throw<'a>(&mut self, expr: &Node<'a>) {
        self.write("throw ");
        self.print(expr);
        self.write(";");
    }

    fn print_continue<'a>(&mut self, label: &Option<Box<Node<'a>>>) {
        self.write("continue");
        if let Some(l) = label {
            self.write(" ");
            self.print(l);
        }
        self.write(";");
    }

    fn print_break<'a>(&mut self, label: &Option<Box<Node<'a>>>) {
        self.write("break");
        if let Some(l) = label {
            self.write(" ");
            self.print(l);
        }
        self.write(";");
    }

    fn print_blank(&mut self) {
        self.write(";");
    }

    fn print_interpolation(&mut self, s: &str) {
        self.write("`");
        self.write(s);
        self.write("`");
    }

    fn print_ident(&mut self, name: &str) {
        self.write(name);
    }

    fn print_double(&mut self, n: f64) {
        self.write(&n.to_string());
    }

    fn print_octal(&mut self, n: u64) {
        self.write(&format!("0o{}", n));
    }

    fn print_hexadecimal(&mut self, n: u64) {
        self.write(&format!("0x{:x}", n));
    }

    fn print_binary_number(&mut self, n: u64) {
        self.write(&format!("0b{:b}", n));
    }

    fn print_idents(&mut self, names: &[&str]) {
        self.write("(");
        self.write(&names.join(", "));
        self.write(")");
    }

    fn print_regex(&mut self, pattern: &str, flags: &Option<&str>) {
        self.write("/");
        self.write(pattern);
        self.write("/");
        if let Some(f) = flags {
            self.write(f);
        }
    }

    fn print_list<'a>(&mut self, elements: &[Option<Node<'a>>]) {
        self.write("[");
        let mut first = true;
        for elem in elements {
            if !first {
                self.write(", ");
            }
            first = false;
            match elem {
                Some(e) => self.print(e),
                None => self.write(""),
            }
        }
        self.write("]");
    }

    fn print_object<'a>(&mut self, properties: &[Node<'a>]) {
        self.write("{");
        self.indent_inc();
        if !properties.is_empty() {
            self.newline();
            let mut first = true;
            for prop in properties {
                if !first {
                    self.write(",");
                    self.newline();
                }
                first = false;
                self.indent();
                self.print(prop);
            }
            self.newline();
        }
        self.indent_dec();
        self.indent();
        self.write("}");
    }

    fn print_paren<'a>(&mut self, expr: &Node<'a>) {
        match expr {
            Node::Binary(_, _, _) | Node::Ternary(_, _, _) => {
                self.write("(");
                self.print(expr);
                self.write(")");
            }
            _ => self.print(expr),
        }
    }

    fn print_closure<'a>(&mut self, params: &[Node<'a>], body: &Node<'a>) {
        self.print_params(params);
        self.write(" => ");
        self.print(body);
    }

    fn print_shorthand<'a>(&mut self, key: &Node<'a>, params: &[Node<'a>], body: &Node<'a>) {
        self.print(key);
        self.print_params(params);
        self.write(" ");
        self.print(body);
    }

    fn print_setter<'a>(&mut self, body: &Node<'a>) {
        self.write("set ");
        self.print(body);
    }

    fn print_getter<'a>(&mut self, body: &Node<'a>) {
        self.write("get ");
        self.print(body);
    }

    fn print_static<'a>(&mut self, body: &Node<'a>) {
        self.write("static ");
        self.print(body);
    }

    fn print_generator<'a>(&mut self, name: &Option<&str>, params: &[Node<'a>], body: &Node<'a>) {
        self.write("function*");
        if let Some(n) = name {
            self.write(" ");
            self.write(n);
        }
        self.print_params(params);
        self.write(" ");
        self.print(body);
    }

    fn print_class<'a>(
        &mut self,
        name: &Option<&str>,
        super_class: &Option<Box<Node<'a>>>,
        body: &[Node<'a>],
    ) {
        self.write("class");
        if let Some(n) = name {
            self.write(" ");
            self.write(n);
        }
        if let Some(s) = super_class {
            self.write(" extends ");
            self.print(s);
        }
        self.write(" {");
        self.indent_inc();
        self.newline();
        for member in body {
            self.indent();
            self.print(member);
            self.newline();
        }
        self.indent_dec();
        self.indent();
        self.write("}");
    }

    fn print_field<'a>(&mut self, key: &Node<'a>, value: &Node<'a>) {
        self.print(key);
        self.write(" = ");
        self.print(value);
    }

    fn print_unary<'a>(&mut self, op: &str, expr: &Node<'a>) {
        self.write(op);
        self.print(expr);
    }

    fn print_ternary<'a>(&mut self, test: &Node<'a>, consequent: &Node<'a>, alternate: &Node<'a>) {
        self.print(test);
        self.write(" ? ");
        self.print(consequent);
        self.write(" : ");
        self.print(alternate);
    }

    fn print_import<'a>(&mut self, spec: &(Option<Box<Node<'a>>>, Box<Node<'a>>)) {
        self.write("import ");
        if let Some(default_spec) = &spec.0 {
            self.print(default_spec);
            self.write(" from ");
        } else {
            self.print(&spec.1);
            self.write(" from ");
        }

        self.print(&spec.1);
        self.write(";");
    }

    fn print_export<'a>(&mut self, expr: &Node<'a>) {
        self.write("export ");
        self.print(expr);
    }

    fn print_default<'a>(&mut self, expr: &Node<'a>) {
        self.write("default ");
        self.print(expr);
    }

    fn print_try<'a>(
        &mut self,
        block: &Node<'a>,
        catch: &Option<Box<Node<'a>>>,
        finally: &Option<Box<Node<'a>>>,
    ) {
        self.write("try ");
        self.print(block);
        if let Some(c) = catch {
            self.write(" catch ");
            self.print(c);
        }
        if let Some(f) = finally {
            self.write(" finally ");
            self.print(f);
        }
    }

    fn print_catch<'a>(&mut self, param: &Option<Box<Node<'a>>>, body: &Node<'a>) {
        self.write("(");
        if let Some(p) = param {
            self.print(p);
        }
        self.write(") ");
        self.print(body);
    }

    fn print_label<'a>(&mut self, label: &Node<'a>, stmt: &Node<'a>) {
        self.print(label);
        self.write(": ");
        self.print(stmt);
    }

    fn print_case<'a>(&mut self, test: &Node<'a>, consequent: &Node<'a>) {
        self.write("case ");
        self.print(test);
        self.write(": ");
        self.print(consequent);
    }

    fn print_args<'a>(&mut self, args: &[Node<'a>]) {
        self.write("(");
        self.print_comma_separated(args);
        self.write(")");
    }

    fn print_list_pattern<'a>(&mut self, elements: &[Option<Node<'a>>]) {
        self.write("[");
        let mut first = true;
        for elem in elements {
            if !first {
                self.write(", ");
            }
            first = false;
            match elem {
                Some(e) => self.print(e),
                None => self.write(""),
            }
        }
        self.write("]");
    }

    fn print_obj_pattern<'a>(&mut self, properties: &[Node<'a>]) {
        self.write("{");
        self.print_comma_separated(properties);
        self.write("}");
    }

    fn print_splat<'a>(&mut self, expr: &Node<'a>) {
        self.write("...");
        self.print(expr);
    }

    fn print_param<'a>(&mut self, param: &Node<'a>, default: &Option<Box<Node<'a>>>) {
        self.print(param);
        if let Some(d) = default {
            self.write(" = ");
            self.print(d);
        }
    }

    fn print_for_trio<'a>(&mut self, trio: &[Option<Node<'a>>]) {
        let parts: Vec<_> = trio
            .iter()
            .map(|p| match p {
                Some(n) => {
                    let mut printer = Printer::new("  ", "\n");
                    printer.print(n);
                    printer.code()
                }
                None => String::new(),
            })
            .collect();
        self.write(&parts.join("; "));
    }

    fn print_for_of<'a>(&mut self, left: &Node<'a>, right: &Node<'a>) {
        self.print(left);
        self.write(" of ");
        self.print(right);
    }

    fn print_for_in<'a>(&mut self, left: &Node<'a>, right: &Node<'a>) {
        self.print(left);
        self.write(" in ");
        self.print(right);
    }

    fn print_variable<'a>(&mut self, kind: &Option<&str>, expr: &Node<'a>) {
        if let Some(k) = kind {
            self.write(k);
            self.write(" ");
        }
        self.print(expr);
    }

    fn print_comma_separated<'a>(&mut self, nodes: &[Node<'a>]) {
        let mut first = true;
        for node in nodes {
            if !first {
                self.write(", ");
            }
            first = false;
            self.print(node);
        }
    }

    fn print_params<'a>(&mut self, params: &[Node<'a>]) {
        self.write("(");
        self.print_comma_separated(params);
        self.write(")");
    }
}
