use std::rc::Rc;
struct ListNode {
    val: i32,
    next: Option<Rc<ListNode>>,
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct Data {
    pub val: i32,
}

#[cfg(test)]
mod tests {
    use crate::rust::smart_pointer::rc::List::{Cons, Nil};

    use super::*;

    //  3
    //   \
    //    5 -> 10 -> Null
    //   /
    // 4
    #[test]
    fn test1() {
        let a = Some(Rc::new(ListNode {
            val: 5,
            next: Some(Rc::new(ListNode {
                val: 10,
                next: None,
            })),
        }));

        let b = Some(Rc::new(ListNode {
            val: 3,
            next: a.clone(),
        }));
        let c = Some(Rc::new(ListNode {
            val: 4,
            next: a.clone(),
        }));
    }

    #[test]
    fn test2() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, a.clone());
        let c = Cons(4, a.clone());
    }

    #[test]
    fn test3() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a)); // 1

        let b = Cons(3, a.clone());
        println!("count after creating b = {}", Rc::strong_count(&a)); // 2

        {
            let c = Cons(4, a.clone());
            println!("count after creating c = {}", Rc::strong_count(&a)); // 3
        }

        println!(
            "count after c going out of scope = {}",
            Rc::strong_count(&a)
        ); // 2
    }

    #[test]
    fn test4() {
        let a = Rc::new(Data { val: 1 });
        let b = a.clone();
        let c = a.clone();
        // wrong!
        // b.val = 1;
        // c.val = 2;
    }
}
