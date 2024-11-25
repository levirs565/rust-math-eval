use crate::base::OperatorKind;

pub enum Node {
    Number(f32),
    Operation(OperatorKind, Box<Node>, Box<Node>),
}

pub fn evaluate(node: &Node) -> f32 {
    match &node {
        Node::Operation(operand, lnode, rnode) => {
            let lvalue = evaluate(lnode.as_ref());
            let rvalue = evaluate(rnode.as_ref());
            match operand {
                OperatorKind::ADD => lvalue + rvalue,
                OperatorKind::SUB => lvalue - rvalue,
                OperatorKind::MULT => lvalue * rvalue,
                OperatorKind::DIV => lvalue / rvalue,
                OperatorKind::POW => lvalue.powf(rvalue),
            }
        }
        Node::Number(val) => *val,
    }
}

pub fn draw_string(node: &Node, tab: String) -> String {
    match node {
        Node::Number(num) => format!("{}{}", tab, num.to_string()),
        Node::Operation(op, left, right) => format!(
            "{}{}\n{}\n{}",
            tab,
            match op {
                OperatorKind::ADD => "+",
                OperatorKind::SUB => "-",
                OperatorKind::MULT => "*",
                OperatorKind::DIV => "/",
                OperatorKind::POW => "^",
            },
            draw_string(&left, format!("{}{}", tab, "    ")),
            draw_string(&right, format!("{}{}", tab, "    "))
        ),
    }
}
