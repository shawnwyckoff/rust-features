fn main() {
    let x = "out";

    {
        let x = "in";
        println!("{}", x);
    }

    println!("{}", x);

    println!("code in {...} has different own scope with outside codes")
}