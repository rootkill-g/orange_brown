fn make_tester<'a>(ans: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |challenge| challenge == ans
}

fn main() {
    let test = make_tester("this_should_match");
    println!("{}", test("*****************"));
    println!("{}", test("hunter_going_wild"));
    println!("{}", test("this_should_match"));
}
