use std::cell::Cell;
use std::rc::Rc;
use yew::services::ConsoleService;

pub type Value = f32;

#[derive(Clone)]
pub struct SharedValue(Rc<Cell<Value>>);

pub type SharedPair = (SharedValue, SharedValue);

impl SharedValue {
    pub fn new(init: Value) -> Self {
        Self(Rc::new(Cell::new(init)))
    }

    pub fn new_pair(x: Value, y: Value) -> SharedPair {
        (Self::new(x), Self::new(y))
    }

    pub fn get(&self) -> Value {
        self.0.get()
    }

    pub fn set(&mut self, value: Value) {
        self.0.set(value)
    }

    pub fn set_str(&mut self, console: &mut ConsoleService, data: yew::html::InputData) {
        match data.value.parse() {
            Ok(v) => self.set(v),
            Err(err) => console.error(&format!("{:?}", err)),
        }
    }
}
