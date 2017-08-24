mod lib;

#[macro_use]
mod mac;

#[derive(Debug)]
struct Test {
    x: i8,
    y: i8
}

fn main() {
    let mut message = 50;

    println!("Hello, world! => {}", message);
    println!("named placeholder => {holder}", holder="Wow!");

    message = lib::test(message);

    println!("change variables value => {}", message);

    // assert_eq!(convert_string, "51");

    println!("use stringify => {}", lib::test(message));

    assert_eq!(51, message);

    lib::return_none(message);

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

    let func: fn(i8) -> i8 = lib::test;

    println!("call named function => {}", func(22));

    let mut v: Vec<i8> = vec![1, 2, 4];

    v[0] = v[0] + 1;

    println!("------------------------------------------");
    println!("before length : {}", v.len());
    println!("before v[0] : {}", v[0]);
    println!("before v[1] : {}", v[1]);
    println!("before last item : {}", last_element!(v));

    lib::take(&mut v);

    println!("after v[0] : {}", v[0]);
    println!("after v[1] : {}", v[1]);
    println!("after length : {}", v.len());
    println!("after last item : {}", last_element!(v));

    // println!("v[0] is : {}", v[0]);
}