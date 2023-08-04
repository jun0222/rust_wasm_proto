use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TodoList {
    items: Vec<String>,
}

#[wasm_bindgen]
impl TodoList {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TodoList {
        TodoList {
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
        }
    }

    pub fn get_items(&self) -> String {
        self.items.join("\n")
    }
}
