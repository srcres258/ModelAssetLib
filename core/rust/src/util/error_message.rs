use std::cell::RefCell;

struct Bool {
    value: bool
}

pub struct ErrorMessage {
    occurred: RefCell<Bool>,
    content: RefCell<String>
}

impl Bool {
    fn new(value: bool) -> Self {
        Self { value }
    }

    fn set(&mut self, value: bool) {
        self.value = value;
    }

    fn get(&self) -> bool {
        self.value
    }
}

impl ErrorMessage {
    pub fn new() -> Self {
        Self {
            occurred: RefCell::new(Bool::new(false)),
            content: RefCell::new(String::new())
        }
    }

    pub fn with_initial(value: &str) -> Self {
        Self {
            occurred: RefCell::new(Bool::new(false)),
            content: RefCell::new(String::from(value))
        }
    }

    pub fn mark_occurred(&self) {
        self.occurred.borrow_mut().set(true);
    }

    pub fn is_occurred(&self) -> bool {
        self.occurred.borrow().get()
    }
    
    pub fn clear_mark(&self) {
        self.occurred.borrow_mut().set(false);
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
