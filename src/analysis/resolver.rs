use std::collections::{HashMap, LinkedList};
use crate::{common::{ast::NodeKind, error::{ChaoError, ErrorBase, ErrorSeverity}, token::TokenKind}, Node};
use super::irgen::{IrInst, IrValue};

static PLUS: &'static TokenKind = &TokenKind::Plus;
static MINUS: &'static TokenKind = &TokenKind::Minus;

fn build_type_table() -> HashMap<(Type, &'static TokenKind, Type), Type> {
    let mut t = HashMap::<(Type, &'static TokenKind, Type), Type>::new();
    
    // Integers
    t.insert((Type::Integer, PLUS, Type::Integer), Type::Integer);
    t.insert((Type::Integer, PLUS, Type::Void), Type::Integer);

    return t;
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Type {
    Integer,
    String,
    Void,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Variable {
    id: String,
    ty: Type,
    mutable: bool,
}

impl Variable {
    pub(self) fn new(id: String, ty: Type, mutable: bool) -> Variable {
        return Variable {
            id,
            ty,
            mutable,
        };
    }
}

struct Scope {
    variables: HashMap<String, Variable>,
}

impl Scope {
    pub(crate) fn new() -> Scope {
        return Scope {
            variables: HashMap::new(),
        };
    }

    pub(self) fn store(&mut self, id: String, v: Variable) {
        self.variables.insert(id, v);
    }

    pub(self) fn get(&self, id: &String) -> Option<&Variable> {
        return self.variables.get(id);
    }
}

pub(crate) struct Resolver {
    scopes: LinkedList<Scope>,
    types: HashMap<(Type, &'static TokenKind, Type), Type>,
    temps: Vec<IrValue>,
}

impl Resolver {
    pub(crate) fn new() -> Resolver {
        let mut scopes = LinkedList::<Scope>::new();
        scopes.push_front(Scope::new());
        
        return Resolver {
            scopes,
            types: build_type_table(), 
            temps: vec![],
        };
    }

    pub(crate) fn resolve<'a>(&'a mut self, ast: Vec<Node>) -> Result<(), Vec<ChaoError<'a>>> {
        let mut errs = Vec::<ChaoError>::new();

        for node in ast {
            self.resolve_node(node).unwrap_or_else(|e| {
                println!("got and error yeahhhh!");
                errs.push(e);
            });
        }

        return Err(errs);
    }

    fn resolve_node<'a>(&mut self, node: Node) -> Result<(), ChaoError<'a>> {
        match node.kind {
            NodeKind::StmtConstant { id, val } => self.def_const_id(id, *val),
            NodeKind::StmtVariable { id, val } => self.def_variable_id(id, *val),
            NodeKind::StmtExpression { expr } => self.resolve_node(*expr),
            NodeKind::ExprAssignment { id, op: _, val } => self.check_assignment(*id, *val),
            _ => todo!("resolve not assign or bind")
        }
    }
}

impl Resolver {
    fn def_const_id<'a>(&mut self, id: String, val: Node) -> Result<(), ChaoError<'a>> {
        let ty = self.type_res(val)?;
        let variable = Variable::new(id.clone(), ty, false);
        self.scopes.front_mut().unwrap().store(id, variable);
        return Ok(())
    }

    fn def_variable_id<'a>(&mut self, id: String, val: Node) -> Result<(), ChaoError<'a>> {
        let ty = self.type_res(val)?;
        let variable = Variable::new(id.clone(), ty, true);
        self.scopes.front_mut().unwrap().store(id, variable);
        return Ok(())
    }

    fn check_assignment<'a>(&mut self, variable: Node, val: Node) -> Result<(), ChaoError<'a>> {
        let line = val.line;
        let offset = val.offset;

        match variable.kind {
            NodeKind::LiteralIdent { id } => {
                let mutable: bool = self.scopes.front().unwrap().get(&id).is_some_and(|x| x.mutable);
                let var_ty: Option<Type> = self.scopes.front().unwrap().get(&id).and_then(|x| Some(x.ty.clone()));

                // if !mutable {
                //     todo!("reassign to a constant error");
                // }

                match var_ty {
                    Some(t) => {
                        let v_ty = self.type_res(val)?;
                        
                        // (todo) find a way to implement type coercion here and implicit casts
                        if v_ty != t {
                            let eb = ErrorBase::IncompatibleTypes { line, offset };
                            return Err(
                                ChaoError::new(eb, ErrorSeverity::Error, false, "cannot reassign '{}' to a different type")
                            );
                        }

                        return Ok(());
                    } 
                    None => {
                        let eb = ErrorBase::UnknownIdentifier { line, offset };
                        return Err(
                            ChaoError::new(eb, ErrorSeverity::Error, false, "identifier could not be found in this scope")
                        );
                    }
                }
            }
            _ => todo!("check assignment non-ident"),
        }
    }
}

impl Resolver {
    fn type_res<'a>(&mut self, val: Node) -> Result<Type, ChaoError<'a>> {
        match val.kind {
            NodeKind::LiteralStr { val: _ } => Ok(Type::String),
            NodeKind::LiteralInt { val: _ } => Ok(Type::Integer), 
            NodeKind::ExprBinary { lhs, op, rhs } => {
                let line = lhs.line;
                let offset = lhs.offset;

                let lhs_ty = self.type_res(*lhs)?;
                let rhs_ty = self.type_res(*rhs)?;
                let oper = match op {
                    TokenKind::Plus => PLUS,
                    _ => todo!()
                };

                match self.types.get(&(lhs_ty, oper, rhs_ty)) {
                    Some(result_ty) => return Ok(result_ty.clone()),
                    None => {
                        let eb = ErrorBase::IncompatibleTypes { line, offset };
                        return Err(
                            ChaoError::new(eb, ErrorSeverity::Error, false, "invalid types for this operator")
                        );
                    }
                }
            }
            _ => todo!("type res non binary or int")
        }
    }
}