use android_properties::AndroidProperty;

fn main() {
    for property in android_properties::prop_values() {
        println!("{}: {}", property.name, property.value);
    }
}
