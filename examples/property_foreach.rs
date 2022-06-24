use android_properties::AndroidProperties;

fn main() {
    let properties = AndroidProperties::new();
    for property in properties.property_values() {
        println!("{:?}", properties.get(&property));
    }
}
