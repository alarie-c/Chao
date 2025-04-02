use crate::common::{ ast::{ Node, NodeKind }, token::TokenKind };

#[derive(Debug)]
pub(super) enum IrValue {
    Temp(usize),
    Identifier(String),
    ConstInt(i32),
}

#[derive(Debug)]
pub(crate) enum IrInst {
    Bind {
        id: String,
        val: IrValue,
    },
    Store {
        id: IrValue,
        val: IrValue,
    }
}

pub(crate) struct IrCompiler {
    temps: usize,
}

impl IrCompiler {
    pub(crate) fn new() -> IrCompiler {
        return IrCompiler { temps: 0 };
    }

    pub(crate) fn compile<'a>(&mut self, ast: Vec<Node<'a>>) -> Vec<IrInst> {
        let mut ir: Vec<IrInst> = vec![];

        for node in ast {
            match node.kind {
                NodeKind::StmtConstant { id, val } => {
                    let ir_val = self.expr(*val);
                    ir.push(IrInst::Bind { id, val: ir_val });
                }
                NodeKind::StmtExpression { expr } => {
                    match expr.kind {
                        NodeKind::ExprAssignment { id, op: _, val } => {
                            let ir_id = self.expr(*id);
                            let ir_val = self.expr(*val);
                            ir.push(IrInst::Store { id: ir_id, val: ir_val });
                        }
                        _ => {}
                    }
                }
                
                _ => unimplemented!("IrCompiler->compile()"),
            }
        }

        return ir;
    }
}

impl IrCompiler {
    fn temp(&mut self) -> IrValue {
        self.temps += 1;
        return IrValue::Temp(self.temps);
    }

    fn expr<'a>(&mut self, node: Node<'a>) -> IrValue {
        match node.kind {
            NodeKind::LiteralIdent { id } => IrValue::Identifier(id),
            NodeKind::LiteralInt { val } => IrValue::ConstInt(val),
            NodeKind::ExprBinary { lhs, op, rhs } => {
                match op {
                    TokenKind::Plus => {
                        let ir_l = self.expr(*lhs);
                        let ir_r = self.expr(*rhs);
                        let temp = self.temp();
                        println!("TEMP {:?} = {:?} + {:?}", temp, ir_l, ir_r);
                        return temp;
                    }
                    _ => unimplemented!("Non + BinaryExprs"),
                }
            }

            _ => unimplemented!("IrCompiler->expr()"),
        }
    }
}
