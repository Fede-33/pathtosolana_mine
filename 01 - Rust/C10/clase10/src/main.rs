fn longest<'a>(x:&'a str, y:&'a str) -> &'a str{
    if x.len() > y.len(){
        x
    } else{
        y
    }
}

struct Foo<'a> {
    x: &'a str,
}

fn main() {
    let s1: &'static str = "hola";
    let s2: &'static str = "Mundo";

    println!("{}", longest(s1, s2));
    println!("{}", longest("Hello", "World!"));
}
