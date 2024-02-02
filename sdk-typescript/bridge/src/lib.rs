use neon::prelude::*;

mod calculator;
//mod person;


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("sum_numbers", calculator::sum_numbers)?;
    Ok(())
}
