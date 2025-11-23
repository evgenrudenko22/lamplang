use super::ast::{Expr, TypedArgument, Stmt};
use super::value::{ValueType};

pub struct CCodeGenerator {
    input: Stmt,
}

impl CCodeGenerator {
    pub fn new(input: Stmt) -> Self {
        Self {
            input
        }
    }

    pub fn generate(&mut self) -> String {
        let mut main = String::from("#include \"area.h\"\n");

        if let Stmt::Block(stmts) = self.input.clone() {
            self.generate_outer(&mut main, stmts.clone());
            let code = self.generate_c_block_of_code(Stmt::Block(stmts.clone()));
            main += format!("void main() {{area_start();{}area_end();}}", code).as_str();
        }

        main
    }

    fn generate_outer(&mut self, code: &mut String, stmts: Vec<Stmt>) {
        self.generate_uses(code, stmts.clone());
        self.generate_structs(code, stmts.clone());
        self.generate_functions(code, stmts.clone());
    }

    fn generate_uses(&mut self, code: &mut String, stmts: Vec<Stmt>) {
        for stmt in stmts.clone() {
            if let Stmt::Use(module) = stmt.clone() {
                code.push_str(format!("#include \"{}.h\"\n", module).as_str());
            }
        }
    }

    fn generate_functions(&mut self, code: &mut String, stmts: Vec<Stmt>) {
        for stmt in stmts.clone() {
            if let Stmt::FunctionDef(name, args, body, return_type) = stmt.clone() {
                code.push_str(self.convert_to_c_function(name.clone(), args, body, return_type).as_str());
            }
        }
    }

    fn generate_structs(&mut self, code: &mut String, stmts: Vec<Stmt>) {
        for stmt in stmts.clone() {
            if let Stmt::Struct(name, fields) = stmt.clone() {
                let c_args_vec: Vec<String> = fields.iter().map(
                    |a| format!("{} {}", self.convert_to_c_type(a.typ), a.name)
                ).collect();
                let c_args_str = c_args_vec.join("; ") + "; ";
                code.push_str(format!("typedef struct {{{}}} {};", c_args_str, name).as_str());
            }
        }
    }

    fn generate_c_block_of_code(&mut self, statement: Stmt) -> String {
        let mut code = String::new();
        if let Stmt::Block(stmts) = statement.clone() {
            for stmt in stmts {
                code += self.generate_c_statement(stmt).as_str();
            }
        } else {
            code += self.generate_c_statement(statement).as_str();
        }

        code
    }

    fn generate_c_stmt_or_block(&mut self, stmt: Stmt) -> String {
        if let Stmt::Block(_) = stmt.clone() {
            self.generate_c_block_of_code(stmt)
        } else {
            self.generate_c_statement(stmt)
        }
    }

    fn generate_c_statement(&mut self, statement: Stmt) -> String {
        match statement {
            Stmt::Assign(name, value) => {
                format!("{} = {};", name, self.generate_c_expression(*value))
            },
            Stmt::VarDef(name, value, v_type) => {
                format!("{} {} = {};", self.convert_to_c_type(v_type), name, self.generate_c_expression(*value))
            }
            Stmt::Return(value) => {
                format!("return {};", self.generate_c_expression(*value))
            }
            Stmt::If(cond, body, else_body) => {
                let mut result: String = String::from(format!("if ({}) {{area_start();{}area_end();}}",
                                                              self.generate_c_expression(*cond),
                                                              self.generate_c_stmt_or_block(*body)).as_str());

                if let Some(else_body) = *else_body {
                    result += format!("else {{area_start();{}area_end();}}", self.generate_c_stmt_or_block(else_body)).as_str();
                }
                result
            },
            Stmt::While(cond, body) => {
                format!("while ({}) {{area_start();{}area_end();}}",
                                     self.generate_c_expression(*cond),
                                     self.generate_c_stmt_or_block(*body))
            }
            Stmt::Function(expr) => {
                format!("{};", self.generate_c_expression(*expr).to_string())
            }
            Stmt::Struct(_, _) => "".to_string(),
            Stmt::FunctionDef(_, _, _, _) | Stmt::Use(_) => "".to_string(),
            _ => "\n".to_string()
        }
    }

    fn generate_c_expression(&mut self, expression: Expr) -> String {
        match expression {
            Expr::Value(v) => {
                v.to_string()
            }
            Expr::VarUse(name) => {
                name.to_string()
            }
            Expr::Binary(op, left, right) => {
                format!("{}{}{}", self.generate_c_expression(*left), op, self.generate_c_expression(*right))
            }
            Expr::Condition(op, left, right) => {
                format!("{}{}{}", self.generate_c_expression(*left), op, self.generate_c_expression(*right))
            }
            Expr::Unary(op, operand) => {
                format!("{}({})", op, self.generate_c_expression(*operand))
            }
            Expr::Functional(name, args) => {
                let c_args_vec: Vec<String> = args.iter().map(|arg| self.generate_c_expression(arg.clone())).collect();
                let c_args = c_args_vec.join(", ");
                format!("{}({})", name, c_args)
            }
            Expr::New(_, _) => unimplemented!()
        }
    }

    fn convert_to_c_function(&mut self, name: String, args: Vec<TypedArgument>, body: Box<Stmt>, return_type: ValueType) -> String {
        let c_type: String = self.convert_to_c_type(return_type);
        let c_args_vec: Vec<String> = args.iter().map(
            |a| format!("{} {}", self.convert_to_c_type(a.typ), a.name)
        ).collect();
        let c_args = c_args_vec.join(",");

        format!("{} {} ({}) {{area_start();{}area_end();}}", c_type, name, c_args, self.generate_c_block_of_code(*body))
    }

    fn convert_to_c_type(&mut self, typ: ValueType) -> String {
        match typ {
            ValueType::Number => "double".to_string(),
            ValueType::String => "char*".to_string(),
            ValueType::Unit => "void".to_string(),
        }
    }
}