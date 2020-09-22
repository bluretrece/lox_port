struct AstPrinter;

impl AstPrinter {
    fn new() -> Self {
        AstPrinter {}
    }
    fn print(&mut self, expr: &Vec<Box<Expr>>) -> String {
        let mut builder = String::new();

        for expression in expr {
            builder.push_str(expression.accept(self).as_str());
        }

        builder
    }

    fn parenthesize(&mut self, name: &str, expr: &Vec<&Box<Expr>>) -> String {
        let mut builder = String::from("(");

        builder.push_str(name);

        for expression in expr {
            builder.push_str(" ");
            builder.push_str(expression.accept(self).as_str());
        }

        builder.push_str(")");

        builder
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expression(
        &mut self,
        expr: &Expr,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> String {
        self.parenthesize(&operator.lexeme, &vec![left, right])
    }

    fn visit_group_expression(&mut self, expr: &Expr, content: &Box<Expr>) -> String {
        self.parenthesize("Group", &vec![content])
    }

    fn visit_literal_expression(&mut self, expr: &Expr, literal: &Literal) -> String {
        literal.to_string()
    }

    fn visit_unary_expression(
        &mut self,
        _expr: &Expr,
        operator: &Token,
        right: &Box<Expr>,
    ) -> String {
        self.parenthesize(&operator.lexeme, &vec![right])
    }
}

