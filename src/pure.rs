use eval::{EvalScope, FuncType};
use eval_ffi::{EvalError, ExprSink, ExprSource, SourceItem, Tag};
use mork_expr::Expr;
use crate::list_helpers::{exp_to_vec, vec_to_exp, exp_to_spans, spans_to_exp, expr_span, items_to_f64s, expr_symbol_content};

macro_rules! relational_binary {
    ($name:ident($x:ident: $tx:ty, $y:ident: $ty:ty) => $e:expr) => {
        pub extern "C" fn $name(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
            let expr = unsafe { &mut *expr };
            let sink = unsafe { &mut *sink };
            let items = expr.consume_head_check(stringify!($name).as_bytes())?;
            if items != 2 { return Err(EvalError::from(concat!(stringify!($name), " takes two arguments"))) }
            let $x = expr.consume::<$tx>()?;
            let $y = expr.consume::<$ty>()?;
            let r : &[u8] = if $e {b"true"} else {b"false"};
            sink.write(SourceItem::Symbol(r))?;
            Ok(())
        }
    }
}

macro_rules! bool_from_string {
    ($name:ident) => {
        pub extern "C" fn $name(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
            let expr = unsafe { &mut *expr };
            let sink = unsafe { &mut *sink };
            let items = expr.consume_head_check(stringify!($name).as_bytes())?;
            if items != 1 { return Err(EvalError::from("only takes one argument")) }
            let SourceItem::Symbol(symbol) = expr.read() else { return Err(EvalError::from("only parses symbols")) };
            let result: bool = str::from_utf8(symbol).map_err(|_| EvalError::from(concat!(stringify!($name), " parsing string not utf8")))?.parse().map_err(|_| EvalError::from(concat!("string not a valid type in ", stringify!($name))))?;
            let r: &[u8] = if result { b"true" } else { b"false" };
            sink.write(SourceItem::Symbol(r.into()))?;
            Ok(())
        }
    }
}

// Relational operators
relational_binary!(lt_u8(x:u8, y:u8) => x < y);
relational_binary!(lt_u16(x:u16, y:u16) => x < y);
relational_binary!(lt_u32(x:u32, y:u32) => x < y);
relational_binary!(lt_u64(x:u64, y:u64) => x < y);
relational_binary!(lt_u128(x:u128, y:u128) => x < y);
relational_binary!(lt_i8(x:i8, y:i8) => x < y);
relational_binary!(lt_i16(x:i16, y:i16) => x < y);
relational_binary!(lt_i32(x:i32, y:i32) => x < y);
relational_binary!(lt_i64(x:i64, y:i64) => x < y);
relational_binary!(lt_i128(x:i128, y:i128) => x < y);
relational_binary!(lt_f32(x:f32, y:f32) => x < y);
relational_binary!(lt_f64(x:f64, y:f64) => x < y);
relational_binary!(gt_u8(x:u8, y:u8) => x > y);
relational_binary!(gt_u16(x:u16, y:u16) => x > y);
relational_binary!(gt_u32(x:u32, y:u32) => x > y);
relational_binary!(gt_u64(x:u64, y:u64) => x > y);
relational_binary!(gt_u128(x:u128, y:u128) => x > y);
relational_binary!(gt_i8(x:i8, y:i8) => x > y);
relational_binary!(gt_i16(x:i16, y:i16) => x > y);
relational_binary!(gt_i32(x:i32, y:i32) => x > y);
relational_binary!(gt_i64(x:i64, y:i64) => x > y);
relational_binary!(gt_i128(x:i128, y:i128) => x > y);
relational_binary!(gt_f32(x:f32, y:f32) => x > y);
relational_binary!(gt_f64(x:f64, y:f64) => x > y);
relational_binary!(eq_u8(x:u8, y:u8) => x == y);
relational_binary!(eq_u16(x:u16, y:u16) => x == y);
relational_binary!(eq_u32(x:u32, y:u32) => x == y);
relational_binary!(eq_u64(x:u64, y:u64) => x == y);
relational_binary!(eq_u128(x:u128, y:u128) => x == y);
relational_binary!(eq_i8(x:i8, y:i8) => x == y);
relational_binary!(eq_i16(x:i16, y:i16) => x == y);
relational_binary!(eq_i32(x:i32, y:i32) => x == y);
relational_binary!(eq_i64(x:i64, y:i64) => x == y);
relational_binary!(eq_i128(x:i128, y:i128) => x == y);
relational_binary!(eq_f32(x:f32, y:f32) => x == y);
relational_binary!(eq_f64(x:f64, y:f64) => x == y);
relational_binary!(ne_u8(x:u8, y:u8) => x != y);
relational_binary!(ne_u16(x:u16, y:u16) => x != y);
relational_binary!(ne_u32(x:u32, y:u32) => x != y);
relational_binary!(ne_u64(x:u64, y:u64) => x != y);
relational_binary!(ne_u128(x:u128, y:u128) => x != y);
relational_binary!(ne_i8(x:i8, y:i8) => x != y);
relational_binary!(ne_i16(x:i16, y:i16) => x != y);
relational_binary!(ne_i32(x:i32, y:i32) => x != y);
relational_binary!(ne_i64(x:i64, y:i64) => x != y);
relational_binary!(ne_i128(x:i128, y:i128) => x != y);
relational_binary!(ne_f32(x:f32, y:f32) => x != y);
relational_binary!(ne_f64(x:f64, y:f64) => x != y);
relational_binary!(le_u8(x:u8, y:u8) => x <= y);
relational_binary!(le_u16(x:u16, y:u16) => x <= y);
relational_binary!(le_u32(x:u32, y:u32) => x <= y);
relational_binary!(le_u64(x:u64, y:u64) => x <= y);
relational_binary!(le_u128(x:u128, y:u128) => x <= y);
relational_binary!(le_i8(x:i8, y:i8) => x <= y);
relational_binary!(le_i16(x:i16, y:i16) => x <= y);
relational_binary!(le_i32(x:i32, y:i32) => x <= y);
relational_binary!(le_i64(x:i64, y:i64) => x <= y);
relational_binary!(le_i128(x:i128, y:i128) => x <= y);
relational_binary!(le_f32(x:f32, y:f32) => x <= y);
relational_binary!(le_f64(x:f64, y:f64) => x <= y);
relational_binary!(ge_u8(x:u8, y:u8) => x >= y);
relational_binary!(ge_u16(x:u16, y:u16) => x >= y);
relational_binary!(ge_u32(x:u32, y:u32) => x >= y);
relational_binary!(ge_u64(x:u64, y:u64) => x >= y);
relational_binary!(ge_u128(x:u128, y:u128) => x >= y);
relational_binary!(ge_i8(x:i8, y:i8) => x >= y);
relational_binary!(ge_i16(x:i16, y:i16) => x >= y);
relational_binary!(ge_i32(x:i32, y:i32) => x >= y);
relational_binary!(ge_i64(x:i64, y:i64) => x >= y);
relational_binary!(ge_i128(x:i128, y:i128) => x >= y);
relational_binary!(ge_f32(x:f32, y:f32) => x >= y);
relational_binary!(ge_f64(x:f64, y:f64) => x >= y);

// Boolean operators
bool_from_string!(bool_from_string);
relational_binary!(and_bool(x: bool, y: bool) => x && y);
relational_binary!(or_bool(x: bool, y: bool) => x || y);
relational_binary!(xor_bool(x: bool, y: bool) => x ^ y);
relational_binary!(implies_bool(x: bool, y: bool) => !x || y);

// --- List operations ---

pub extern "C" fn length(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"length")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }

    let tuple_expr = expr.consume::<Expr>()?;
    let n = exp_to_vec(tuple_expr)?.len() as i64;
    let num_str = n.to_string();
    sink.write(SourceItem::Symbol(num_str.as_bytes().into()))?;
    Ok(())
}

pub extern "C" fn car(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };

    if expr.consume_head_check(b"car")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let tuple_expr = expr.consume::<Expr>()?;

    let items = exp_to_vec(tuple_expr)?;
    if items.is_empty() {
        return Err(EvalError::from("car on empty tuple"));
    }

    sink.extend_from_slice(expr_span(items[0]))?;
    Ok(())
}

pub extern "C" fn cdr(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };

    if expr.consume_head_check(b"cdr")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let tuple_expr = expr.consume::<Expr>()?;

    let items = exp_to_vec(tuple_expr)?;
    if items.is_empty() {
        return Err(EvalError::from("cdr on empty tuple"));
    }

    vec_to_exp(sink, &items[1..])
}

pub extern "C" fn cons(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };

    let arity = expr.consume_head_check(b"cons")?;
    let (head, tail_tuple) = match arity {
        1 => {
            let args = expr.consume::<Expr>()?;
            let items = exp_to_vec(args)?;
            if items.len() != 2 {
                return Err(EvalError::from("cons pair form takes two items"));
            }
            (items[0], items[1])
        }
        2 => {
            let head = expr.consume::<Expr>()?;
            let tail_tuple = expr.consume::<Expr>()?;
            (head, tail_tuple)
        }
        _ => return Err(EvalError::from("takes one argument pair or two arguments")),
    };
    let head_span = expr_span(head).to_vec();
    let tail_items = exp_to_spans(tail_tuple)?;

    sink.write(SourceItem::Tag(Tag::Arity((tail_items.len() + 1) as u8)))?;
    sink.extend_from_slice(&head_span)?;
    for e in &tail_items {
        sink.extend_from_slice(e)?;
    }
    Ok(())
}

pub extern "C" fn decons(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };

    if expr.consume_head_check(b"decons")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let tuple_expr = expr.consume::<Expr>()?;

    let items = exp_to_vec(tuple_expr)?;
    if items.is_empty() {
        return Err(EvalError::from("decons on empty tuple"));
    }

    sink.write(SourceItem::Tag(Tag::Arity(2)))?;
    sink.extend_from_slice(expr_span(items[0]))?;
    vec_to_exp(sink, &items[1..])?;
    Ok(())
}

pub extern "C" fn first(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"first")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let pair = expr.consume::<Expr>()?;
    let items = exp_to_vec(pair)?;
    if items.is_empty() {
        return Err(EvalError::from("first on empty tuple"));
    }
    sink.extend_from_slice(expr_span(items[0]))?;
    Ok(())
}

pub extern "C" fn first_from_pair(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"first-from-pair")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let pair = expr.consume::<Expr>()?;
    let items = exp_to_vec(pair)?;
    if items.is_empty() {
        return Err(EvalError::from("first-from-pair on empty tuple"));
    }
    sink.extend_from_slice(expr_span(items[0]))?;
    Ok(())
}

pub extern "C" fn second_from_pair(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"second-from-pair")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let pair = expr.consume::<Expr>()?;
    let items = exp_to_vec(pair)?;
    if items.len() < 2 {
        return Err(EvalError::from("second-from-pair on tuple with fewer than 2 elements"));
    }
    sink.extend_from_slice(expr_span(items[1]))?;
    Ok(())
}

pub extern "C" fn unique_atom(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"unique-atom")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let list = expr.consume::<Expr>()?;
    let items = exp_to_vec(list)?;
    let mut seen = Vec::with_capacity(items.len());
    let mut unique = Vec::with_capacity(items.len());
    for item in &items {
        let span = expr_span(*item);
        if !seen.iter().any(|s: &&[u8]| *s == span) {
            seen.push(span);
            unique.push(*item);
        }
    }
    vec_to_exp(sink, &unique)
}

pub extern "C" fn size_atom(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"size-atom")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let tuple_expr = expr.consume::<Expr>()?;
    let n = exp_to_vec(tuple_expr)?.len() as i64;
    let num_str = n.to_string();
    sink.write(SourceItem::Symbol(num_str.as_bytes().into()))?;
    Ok(())
}

pub extern "C" fn car_atom(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"car-atom")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let list = expr.consume::<Expr>()?;
    let items = exp_to_vec(list)?;
    if items.is_empty() {
        return Err(EvalError::from("car-atom on empty tuple"));
    }
    sink.extend_from_slice(expr_span(items[0]))?;
    Ok(())
}

pub extern "C" fn cdr_atom(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"cdr-atom")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let list = expr.consume::<Expr>()?;
    let items = exp_to_vec(list)?;
    if items.is_empty() {
        return Err(EvalError::from("cdr-atom on empty tuple"));
    }
    vec_to_exp(sink, &items[1..])
}

pub extern "C" fn index_atom(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    let arity = expr.consume_head_check(b"index-atom")?;
    let (items, index_span) = match arity {
        1 => {
            let args = expr.consume::<Expr>()?;
            let pair = exp_to_vec(args)?;
            if pair.len() != 2 {
                return Err(EvalError::from("index-atom pair form takes two items"));
            }
            (exp_to_spans(pair[0])?, expr_span(pair[1]).to_vec())
        }
        2 => {
            let list = expr.consume::<Expr>()?;
            let items = exp_to_spans(list)?;
            let index_expr = expr.consume::<Expr>()?;
            (items, expr_span(index_expr).to_vec())
        }
        _ => return Err(EvalError::from("takes one argument pair or two arguments")),
    };
    let index_str = unsafe { std::str::from_utf8_unchecked(index_span.get(1..).ok_or_else(|| EvalError::from("invalid index span"))?) };
    let index: usize = index_str.parse().map_err(|_| EvalError::from("invalid index"))?;
    if index >= items.len() {
        return Err(EvalError::from("index out of bounds"));
    }
    sink.extend_from_slice(&items[index])?;
    Ok(())
}

pub extern "C" fn is_member(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    let arity = expr.consume_head_check(b"is-member")?;
    let (elem_span, items) = match arity {
        1 => {
            let args = expr.consume::<Expr>()?;
            let pair = exp_to_vec(args)?;
            if pair.len() != 2 {
                return Err(EvalError::from("is-member pair form takes two items"));
            }
            (expr_span(pair[0]).to_vec(), exp_to_spans(pair[1])?)
        }
        2 => {
            let elem = expr.consume::<Expr>()?;
            let elem_span = expr_span(elem).to_vec();
            let list = expr.consume::<Expr>()?;
            (elem_span, exp_to_spans(list)?)
        }
        _ => return Err(EvalError::from("takes one argument pair or two arguments")),
    };
    let found = items.iter().any(|item| item == &elem_span);
    let s = if found { "true" } else { "false" };
    sink.write(SourceItem::Symbol(s.as_bytes().into()))?;
    Ok(())
}

fn consume_list_pair(expr: &mut ExprSource, head: &[u8]) -> Result<(Vec<Vec<u8>>, Vec<Vec<u8>>), EvalError> {
    match expr.consume_head_check(head)? {
        1 => {
            let args = expr.consume::<Expr>()?;
            let pair = exp_to_vec(args)?;
            if pair.len() != 2 {
                return Err(EvalError::from("pair form takes two list items"));
            }
            Ok((exp_to_spans(pair[0])?, exp_to_spans(pair[1])?))
        }
        2 => {
            let list1 = expr.consume::<Expr>()?;
            let items1 = exp_to_spans(list1)?;
            let list2 = expr.consume::<Expr>()?;
            let items2 = exp_to_spans(list2)?;
            Ok((items1, items2))
        }
        _ => Err(EvalError::from("takes one argument pair or two arguments")),
    }
}

pub extern "C" fn subtraction_atom(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    let (items1, items2) = consume_list_pair(expr, b"subtraction-atom")?;
    let mut result = Vec::with_capacity(items1.len());
    let mut to_remove = items2;
    for item in items1 {
        if let Some(pos) = to_remove.iter().position(|s| s == &item) {
            to_remove.swap_remove(pos);
        } else {
            result.push(item);
        }
    }
    spans_to_exp(sink, &result)
}

pub extern "C" fn union_atom(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    let (items1, items2) = consume_list_pair(expr, b"union-atom")?;
    let mut result = Vec::with_capacity(items1.len() + items2.len());
    for item in items1.into_iter().chain(items2) {
        if !result.iter().any(|seen| seen == &item) {
            result.push(item);
        }
    }
    spans_to_exp(sink, &result)
}

pub extern "C" fn intersection_atom(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    let (items1, items2) = consume_list_pair(expr, b"intersection-atom")?;
    let result_capacity = items1.len().min(items2.len());
    let mut counts2: Vec<(Vec<u8>, usize)> = Vec::with_capacity(items2.len());
    for item in items2 {
        if let Some((_, count)) = counts2.iter_mut().find(|(s, _)| s == &item) {
            *count += 1;
        } else {
            counts2.push((item, 1));
        }
    }
    let mut result = Vec::with_capacity(result_capacity);
    for item in items1 {
        if let Some((_, count)) = counts2.iter_mut().find(|(s, _)| s == &item) {
            if *count > 0 {
                result.push(item);
                *count -= 1;
            }
        }
    }
    spans_to_exp(sink, &result)
}

pub extern "C" fn append(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    let (mut result, items2) = consume_list_pair(expr, b"append")?;
    result.extend(items2);
    spans_to_exp(sink, &result)
}

pub extern "C" fn last(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"last")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let list = expr.consume::<Expr>()?;
    let items = exp_to_vec(list)?;
    if items.is_empty() {
        return Err(EvalError::from("last on empty list"));
    }
    sink.extend_from_slice(expr_span(items[items.len() - 1]))?;
    Ok(())
}

pub extern "C" fn reverse(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"reverse")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let list = expr.consume::<Expr>()?;
    let mut items = exp_to_vec(list)?;
    items.reverse();
    vec_to_exp(sink, &items)
}

pub extern "C" fn exclude_item(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    let arity = expr.consume_head_check(b"exclude-item")?;
    let (elem_span, items) = match arity {
        1 => {
            let args = expr.consume::<Expr>()?;
            let pair = exp_to_vec(args)?;
            if pair.len() != 2 {
                return Err(EvalError::from("exclude-item pair form takes two items"));
            }
            (expr_span(pair[0]).to_vec(), exp_to_spans(pair[1])?)
        }
        2 => {
            let elem = expr.consume::<Expr>()?;
            let elem_span = expr_span(elem).to_vec();
            let list = expr.consume::<Expr>()?;
            (elem_span, exp_to_spans(list)?)
        }
        _ => return Err(EvalError::from("takes one argument pair or two arguments")),
    };
    let result: Vec<Vec<u8>> = items.into_iter().filter(|item| item != &elem_span).collect();
    spans_to_exp(sink, &result)
}

pub extern "C" fn min_atom(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"min-atom")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let list = expr.consume::<Expr>()?;
    let items = exp_to_vec(list)?;
    if items.is_empty() {
        return Err(EvalError::from("min-atom on empty list"));
    }
    let vals = items_to_f64s(&items)?;
    let idx = vals.iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(i, _)| i)
        .unwrap();
    sink.extend_from_slice(expr_span(items[idx]))?;
    Ok(())
}

pub extern "C" fn max_atom(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"max-atom")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let list = expr.consume::<Expr>()?;
    let items = exp_to_vec(list)?;
    if items.is_empty() {
        return Err(EvalError::from("max-atom on empty list"));
    }
    let vals = items_to_f64s(&items)?;
    let idx = vals.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(i, _)| i)
        .unwrap();
    sink.extend_from_slice(expr_span(items[idx]))?;
    Ok(())
}

pub extern "C" fn sort_atom(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"sort-atom")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let list = expr.consume::<Expr>()?;
    let mut items = exp_to_vec(list)?;
    items.sort_unstable_by(|a, b| {
        match (expr_symbol_content(*a), expr_symbol_content(*b)) {
            (Some(a), Some(b)) => a.cmp(b),
            (None, None) => expr_span(*a).cmp(expr_span(*b)),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
        }
    });
    vec_to_exp(sink, &items)
}

pub extern "C" fn sort_math(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    let expr = unsafe { &mut *expr };
    let sink = unsafe { &mut *sink };
    if expr.consume_head_check(b"sort-math")? != 1 {
        return Err(EvalError::from("takes one argument"));
    }
    let list = expr.consume::<Expr>()?;
    let items = exp_to_vec(list)?;
    let vals = items_to_f64s(&items)?;
    let mut paired: Vec<_> = items.into_iter().zip(vals).collect();
    paired.sort_unstable_by(|a, b| a.1.total_cmp(&b.1));
    let sorted_items: Vec<Expr> = paired.into_iter().map(|(e, _)| e).collect();
    vec_to_exp(sink, &sorted_items)
}

fn foldl_impl(expr: &mut ExprSource, sink: &mut ExprSink, head: &[u8]) -> Result<(), EvalError> {
    let (func_name, mut accum_bytes, items) = match expr.consume_head_check(head)? {
        1 => {
            let args = expr.consume::<Expr>()?;
            let triplet = exp_to_vec(args)?;
            if triplet.len() != 3 {
                return Err(EvalError::from("fold pair form takes three items"));
            }
            (
                expr_span(triplet[0]).to_vec(),
                expr_span(triplet[1]).to_vec(),
                exp_to_spans(triplet[2])?,
            )
        }
        3 => {
            let func = expr.consume::<Expr>()?;
            let func_name = expr_span(func).to_vec();
            let init = expr.consume::<Expr>()?;
            let accum_bytes = expr_span(init).to_vec();
            let list = expr.consume::<Expr>()?;
            (func_name, accum_bytes, exp_to_spans(list)?)
        }
        _ => return Err(EvalError::from("takes one argument triplet or three arguments")),
    };
    for item_bytes in &items {
        let mut new_accum = Vec::with_capacity(1 + func_name.len() + accum_bytes.len() + item_bytes.len());
        new_accum.push(mork_expr::item_byte(Tag::Arity(3)));
        new_accum.extend_from_slice(&func_name);
        new_accum.extend_from_slice(&accum_bytes);
        new_accum.extend_from_slice(item_bytes);
        accum_bytes = new_accum;
    }
    sink.extend_from_slice(&accum_bytes)?;
    Ok(())
}

pub extern "C" fn foldl(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    foldl_impl(unsafe { &mut *expr }, unsafe { &mut *sink }, b"foldl")
}

pub extern "C" fn foldl_atom(expr: *mut ExprSource, sink: *mut ExprSink) -> Result<(), EvalError> {
    foldl_impl(unsafe { &mut *expr }, unsafe { &mut *sink }, b"foldl-atom")
}

// Registration

pub fn register(scope: &mut EvalScope) {
    // Relational operators
    scope.add_func("lt_i8", lt_i8, FuncType::Pure);
    scope.add_func("gt_i8", gt_i8, FuncType::Pure);
    scope.add_func("le_i8", le_i8, FuncType::Pure);
    scope.add_func("ge_i8", ge_i8, FuncType::Pure);
    scope.add_func("eq_i8", eq_i8, FuncType::Pure);
    scope.add_func("ne_i8", ne_i8, FuncType::Pure);
    scope.add_func("lt_i16", lt_i16, FuncType::Pure);
    scope.add_func("gt_i16", gt_i16, FuncType::Pure);
    scope.add_func("le_i16", le_i16, FuncType::Pure);
    scope.add_func("ge_i16", ge_i16, FuncType::Pure);
    scope.add_func("eq_i16", eq_i16, FuncType::Pure);
    scope.add_func("ne_i16", ne_i16, FuncType::Pure);
    scope.add_func("lt_i32", lt_i32, FuncType::Pure);
    scope.add_func("gt_i32", gt_i32, FuncType::Pure);
    scope.add_func("le_i32", le_i32, FuncType::Pure);
    scope.add_func("ge_i32", ge_i32, FuncType::Pure);
    scope.add_func("eq_i32", eq_i32, FuncType::Pure);
    scope.add_func("ne_i32", ne_i32, FuncType::Pure);
    scope.add_func("lt_i64", lt_i64, FuncType::Pure);
    scope.add_func("gt_i64", gt_i64, FuncType::Pure);
    scope.add_func("le_i64", le_i64, FuncType::Pure);
    scope.add_func("ge_i64", ge_i64, FuncType::Pure);
    scope.add_func("eq_i64", eq_i64, FuncType::Pure);
    scope.add_func("ne_i64", ne_i64, FuncType::Pure);
    scope.add_func("lt_i128", lt_i128, FuncType::Pure);
    scope.add_func("gt_i128", gt_i128, FuncType::Pure);
    scope.add_func("le_i128", le_i128, FuncType::Pure);
    scope.add_func("ge_i128", ge_i128, FuncType::Pure);
    scope.add_func("eq_i128", eq_i128, FuncType::Pure);
    scope.add_func("ne_i128", ne_i128, FuncType::Pure);
    scope.add_func("lt_f32", lt_f32, FuncType::Pure);
    scope.add_func("gt_f32", gt_f32, FuncType::Pure);
    scope.add_func("le_f32", le_f32, FuncType::Pure);
    scope.add_func("ge_f32", ge_f32, FuncType::Pure);
    scope.add_func("eq_f32", eq_f32, FuncType::Pure);
    scope.add_func("ne_f32", ne_f32, FuncType::Pure);
    scope.add_func("lt_f64", lt_f64, FuncType::Pure);
    scope.add_func("gt_f64", gt_f64, FuncType::Pure);
    scope.add_func("le_f64", le_f64, FuncType::Pure);
    scope.add_func("ge_f64", ge_f64, FuncType::Pure);
    scope.add_func("eq_f64", eq_f64, FuncType::Pure);
    scope.add_func("ne_f64", ne_f64, FuncType::Pure);
    scope.add_func("lt_u8", lt_u8, FuncType::Pure);
    scope.add_func("gt_u8", gt_u8, FuncType::Pure);
    scope.add_func("le_u8", le_u8, FuncType::Pure);
    scope.add_func("ge_u8", ge_u8, FuncType::Pure);
    scope.add_func("eq_u8", eq_u8, FuncType::Pure);
    scope.add_func("ne_u8", ne_u8, FuncType::Pure);
    scope.add_func("lt_u16", lt_u16, FuncType::Pure);
    scope.add_func("gt_u16", gt_u16, FuncType::Pure);
    scope.add_func("le_u16", le_u16, FuncType::Pure);
    scope.add_func("ge_u16", ge_u16, FuncType::Pure);
    scope.add_func("eq_u16", eq_u16, FuncType::Pure);
    scope.add_func("ne_u16", ne_u16, FuncType::Pure);
    scope.add_func("lt_u32", lt_u32, FuncType::Pure);
    scope.add_func("gt_u32", gt_u32, FuncType::Pure);
    scope.add_func("le_u32", le_u32, FuncType::Pure);
    scope.add_func("ge_u32", ge_u32, FuncType::Pure);
    scope.add_func("eq_u32", eq_u32, FuncType::Pure);
    scope.add_func("ne_u32", ne_u32, FuncType::Pure);
    scope.add_func("lt_u64", lt_u64, FuncType::Pure);
    scope.add_func("gt_u64", gt_u64, FuncType::Pure);
    scope.add_func("le_u64", le_u64, FuncType::Pure);
    scope.add_func("ge_u64", ge_u64, FuncType::Pure);
    scope.add_func("eq_u64", eq_u64, FuncType::Pure);
    scope.add_func("ne_u64", ne_u64, FuncType::Pure);
    scope.add_func("lt_u128", lt_u128, FuncType::Pure);
    scope.add_func("gt_u128", gt_u128, FuncType::Pure);
    scope.add_func("le_u128", le_u128, FuncType::Pure);
    scope.add_func("ge_u128", ge_u128, FuncType::Pure);
    scope.add_func("eq_u128", eq_u128, FuncType::Pure);
    scope.add_func("ne_u128", ne_u128, FuncType::Pure);

    // Boolean operators
    scope.add_func("bool_from_string", bool_from_string, FuncType::Pure);
    scope.add_func("and_bool", and_bool, FuncType::Pure);
    scope.add_func("or_bool", or_bool, FuncType::Pure);
    scope.add_func("xor_bool", xor_bool, FuncType::Pure);
    scope.add_func("implies_bool", implies_bool, FuncType::Pure);

    // List operations
    scope.add_func("length", length, FuncType::Pure);
    scope.add_func("car", car, FuncType::Pure);
    scope.add_func("cdr", cdr, FuncType::Pure);
    scope.add_func("cons", cons, FuncType::Pure);
    scope.add_func("decons", decons, FuncType::Pure);
    scope.add_func("first-from-pair", first_from_pair, FuncType::Pure);
    scope.add_func("first", first, FuncType::Pure);
    scope.add_func("second-from-pair", second_from_pair, FuncType::Pure);
    scope.add_func("unique-atom", unique_atom, FuncType::Pure);
    scope.add_func("size-atom", size_atom, FuncType::Pure);
    scope.add_func("car-atom", car_atom, FuncType::Pure);
    scope.add_func("cdr-atom", cdr_atom, FuncType::Pure);
    scope.add_func("index-atom", index_atom, FuncType::Pure);
    scope.add_func("is-member", is_member, FuncType::Pure);
    scope.add_func("subtraction-atom", subtraction_atom, FuncType::Pure);
    scope.add_func("union-atom", union_atom, FuncType::Pure);
    scope.add_func("intersection-atom", intersection_atom, FuncType::Pure);
    scope.add_func("append", append, FuncType::Pure);
    scope.add_func("foldl-atom", foldl_atom, FuncType::Pure);
    scope.add_func("foldl", foldl, FuncType::Pure);
    scope.add_func("last", last, FuncType::Pure);
    scope.add_func("reverse", reverse, FuncType::Pure);
    scope.add_func("exclude-item", exclude_item, FuncType::Pure);
    scope.add_func("min-atom", min_atom, FuncType::Pure);
    scope.add_func("max-atom", max_atom, FuncType::Pure);
    scope.add_func("sort-atom", sort_atom, FuncType::Pure);
    scope.add_func("sort-math", sort_math, FuncType::Pure);
}
