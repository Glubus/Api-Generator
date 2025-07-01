// rust_type.rhai
let value = params[0];
let format = if params.len() > 1 { params[1] } else { () };
let type = if params.len() > 2 { params[2] } else { () };


fn simple_type_to_rust(value,format) {
    switch value {
        "integer" => "i32",
        "number" => "f64",
        "boolean" => "bool",
        "string" => if format == "date" { "chrono::NaiveDate" } else { "String" },
    }
}

fn object_to_rust(object_type) {
    object_type  
}

if value == "array" {
    if (format == "object") {
        "Vec<" + object_to_rust(type) + ">"
    } else {
        "Vec<" + simple_type_to_rust(format,()) + ">"
    }
} 
else if value == "object" {
    object_to_rust(format)
} 
else
 {
    simple_type_to_rust(value,format)
}
