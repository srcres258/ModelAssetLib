use std::cell::RefCell;

pub struct ErrorMessage {
    content: RefCell<String>
}

impl ErrorMessage {
    pub fn new() -> Self {
        Self {
            content: RefCell::new(String::new())
        }
    }

    pub fn with_initial(value: &str) -> Self {
        Self {
            content: RefCell::new(String::from(value))
        }
    }

    pub fn get(&self) -> String {
        self.content.borrow().clone()
    }

    pub fn set(&self, value: &str) {
        let mut str = self.content.borrow_mut();
        str.clear();
        value.chars().for_each(|c| {
            str.push(c);
        })
    }
}
