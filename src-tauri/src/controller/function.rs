use std::error::Error;

use meval::{Expr, Context};

pub struct Function{
    expr: Expr,
}

impl Function {
    pub fn new(expr_str: &str) -> Result<Function, Box<dyn Error>>{
        let expr: Expr = expr_str.parse()?;
        Ok(Function {expr})
    }

    pub fn eval(&self, x:&f64, y:&f64) -> Result<f64, Box<dyn Error>> {
        let mut ctx = Context::new();
        ctx.var("x", *x);
        ctx.var("X", *x);
        ctx.var("y", *y);
        ctx.var("Y", *y);

        Ok(self.expr.eval_with_context(ctx)?)
    }
}
