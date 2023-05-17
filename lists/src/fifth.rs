//----------------------------------------------------------------
// SELF REFERENCING SOLUTION
//----------------------------------------------------------------
// Each part of the code is technically correct in isolation (we can call push and pop once) but then the absurdity of what we created takes affect and everything just locks up.

// I'm sure there is some use for what we've written, but as far as I'm concerned it's just syntatically valid gibberish. We're saying we contain something with lifetime 'a, and that push and pop borrows self for that lifetime. That's weird but Rust can look at each part of our code individually and it doesn't see any rules being broken.

// But as soon as we try to actually use the list, the compiler quickly goes "yep you've borrowed self mutably for 'a, so you can't use self anymore until the end of 'a" but also "because you contain 'a, it must be valid for the entire list's existence".

// It's nearly a contradiction but there is one solution: as soon as you push or pop, the list "pins" itself in place and can't be accessed anymore. It has swallowed its own proverbial tail, and ascended to a world of dreams.

// error[E0499]: cannot borrow `list` as mutable more than once at a time
//   --> src/fifth.rs:78:9
//    |
// 75 |         assert_eq!(list.pop(), None);
//    |                    ---------- first mutable borrow occurs here
// ...
// 78 |         list.push(1);
//    |         ^^^^^^^^^^^^
//    |         |
//    |         second mutable borrow occurs here
//    |         first borrow later used here

// pub struct List<'a, T> {
//     head: Link<T>,
//     tail: Option<&'a mut Node<T>>,
// }

// type Link<T> = Option<Box<Node<T>>>;

// struct Node<T> {
//     elem: T,
//     next: Link<T>,
// }

// impl<'a, T> Default for List<'a, T> {
//     fn default() -> Self {
//         List::new()
//     }
// }

// impl<'a, T> List<'a, T> {
//     pub fn new() -> Self {
//         List {
//             head: None,
//             tail: None,
//         }
//     }

//     pub fn push(&'a mut self, elem: T) {
//         let new_tail = Box::new(Node {
//             elem,
//             // When you push onto the tail, your next is always None
//             next: None,
//         });

//         let new_tail = match self.tail.take() {
//             Some(mut old_tail) => {
//                 // If the old tail existed, update it to point to the new tail
//                 old_tail.next = Some(new_tail);
//                 old_tail.next.as_deref_mut()
//             }
//             None => {
//                 // Otherwise, update the head to point to it
//                 self.head = Some(new_tail);
//                 self.head.as_deref_mut()
//             }
//         };

//         self.tail = new_tail
//     }

//     pub fn pop(&'a mut self) -> Option<T> {
//         // Grab the list's current head
//         self.head.take().map(|head| {
//             let head = *head;
//             self.head = head.next;

//             // If we're out of `head`, make sure to set the tail to `None`.
//             if self.head.is_none() {
//                 self.tail = None;
//             }

//             head.elem
//         })
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::List;

//     #[test]
//     fn basics() {
//         let mut list = List::default();

//         // Check empty list behaves right
//         assert_eq!(list.pop(), None);

//         // Populate list
//         list.push(1);
//         list.push(2);
//         list.push(3);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(1));
//         assert_eq!(list.pop(), Some(2));

//         // Push some more just to make sure nothing's corrupted
//         list.push(4);
//         list.push(5);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(3));
//         assert_eq!(list.pop(), Some(4));

//         // Check exhaustion
//         assert_eq!(list.pop(), Some(5));
//         assert_eq!(list.pop(), None);
//     }
// }
//----------------------------------------------------------------
// END SELF REFERENCING SOLUTION
//----------------------------------------------------------------
