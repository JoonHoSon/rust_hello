#[derive(Debug)]
struct Test {
    x: i8,
    y: i8
}

macro_rules! last_element {
    ($v: ident) => (
        $v[$v.len() - 1]
    )
}

fn main() {
    let mut message = 50;

    println!("Hello, world! => {}", message);

    message = test(message);

    println!("change variables value => {}", message);

    // assert_eq!(convert_string, "51");

    println!("use stringify => {}", test(message));

    assert_eq!(51, message);

    return_none(message);

    let st: Test = Test {x: 1, y: 2};
    let t = "Hello, world!";

    println!("x : {}, y : {}", st.x, st.y);
    println!("{} length => {}", t, t.len());

    let mut count = 0;

    loop {
        // println!("{}", count);

        count += 1;

        if count >= 1000 {
            break;
        }
    }

    let func: fn(i8) -> i8 = test;

    println!("call named function => {}", func(22));

    let mut v: Vec<i8> = vec![1, 2, 4];

    v[0] = v[0] + 1;

    println!("------------------------------------------");
    println!("before length : {}", v.len());
    println!("before v[0] : {}", v[0]);
    println!("before v[1] : {}", v[1]);
    println!("before last item : {}", last_element!(v));

    take(&mut v);

    println!("after v[0] : {}", v[0]);
    println!("after v[1] : {}", v[1]);
    println!("after length : {}", v.len());
    println!("after last item : {}", last_element!(v));

    // println!("v[0] is : {}", v[0]);
}

fn test(x: i8) -> i8 {
    x + 1
}

fn return_none(x: i8) -> () {
    println!("return none => {}", x);
}

fn take(v: &mut Vec<i8>) -> () {
    println!("in take function >> {}", v[0]);
    v[0] = v[0] + 1;
    v[1] = v[1] * 3;
    v.push(100);
}