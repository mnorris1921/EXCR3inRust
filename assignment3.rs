
struct binOpC {
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
}

struct EXCR3 {
    numC: i32,
    binOpC: Option<Box<binOpC>>,
    boolC: bool,
    ifC: Option<Box<ifC>>,
    idC: String,
    appC: Option<Box<appC>>,
    lamC: Option<Box<lamC>>
}

fn main() {
   println!("Hello, world!");
}



