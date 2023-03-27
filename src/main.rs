use std::cell::RefCell;
use std::rc::Rc;

mod lists;

struct Cons {
    value : i32,
    next : Option<Rc<RefCell<Cons>>>
}

impl Cons {
    pub fn new(i : i32) -> Cons {
        Cons{value: i, next : None}
    }

    pub fn append(&self, i : i32) -> Cons {
        Cons::new(i)
    }

    pub fn prepend(&self, i : i32) -> Cons {
        Cons::new(i)
    }

    pub fn head(&self) -> i32 {
        self.value
    }

    pub fn drop_first(&self, i : i32) -> Cons {
        Cons::new(i)
    }

    pub fn drop_last(&self, i : i32) -> Cons {
        Cons::new(i)
    }
}

#[cfg(test)]
mod tests {
    use crate::Cons;

    #[test]
    fn new_list() {
        assert_eq!(42, Cons::new(42).head());
    }
}

fn main() {
    println!("Hello, world!");
}
