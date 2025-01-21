use auto_test_macro::auto_test_dispatchable;

#[derive(Debug)]
struct CustomizedType {
    x: i32,
    y: i32,
}

#[auto_test_dispatchable((1), (2), (3), & true)]
#[auto_test_dispatchable((-11), (-2), (-3), & false)]
fn single_arg_function(x: i32) -> Result<(), &'static str> {
    if x > 0 {
        Ok(())
    } else {
        Err("Input must be positive")
    }
}

#[auto_test_dispatchable((1, "hello"), & true)]
#[auto_test_dispatchable((-1, ""), & false)]
fn multi_arg_function(x: i32, s: &str) -> Result<(), &'static str> {
    if x > 0 && !s.is_empty() {
        Ok(())
    } else {
        Err("Invalid input")
    }
}

#[auto_test_dispatchable((CustomizedType { x: 1, y: 2 }), & true)]
#[auto_test_dispatchable((CustomizedType { x: -1, y: -2 }), & false)]
fn customized_type_function(x: CustomizedType) -> Result<(), &'static str> {
    if x.x > 0 && x.y > 0 {
        Ok(())
    } else {
        Err("Invalid input")
    }
}