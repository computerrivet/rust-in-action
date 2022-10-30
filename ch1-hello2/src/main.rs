fn main() {
    println!("Hello, world!");
    greet_world();
}

fn greet_world() {
    print!("Hello World");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions {
        println!("{}", &region)
    }
}
