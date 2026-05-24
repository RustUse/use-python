use use_python_value::PythonPrimitiveValue;

fn main() {
    let value = PythonPrimitiveValue::Int(String::from("12345678901234567890"));

    assert_eq!(value.type_name(), "int");
    assert!(value.is_numeric());
    assert!(value.is_truthy_like());
}
