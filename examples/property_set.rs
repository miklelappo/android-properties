use android_properties::AndroidProperties;

fn main() {
    let properties = AndroidProperties::new();
    properties.set_property("hello.world", "hello").expect("Cannot set android property");
}
