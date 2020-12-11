const HELLO_WORLD_PROPERTY: &str = "hello.world";
fn main() {
    let hello_world = android_properties::getprop(HELLO_WORLD_PROPERTY);
    match hello_world.value {
        Some(ref _value) => println!("{}", hello_world),
        None => println!("Property {} not found", hello_world.name)
    };
}
