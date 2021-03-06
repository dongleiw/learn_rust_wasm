extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::rc::Rc;

use crate::wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::MouseEvent;

mod person;

#[wasm_bindgen]
extern {
	pub fn alert(s :&str);
}

#[wasm_bindgen]
pub fn strlen(s :&str) -> usize{
	let log = format!("strlen str[{}] in rust wasm",s);
	console::log_1( &log.as_str().into() );
	return s.len();
}

#[wasm_bindgen]
pub fn add(e :MouseEvent){
	console::log_2(&"in rust wasm".into(), &e);
}


fn log_in_html(s :&String){
	let document = web_sys::window().expect("window not exist").document().expect("document not exist");
	let pre = document.get_element_by_id("log").expect("pre not found");
	let text_content = format!("{}\nin rust wasm: {}", pre.text_content().unwrap(), s);
	pre.set_text_content( Some(text_content.as_str()) );
}

fn create_btn_and_add_click_event(){
	let document = web_sys::window().expect("window not exist").document().expect("document not exist");
	let body = document.body().expect("body not exist");
	let btn = Rc::new(document.create_element("button").expect("failed to create button"));
	btn.set_text_content(Some("created_in_rust_wasm: 0"));
	btn.set_id("created_in_rust_wasm");
	
	{
		let btn_cloned = btn.clone();
		let mut click_count = 0;
		let eh_onclick = Closure::wrap(Box::new(move||{
			log_in_html( &format!("btn[created_in_rust_wasm] clicked: {}", click_count) );

			click_count += 1;
			let new_text_content = format!("created_in_rust_wasm: {}", click_count);
			btn_cloned.set_text_content(Some(new_text_content.as_str()));
		}) as Box<dyn FnMut()>);

		btn.add_event_listener_with_callback("click", eh_onclick.as_ref().unchecked_ref()).unwrap();
		// Closure `handler` drop的时候会销毁内存, 对应的js function会失效. 
		// 所以为了让该函数始终生效, 需要forget. 也就是在drop的时候保持js function有效.
		// 会导致leak memory
		eh_onclick.forget();
	}

	let div = document.get_element_by_id("2").expect("div[2] not found");
	div.append_child(&btn).unwrap();

	log_in_html(&"create btn. register onclick event handler".into());
}

fn find_btn_and_add_click_event(){
	let document = web_sys::window().expect("window not exist").document().expect("document not exist");
	let e = document.get_element_by_id("handle_event_in_rust_wasm");
	if e.is_some() {
		let btn = e.unwrap();
		let mut click_count = 0;
		let handler = Closure::wrap(Box::new(move ||{
			click_count += 1;
			log_in_html(&format!("btn[handle_event_in_rust_wasm] clicked: {}", click_count));
		}) as Box<dyn FnMut()>);

		// callback.as_ref() 获得JsValue
		btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref()).unwrap();
		handler.forget();
	}else{
		log_in_html(& String::from("btn[handle_event_in_rust_wasm] not found") );
	}
}

#[wasm_bindgen]
pub fn create_element_and_append(container_id :String){
	let document = web_sys::window().expect("window not exist").document().expect("document not exist");
	
	let container = document.get_element_by_id(container_id.as_ref());
	if container.is_some(){
		let p = document.create_element("p").expect("failed to create p element");
		p.set_text_content( Some("this element is created in rust-wasm" ) );
		container.unwrap().append_child(&p);
	}else{
		console::log_1( &format!("container[{}]", container_id).into() );
	}
}
#[wasm_bindgen]
pub fn delete_element_in_container(container_id :String){
	let document = web_sys::window().expect("window not exist").document().expect("document not exist");
	
	let container = document.get_element_by_id(container_id.as_ref());
	if container.is_none(){
		console::log_1( &format!("container[{}] not found", container_id).into() );
		return;
	}
	let container = container.unwrap();
	let children = container.children();
	let mut idx = 0;
	while idx< children.length(){
		let child = children.item(idx).unwrap();
		if child.node_name()=="P"{
			container.remove_child(&child);
			break;
		}
		idx += 1;
	}
}


#[wasm_bindgen]
pub fn init_wasm(){
	create_btn_and_add_click_event();
	find_btn_and_add_click_event();
}
