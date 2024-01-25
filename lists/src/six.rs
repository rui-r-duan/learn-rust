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
