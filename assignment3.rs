
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

use std::convert::AsRef;

enum Operator {
   plus,
   mult,
   div,
   sub,
   leq,
   eq
}

enum EXCR3 {
    numC {n: i32},
    binOpC {op: Operator, left: Box<EXCR3>, right: Box<EXCR3>},
    //boolC {b: bool},
    //ifC {test: Option<Box<EXCR3>>, then: Option<Box<EXCR3>>, els: Option<Box<EXCR3>>},
    //idC {id: String},
    //appC {fun: Option<Box<EXCR3>>, args: Vec<Option<Box<EXCR3>>>},
    //lamC {body: Option<Box<EXCR3>>, args: Vec<String>}
}

/*impl PartialEq for EXCR3 {
   fn eq(&self, other: &EXCR3) -> bool {
      true
   }
}*/

struct Binding {
    name : String,
    val: Box<Value>
}

struct Environment {
   Env : Vec<Box<Binding>>
}

enum Value {
    numV { n : i32},
    //boolV { b : bool},
    //closV { args :  Vec<String>, body : Option<Box<EXCR3>>, env : Option<Box<Environment>>}, 
}

fn interp(e: EXCR3) -> i32 {
   match e {
      EXCR3::numC {n} => n,
      EXCR3::binOpC {op: Operator::plus, left, right} => 
         interp(*left) + interp(*right),
      EXCR3::binOpC {op: Operator::mult, left, right} => 
         interp(*left) * interp(*right),
      EXCR3::binOpC {op: Operator::div, left, right} => 
         interp(*left) / interp(*right),
      EXCR3::binOpC {op: Operator::sub, left, right} => 
         interp(*left) - interp(*right),
      EXCR3::binOpC {op: Operator::leq, left, right} => 
         //interp(*left) + interp(*right)
         0,
      EXCR3::binOpC {op: Operator::eq, left, right} => 
         //interp(*left) + interp(*right)
         0
   }
}

/*fn parse(s: String) -> EXCR3 {
   match s {
      Some(number) => EXCR3::numC {n: number},
      None => EXCR3::idC {id: s}
   }
}*/

fn test_suite() {
   // Something weird with Rust comparing equality of structs, will figure out tomorrow...
   assert_eq!(interp(EXCR3::numC{n: 4}), 4);
   //assert_eq!(interp(EXCR3::binOpC{op: "+", left: EXCR3::numC{n: 2}, right: EXCR3::numC{n: 2}},
           //Environment{Env: Vec::new()}), 4);
}

fn main() {
   test_suite();
}

