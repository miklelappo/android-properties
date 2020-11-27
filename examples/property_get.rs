const HELLO_WORLD_PROPERTY: &str = "hello.world";
fn main() {
    let hello_world = android_properties::getprop(HELLO_WORLD_PROPERTY).unwrap_or("hello".to_string());
}
