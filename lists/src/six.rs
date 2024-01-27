use std::{marker::PhantomData, ptr::NonNull};

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<T>,
}

//----------------------------------------------------------------
// *mut Node<T> is invariant, so Link<T> is invariant, so
// LinkedList<T> is invariant.
//----------------------------------------------------------------
//type Link<T> = *mut Node<T>;

//----------------------------------------------------------------
// Option<<NonNull<Node<T>>> is covariant, so Link<T> is covariant, so
// LinkedList<T> is covariant.
// Why do we need LinkedList<T> to be covariant?

// Because sometimes T can be a reference, say &'small S.  If there is
// a function parameter of type LinkedList<&'small S>, then we hope
// that the parameter can accept an argument who lives longer than
// 'small, say LinkedList<&'big S>.  In Rust, &'big S is treated as a
// subtype of &'small S.  If LinkedList<T> is invariant, the subtyping
// property cannot be preserved, it means that LinkedList<&'big S> is
// not a subtype of LinkedList<&'small S>.  Only when LinkedList<T> is
// covariant, can Rust treat LinkedList<&'big S> as a subtype of
// LinkedList<&'small S>.
//----------------------------------------------------------------
type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));
            if let Some(old) = self.front {
                // Put the new front before the old one
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            } else {
                // If there's no front, then we're the empty list and need
                // to set the back too.  Also here's some integrity checks
                // for testing, in case we mess up.
                debug_assert!(self.back.is_none());
                debug_assert!(self.front.is_none());
                debug_assert!(self.len == 0);
                self.back = Some(new);
            }
            self.front = Some(new);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            // Only have to do stuff if there is a front node to pop.
            // Note that we don't need to mess around with `take` anymore
            // because everything is Copy and there are no dtors that will
            // run if we mess up ... right? :) Riiiight? :)))
	    //
	    // --> No, it is wrong!  Boxed_node's dtor will be run if
	    // the debug_assert line panics or the self.len -= 1 line panics!
	    //
	    // To maintain the PANIC SAFETY, we must make sure that
	    // the possible panic points are at the very start or very end of
	    // the function, ensuring that the invariance-breaking code block
	    // is in the middle and does not panic.
            self.front.map(|node| {
                // Bring the Box back to life so we can move out its value and
                // Drop it (Box continues to magically understand this for us).
                let boxed_node = Box::from_raw(node.as_ptr()); // It can panic!
                let result = boxed_node.elem;

                // Make the next node into the new front.
                self.front = boxed_node.back;
                if let Some(new) = self.front {
                    // Cleanup its reference to the removed node
                    (*new.as_ptr()).front = None;
                } else {
                    // If the front is now null, then this list is now empty!

		    // The debug_assert! line can panic, and leave self.back
		    // still pointing to the freed boxed_node, and this could
		    // result in use-after-free!
		    // We can move the assert line after self.back = None,
		    // but a better way is just remove the assert line.

                    // debug_assert!(self.len == 1); // It can panic!

                    self.back = None;
                }

                self.len -= 1;	// It can panic!
                result
                // Box gets implicitly freed here, knows there is no T.
            })
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }
}
