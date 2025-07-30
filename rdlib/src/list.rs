use std::rc::Rc;

pub type Link<T> = Option<Rc<Box<Node<T>>>>;

pub struct Node<T: Copy> {
    elem: T,
    next: Link<T>,
}

pub fn to_string<T: Copy + std::fmt::Display>(first: &Link<T>) -> String {
    match first {
        None => "nil".to_string(),
        Some(node) => format!(
            "{}({}) -> {}",
            node.elem,
            Rc::strong_count(&node),
            to_string(&node.next),
        ),
    }
}

pub fn member<T: Copy + PartialEq>(x: T, first: &Link<T>) -> bool {
    match first {
        None => false,
        Some(node) => {
            if node.elem == x {
                true
            } else {
                member(x, &node.next)
            }
        }
    }
}

pub fn cons<T: Copy>(x: T, next: &Link<T>) -> Link<T> {
    Some(Rc::new(Box::new(Node {
        elem: x,
        next: rc_clone(next),
    })))
}

pub fn car<T: Copy>(list: &Link<T>) -> Option<T> {
    match list {
        None => None,
        Some(node) => Some(node.elem),
    }
}

pub fn cdr<T: Copy>(list: &Link<T>) -> Link<T> {
    match list {
        None => None,
        Some(node) => match &node.next {
            None => None,
            Some(next_node) => Some(Rc::clone(next_node)),
        },
    }
}

fn rc_clone<T: Copy>(list: &Link<T>) -> Link<T> {
    match list {
        None => None,
        Some(node) => Some(Rc::clone(node)),
    }
}

#[macro_export]
macro_rules! list {
    () => {
        None
    };

    ($first:expr $(, $rest:expr)*) => {
        cons($first, &list!($($rest),*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = cons(3, &cons(2, &cons(1, &None)));
        let s = to_string(&list1);
        println!("list1: {}", s);
        println!("Is 3 in the list? {}", member(3, &list1));
        println!("Is 8 in the list? {}", member(8, &list1));

        let list2 = cons(7, &list1);
        println!("list2: {}", to_string(&list2));
        println!("list1: {}", to_string(&list1));

        let list3 = cons(8, &cdr(&list1));
        println!("list3: {}", to_string(&list3));
        println!("list2: {}", to_string(&list2));
        println!("list1: {}", to_string(&list1));

        let list4 = cons(9, &cdr(&cdr(&list1)));
        println!();
        println!("list4: {}", to_string(&list4));
        println!("list3: {}", to_string(&list3));
        println!("list2: {}", to_string(&list2));
        println!("list1: {}", to_string(&list1));
        println!("Is 3 in list4? {}", member(3, &list4));
        println!("Is 8 in list4? {}", member(8, &list4));
        println!("Is 1 in list4? {}", member(1, &list4));
        println!("Is 9 in list4? {}", member(9, &list4));

        let list5 = list!(1, 2, 3, 100);
        println!("list5: {}", to_string(&list5));
        match car(&list5) {
            None => println!("car(&list5) = {:?}", None::<i32>),
            Some(x) => println!("car(&list5) = {}", x),
        }

        let list6 = cons(89, &cdr(&cdr(&cdr(&list5))));
        println!("list5: {}", to_string(&list5));
        println!("list6: {}", to_string(&list6));
    }
}
