use handlebars::{Handlebars, RenderContext, Helper, HelperResult};
use pulldown_cmark::{Parser, html};

pub fn markdown_helper (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> HelperResult {
    let param = h.param(0).unwrap();
    let mut buf = String::new();
    let p = Parser::new(param.value().as_str().unwrap());
    html::push_html(&mut buf, p);
    rc.writer.write(buf.into_bytes().as_ref())?;
    Ok(())
}

pub fn render_markdown(text: &str) -> String {
   let mut buf = String::new();
   let p = Parser::new(text);
   html::push_html(&mut buf, p);
   buf
}