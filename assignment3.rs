
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
    numC {n: i32},
    binOpC {op: String, left: Option<Box<EXCR3>>, right: Option<Box<EXCR3>>},
    boolC {b: bool},
    ifC {test: Option<Box<EXCR3>>, then: Option<Box<EXCR3>>, els: Option<Box<EXCR3>>},
    idC {id: String},
    appC {fun: Option<Box<EXCR3>>, args: Vec<Option<Box<EXCR3>>>},
    lamC {body: Option<Box<EXCR3>>, args: Vec<String>}
}

// Michael: nums, with
// Varsha: bool, binop
// Savannah: String (symbol ids), ifC

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
    closV { args :  Vec<String>, body : Option<Box<EXCR3>>, env : Option<Box<Environment>>},
    tempV 
}

fn interp(e: EXCR3, env: Environment) -> Value {
   match e {
      EXCR3::numC {n} => return Value::numV {n: n},
      EXCR3::binOpC {op, left, right} => Value::tempV,
      EXCR3::boolC {b} => return Value::boolV {b: b},
      EXCR3::ifC {test, then, els} => Value::tempV,
      EXCR3::idC {id} => Value::tempV,
      EXCR3::appC {fun, args} => Value::tempV,
      EXCR3::lamC {body, args} => Value::tempV,
   }
}

fn test_suite() {
   // Something weird with Rust comparing equality of structs, will figure out tomorrow...
   //assert_eq!(interp(EXCR3::numC{n: 4}, Environment{Env: Vec::new()}), Value::numV{n: 4});
}

fn main() {
   test_suite();
}

