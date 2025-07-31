struct X { val: i32 }
impl std::ops::Deref for X {
    type Target = i32;
    fn deref(&self) -> &i32 { &self.val }
}

trait M { fn m(self); }
impl M for i32   { fn m(self) { println!("i32::m()");  } }
impl M for X     { fn m(self) { println!("X::m()");    } }
impl M for &X    { fn m(self) { println!("&X::m()");   } }
impl M for &&X   { fn m(self) { println!("&&X::m()");  } }
impl M for &&&X  { fn m(self) { println!("&&&X::m()"); } }

trait RefM { fn refm(&self); }
impl RefM for i32  { fn refm(&self) { println!("i32::refm()");  } }
impl RefM for X    { fn refm(&self) { println!("X::refm()");    } }
impl RefM for &X   { fn refm(&self) { println!("&X::refm()");   } }
impl RefM for &&X  { fn refm(&self) { println!("&&X::refm()");  } }
impl RefM for &&&X { fn refm(&self) { println!("&&&X::refm()"); } }


struct Y { val: i32 }
impl std::ops::Deref for Y {
    type Target = i32;
    fn deref(&self) -> &i32 { &self.val }
}

struct Z { val: Y }
impl std::ops::Deref for Z {
    type Target = Y;
    fn deref(&self) -> &Y { &self.val }
}


#[derive(Clone, Copy)]
struct A;

impl M for    A { fn m(self) { println!("A::m()");    } }
impl M for &&&A { fn m(self) { println!("&&&A::m()"); } }

impl RefM for    A { fn refm(&self) { println!("A::refm()");    } }
impl RefM for &&&A { fn refm(&self) { println!("&&&A::refm()"); } }


fn main() {
    // I'll use @ to denote left side of the dot operator
    (*X{val:42}).m();        // i32::m()    , Self == @
    X{val:42}.m();           // X::m()      , Self == @
    (&X{val:42}).m();        // &X::m()     , Self == @
    (&&X{val:42}).m();       // &&X::m()    , Self == @
    (&&&X{val:42}).m();      // &&&X:m()    , Self == @
    (&&&&X{val:42}).m();     // &&&X::m()   , Self == *@
    (&&&&&X{val:42}).m();    // &&&X::m()   , Self == **@
    println!("-------------------------");

    (*X{val:42}).refm();     // i32::refm() , Self == @
    X{val:42}.refm();        // X::refm()   , Self == @
    (&X{val:42}).refm();     // X::refm()   , Self == *@
    (&&X{val:42}).refm();    // &X::refm()  , Self == *@
    (&&&X{val:42}).refm();   // &&X::refm() , Self == *@
    (&&&&X{val:42}).refm();  // &&&X::refm(), Self == *@
    (&&&&&X{val:42}).refm(); // &&&X::refm(), Self == **@
    println!("-------------------------");

    Y{val:42}.refm();        // i32::refm() , Self == *@
    Z{val:Y{val:42}}.refm(); // i32::refm() , Self == **@
    println!("-------------------------");

    A.m();                   // A::m()      , Self == @
    // without the Copy trait, (&A).m() would be a compilation error:
    // cannot move out of borrowed content
    (&A).m();                // A::m()      , Self == *@
    (&&A).m();               // &&&A::m()   , Self == &@
    (&&&A).m();              // &&&A::m()   , Self == @
    A.refm();                // A::refm()   , Self == @
    (&A).refm();             // A::refm()   , Self == *@
    (&&A).refm();            // A::refm()   , Self == **@
    (&&&A).refm();           // &&&A::refm(), Self == @
}
