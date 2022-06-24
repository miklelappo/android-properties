use android_properties::{AndroidProperties, AndroidProperty};

const HELLO_WORLD_PROPERTY: &str = "hello.world";

fn main() {
    let properties = AndroidProperties::new();

    properties.set_property(HELLO_WORLD_PROPERTY, "initial value").expect("Cannot set android property");

    let hello_world = AndroidProperty::new(HELLO_WORLD_PROPERTY);
    println!("Initial property: {:?}", properties.get(&hello_world));

    properties.set_property(HELLO_WORLD_PROPERTY, "refreshed value").expect("Cannot set android property");
    println!("Refreshed property: {:?}", properties.get(&hello_world));
}
