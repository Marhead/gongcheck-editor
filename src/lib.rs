extern crate wasm_bindgen; 
use gloo::{events::EventListener};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;

#[wasm_bindgen(js_name = WebEditor)]
pub fn web_editor(dom: &str) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists"); //window 객체를 러스트에서 다룰 수 있도록 가져옴
    let document = window.document().expect("should have a document on window"); //마찬가지로 document 객체를 가져올 수 있도록 가져옴
    let ele = &format!("{}", dom);
    let wrapper = document.query_selector(&ele).unwrap().unwrap().dyn_into::<web_sys::HtmlElement>().unwrap(); //javascript(index.html)에서 입력한 id를 가져옴
    wrapper.set_class_name("wasm-editor"); //해당 div의 클래스를 가져온다.
    let editor = document.create_element("div")?; //div를 하나 생성
    editor.set_class_name("wasm-content");
    editor.set_attribute("contenteditable", "true"); //편집 가능한 div로 속성을 정함
    editor.set_attribute("placeholder", "내용을 입력하세요."); //placeholder 속성이나 div에는 동작하지 않음. 추후 기능을 넣기 위해 넣은 요소
    editor.set_inner_html("<p><br></p>"); //개행(엔터) 처리가 될 때 p태그가 추가되게 하기 위하여 초기값으로 <p> 태그를 넣어준다.

    wrapper.append_child(&editor); // 추가
    Ok(()) //리턴
}

// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
