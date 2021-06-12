extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Person{
	name :String,
	age :u32,
}
#[wasm_bindgen]
impl Person{
	pub fn new()->Person{
		return Person{
			name: "Rust".to_string(),
			age: 30,
		}
	}
	pub fn ToString(&self) ->String{
		return format!("name={} age={}", self.name, self.age);
	}
	pub fn SetAge(&mut self, age :u32){
		self.age = age;
	}
	pub fn SetName(&mut self, name :String){
		self.name = name;
	}
}
