use crate::types::Url;

use neon::prelude::*;

fn parse_url(mut cx: FunctionContext) -> JsResult<JsObject> {
    let input = cx.argument::<JsString>(0)?;
    let input = input.value(&mut cx);

    let url = match Url::new(&input) {
        Ok(url) => url,
        Err(error) => {
            let js_error = cx.error(format!("invalid url: {}", error))?;
            return cx.throw(js_error)?;
        }
    };

    url.to_object(&mut cx)
}

#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parseUrl", parse_url)?;
    Ok(())
}
