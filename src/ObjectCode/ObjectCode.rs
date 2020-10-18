type Operator = String;
type Reg = i32;
type Module = Vec<Instr>;

pub enum Literal {
    Int(i32),
    Real(f32),
    Str(String),
    Bool(bool),
    EmptyList,
    Nil,
}

pub enum Instr {
    Regs((Operator, Vec<Reg>)),
    RegsList((Operator, Vec<Reg>, Literal)),
    Goto(i32),
    LoadFunc((Reg, i32, Vec<Instr>)),
    RegInt((Operator, Reg, Reg, i32)),
}

pub struct ObjCode {}

trait ObjectUnparser {
    fn module(ilist: Vec<Instr>) -> Vec<String>;
    fn literal(lit: Literal) -> Vec<String>;
}

impl ObjectUnparser for ObjCode {
    fn module(ilist: Vec<Instr>) -> Vec<String> {
        todo!()
    }

    fn literal(lit: Literal) -> Vec<String> {
        let mut str_list = Vec::new();
        let t;

        str_list.push(match lit {
            Literal::Int(n) => {
                t = n.to_string();
                t.as_str()
            }

            Literal::Real(n) => {
                t = n.to_string();
                t.as_str()
            }

            Literal::Bool(b) => {
                if b {
                    "true"
                } else {
                    "false"
                }
            }

            Literal::EmptyList => "emptylist",

            Literal::Nil => "nil",

            Literal::Str(s) => {
                // Converts the characters to their decimal representations
                let codes: String = s
                    .chars()
                    .map(|c| c.to_digit(10).unwrap().to_string())
                    .collect();

                // Generates a string with the generated decimals
                t = format!("string {len} {}", codes, len = s.chars().count());
                t.as_str()
            }
        });

        str_list.into_iter().map(|x| x.to_owned()).collect()
    }
}
