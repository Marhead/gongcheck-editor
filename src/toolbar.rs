use wasm_bindgen::JsValue;
use web_sys::{Document, Element};

pub fn create_toolbar(document: &Document, wrapper: &Element) -> Result<(), JsValue> {
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

    Ok(())
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