use android_properties::{AndroidProperties, AndroidProperty};

const HELLO_WORLD_PROPERTY: &str = "hello.world";

fn main() {
    let properties = AndroidProperties::new();
    let hello_world = AndroidProperty::new(HELLO_WORLD_PROPERTY);
    properties.set(&hello_world, "bonjour").unwrap();

    match properties.get(&hello_world) {
        Some(value) => println!("{:?}", value),
        None => println!("Property {} not found", hello_world.name()),
    };
}
