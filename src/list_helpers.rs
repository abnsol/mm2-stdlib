use eval::EvalScope;
use eval_ffi::{EvalError, ExprSink, ExprSource, Tag};
use mork_expr::{Expr, ExprEnv, SourceItem};

pub fn expr_span(e: Expr) -> &'static [u8] {
    unsafe { e.span().as_ref().unwrap() }
}

pub fn exp_to_vec(tuple_expr: Expr) -> Result<Vec<Expr>, EvalError> {
    let raw_byte = unsafe { *tuple_expr.ptr };
    let expression_tag = mork_expr::byte_item(raw_byte);

    match expression_tag {
        Tag::Arity(arity_size) => {
            let mut parent_env = ExprEnv::new(0, tuple_expr);
            let mut child_envs = Vec::with_capacity(arity_size as usize);
            parent_env.args(&mut child_envs);

            let mut extracted_expressions = Vec::with_capacity(arity_size as usize);
            for child_env in child_envs {
                let child_expr = child_env.subsexpr();
                extracted_expressions.push(child_expr);
            }
            Ok(extracted_expressions)
        }
        _ => {
            Err(EvalError::from("expects a tuple/expression argument"))
        }
    }
}

pub fn vec_to_exp(sink: &mut ExprSink, items: &[Expr]) -> Result<(), EvalError> {
    sink.write(SourceItem::Tag(Tag::Arity(items.len() as u8)))?;
    for e in items {
        sink.extend_from_slice(expr_span(*e))?;
    }
    Ok(())
}

pub fn exp_to_spans(tuple_expr: Expr) -> Result<Vec<Vec<u8>>, EvalError> {
    exp_to_vec(tuple_expr)?
        .into_iter()
        .map(|e| Ok(expr_span(e).to_vec()))
        .collect()
}

pub fn spans_to_exp(sink: &mut ExprSink, items: &[Vec<u8>]) -> Result<(), EvalError> {
    sink.write(SourceItem::Tag(Tag::Arity(items.len() as u8)))?;
    for item in items {
        sink.extend_from_slice(item)?;
    }
    Ok(())
}

pub fn expr_symbol_content(e: Expr) -> Option<&'static [u8]> {
    unsafe { e.symbol()?.as_ref() }
}

pub fn items_to_f64s(items: &[Expr]) -> Result<Vec<f64>, EvalError> {
    items.iter().map(|e| {
        let span = expr_span(*e);
        let s = unsafe { std::str::from_utf8_unchecked(
            span.get(1..).ok_or_else(|| EvalError::from("empty element"))?
        )};
        s.parse().map_err(|_| EvalError::from("element is not a number"))
    }).collect()
}
