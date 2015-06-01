
/*struct binOpC {
   op: String,
   left: Option<Box<EXCR3>>,
   right: Option<Box<EXCR3>>
}

struct ifC {
   test: Option<Box<EXCR3>>,
   then: Option<Box<EXCR3>>,
   els: Option<Box<EXCR3>>
}

struct appC {
   fun: Option<Box<EXCR3>>,
   args: Vec<Option<Box<EXCR3>>>
}

struct lamC {
   body: Option<Box<EXCR3>>,
   args: Vec<String>
}*/

enum EXCR3 {
    i32,
    binOpC { op: String, left: Option<Box<EXCR3>>, right: Option<Box<EXCR3>> },
    bool,
    ifC { test: Option<Box<EXCR3>>, then: Option<Box<EXCR3>>, els: Option<Box<EXCR3>> },
    String,
    appC { fun: Option<Box<EXCR3>>, args: Vec<Option<Box<EXCR3>>> },
    lamC { body: Option<Box<EXCR3>>, args: Vec<String> }
}

// Michael: nums, with
// Varsha: bool, binop

struct Binding {
    name : String,
    val: Option<Box<Value>>
}
struct Environment {
   Env : Vec<Option<Box<Binding>>>
}

enum Value {
    numV { n : i32},
    boolV { b : bool},
    closV { args :  Vec<String>, body : Option<Box<EXCR3>>, env : Option<Box<Environment>>}
}

fn main() {
   println!("Hello, world!");
}



