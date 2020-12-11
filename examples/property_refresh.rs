use android_properties::setprop;

const HELLO_WORLD_PROPERTY: &str = "hello.world";

fn main() {
    setprop(HELLO_WORLD_PROPERTY, "initial value").expect("Cannot set android property");
    let mut hello_world = android_properties::getprop(HELLO_WORLD_PROPERTY);
    println!("Initial property: {}", hello_world);
    
    setprop(HELLO_WORLD_PROPERTY, "refreshed value").expect("Cannot set android property");
    hello_world.refresh();
    println!("Refreshed property: {}", hello_world);
}
