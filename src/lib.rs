extern crate wasm_bindgen; 
use gloo::{events::EventListener};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, console, InputEvent};
use pulldown_cmark::{html, Parser};

mod toolbar;

#[wasm_bindgen(js_name = WebEditor)]
pub struct WebEditor {
    editor: HtmlElement,
    preview: HtmlElement,
}

#[wasm_bindgen]
impl WebEditor {
    #[wasm_bindgen(constructor)]
    pub fn new(editor_selector: &str, preview_selector: &str) -> Result<WebEditor, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        
        let editor = document.query_selector(editor_selector)?.unwrap().dyn_into::<HtmlElement>()?;
        let preview = document.query_selector(preview_selector)?.unwrap().dyn_into::<HtmlElement>()?;
        
        editor.set_attribute("contenteditable", "true")?;
        
        let editor_clone = editor.clone();
        let preview_clone = preview.clone();
        let closure = Closure::wrap(Box::new(move |_: InputEvent| {
            let markdown = editor_clone.inner_text();
            let html = markdown_to_html(&markdown);
            preview_clone.set_inner_html(&html);
        }) as Box<dyn FnMut(_)>);
        
        editor.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
        
        Ok(WebEditor { editor, preview })
    }
    
    pub fn set_content(&self, content: &str) -> Result<(), JsValue> {
        self.editor.set_inner_text(content);
        let html = markdown_to_html(content);
        self.preview.set_inner_html(&html);
        Ok(())
    }
}

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}