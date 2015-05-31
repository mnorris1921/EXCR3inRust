fn main() {
   let mut x = returns_an_int();
}

fn returns_an_int() {
   4
}

#[test]
fn this_tests_code() {
   if returns_an_int() != 3 {
      panic!("Fail!");
   }
}


/*
fn div_by_three(num: int) -> bool {
   true
}

#[test]
fn test_div_by_three() {
    if div_by_three(1) {
        fail!("One is not three");
    }
}*/
