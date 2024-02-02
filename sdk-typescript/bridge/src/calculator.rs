use sdk_core::calculator::sum_numbers as base_sum_numbers;
use neon::{
    context::Context,
    prelude::*,
    types::{JsNumber},
};

pub fn sum_numbers(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a = cx.argument::<JsNumber>(0)?.value(&mut cx) as i8;
    let b= cx.argument::<JsNumber>(1)?.value(&mut cx) as i8;

    Ok(cx.number(base_sum_numbers(a, b)))
}