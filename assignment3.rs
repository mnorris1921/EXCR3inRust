
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

fn main() {
   println!("Hello, world!");
}



