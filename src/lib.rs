extern crate wasm_bindgen; 
use gloo::{events::EventListener};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use web_sys::InputEvent;
use web_sys::{Document, Element, HtmlElement, console};
use pulldown_cmark::{html, Parser};

#[wasm_bindgen(js_name = WebEditor)]
pub fn web_editor(dom: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();                          // Get `document` object
    
    let wrapper = document.query_selector(dom)?.unwrap().dyn_into::<HtmlElement>()?;    // javascript(index.html)에서 입력한 id를 가져옴
    wrapper.set_class_name("wasm-editor");                                                           // 해당 div의 클래스를 가져온다.
    
    let editor = document.create_element("div")?;                                           // div를 하나 생성
    editor.set_class_name("wasm-content");
    editor.set_attribute("contenteditable", "true")?;                                                // 편집 가능한 div로 속성을 정함
    editor.set_inner_html("<p><br></p>");
    
    // Set up an event listener for input events on the editor
    let editor_clone = editor.clone();
    let closure = Closure::wrap(Box::new(move |_event: web_sys::InputEvent| {
        let editor = editor_clone.dyn_ref::<HtmlElement>().unwrap();
        let text = editor.inner_text(); // Get the current text from the editor
        console::log_1(&text.into()); // Print the text to the console
    }) as Box<dyn FnMut(_)>);

    editor.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
    closure.forget(); // Note: this leaks memory; for a real application, you should manage closures' lifetimes more carefully
                                             // 개행(엔터) 처리가 될 때 p태그가 추가되게 하기 위하여 초기값으로 <p> 태그를 넣어준다.

    /** 툴바 추가 **/
    let toolbar = document.create_element("div").unwrap();                                  // 툴바의 부모 div
    toolbar.set_class_name("wasm-toolbar");                                                         //css를 위해 클래스를 설정해줌.

    let textleft = document.create_element("button").unwrap();     // document.createElement('button')
    textleft.set_class_name("toolbar-menu text-align-left");                // textleft.className
    textleft.set_inner_html(&"왼");                                         // textleft.setInnerHtml
    toolbar.append_child(&textleft);                                        // toolbar.appendChild

    let textleft = document.create_element("button").unwrap();
    textleft.set_class_name("toolbar-menu text-align-left");
    textleft.set_inner_html(&"왼");
    toolbar.append_child(&textleft);
    
    let textcenter = document.create_element("button").unwrap();
    textcenter.set_class_name("toolbar-menu text-align-center");
    textcenter.set_inner_html(&"가");
    toolbar.append_child(&textcenter);
    
    let textright = document.create_element("button").unwrap();
    textright.set_class_name("toolbar-menu text-align-right");
    textright.set_inner_html(&"오");
    toolbar.append_child(&textright);
    
    let bold = document.create_element("button").unwrap();
    bold.set_class_name("toolbar-menu bold");
    bold.set_inner_html(&"B");
    toolbar.append_child(&bold);
    
    let italic = document.create_element("button").unwrap();
    italic.set_class_name("toolbar-menu italic");
    italic.set_inner_html(&"I");
    toolbar.append_child(&italic);
    
    let underline = document.create_element("button").unwrap();
    underline.set_class_name("toolbar-menu underline");
    underline.set_inner_html(&"U");
    toolbar.append_child(&underline);
    
    for i in 1..6 {
        let heading = document.create_element("button").unwrap();
        heading.set_class_name("toolbar-menu heading");
        heading.set_inner_html(&format!("H{}", i));
        toolbar.append_child(&heading);
    }
    
    let hyperlink = document.create_element("button").unwrap();
    hyperlink.set_class_name("toolbar-menu hyperlink");
    hyperlink.set_inner_html(&"A");
    
    wrapper.append_child(&toolbar)?; // 에디터내에 툴바 div를 추가.
    wrapper.append_child(&editor)?; // 추가
    
    Ok(()) //리턴
}

pub fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

// fn create_and_append_button(document: &Document, parent: &Element, class: &str, inner_html: &str) -> Result<(), JsValue> {
//     let button = create_element(document, "button", Some(class), Some(inner_html))?;
//     parent.append_child(&button)?;
//     Ok(())
// }

// fn create_element(document: &Document, tag: &str, class_name: Option<&str>, inner_html: Option<&str>) -> Result<Element, JsValue> {
//     let element = document.create_element(tag)?;
//     if let Some(class_name) = class_name {
//         element.set_class_name(class_name);
//     }
//     if let Some(inner_html) = inner_html {
//         element.set_inner_html(inner_html);
//     }
//     Ok(element)
// }