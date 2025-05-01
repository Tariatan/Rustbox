/// An operation to perform on two subexpressions.
#[derive(Debug)]
pub(crate) enum Operation
{
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
pub(crate) enum Expression
{
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}

pub(crate) fn eval(e: Expression) -> i64
{
    match e
    {
        Expression::Op { op, left, right } =>
            {
                let left = eval(*left);
                let right = eval(*right);
                match op
                {
                    Operation::Add => left + right,
                    Operation::Sub => left - right,
                    Operation::Mul => left * right,
                    Operation::Div => left / right,
                }
            }
        Expression::Value(v) => v,
    }
}