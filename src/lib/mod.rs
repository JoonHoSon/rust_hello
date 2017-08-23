pub fn test(x: i8) -> i8 {
    x + 1
}

pub fn return_none(x: i8) -> () {
    println!("return none => {}", x);
}

pub fn take(v: &mut Vec<i8>) -> () {
    println!("in take function >> {}", v[0]);
    v[0] = v[0] + 1;
    v[1] = v[1] * 3;
    v.push(100);
}