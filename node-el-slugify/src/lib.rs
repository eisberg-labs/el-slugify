use neon::prelude::*;
use el_slugify::{slugify, slugify_with_replacement};


fn slugify_with_replacement_api(mut cx: FunctionContext) -> JsResult<JsString> {
    let value = cx.argument::<JsString>(0)?;
    let replacement = cx.argument::<JsString>(1)?;

    let value_string = value.value(&mut cx);
    let replacement_string = replacement.value(&mut cx).chars().next().unwrap_or('-');
    let slug = slugify_with_replacement(value_string.as_str(), replacement_string);
    Ok(cx.string(slug))
}

fn slugify_api(mut cx: FunctionContext) -> JsResult<JsString> {
    let value = cx.argument::<JsString>(0)?;

    let value_string = value.value(&mut cx);
    let slug = slugify(value_string.as_str());
    Ok(cx.string(slug))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("slugify", slugify_api)?;
    cx.export_function("slugify_with_replacement", slugify_with_replacement_api)?;
    Ok(())
}
