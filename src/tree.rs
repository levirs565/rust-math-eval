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

fn make_padding(
    bar_position: &mut Vec<(usize, usize)>,
    padding_size: usize,
    is_end: bool,
) -> String {
    let mut padding = String::new();
    let mut bar = bar_position.iter().peekable();

    for i in 0..padding_size {
        if let Some((pos, _)) = bar.peek() {
            if i == *pos {
                padding.push('|');
                bar.next();
                continue;
            }
        }

        padding.push(if i + 4 >= padding_size && !is_end { '-' } else { ' ' });
    }

    if !is_end {
        if let Some((_, count)) = bar_position.last_mut() {
            *count -= 1;

            if *count == 0 {
                bar_position.pop();
            }
        }
    }

    padding
}

fn draw_string_internal(
    node: &Node,
    padding_size: usize,
    bar_position: &mut Vec<(usize, usize)>,
) -> String {
    let padding = make_padding(bar_position, padding_size, false);

    match node {
        Node::Number(num) => format!("{}{}", padding, num.to_string()),
        Node::Operation(op, left, right) => {
            bar_position.push((padding_size, 2));
            let left = draw_string_internal(&left, padding_size + 4, bar_position);
            let right = draw_string_internal(&right, padding_size + 4, bar_position);
            let padding_end = make_padding(bar_position, padding_size, true);
            format!(
                "{}{}\n{}\n{}\n{}",
                padding,
                match op {
                    OperatorKind::ADD => "+",
                    OperatorKind::SUB => "-",
                    OperatorKind::MULT => "*",
                    OperatorKind::DIV => "/",
                    OperatorKind::POW => "^",
                },
                left,
                right,
                padding_end
            )
        }
    }
}

pub fn draw_string(node: &Node) -> String {
    let mut bar_position: Vec<(usize, usize)> = Vec::new();
    draw_string_internal(node, 0, &mut bar_position)
}
