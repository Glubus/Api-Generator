// python_type.rhai
let value = params[0];
let format = if params.len() > 1 { params[1] } else { () };
let type = if params.len() > 2 { params[2] } else { () };

fn simple_type_to_python(value, format) {
    switch value {
        "integer" => "int",
        "number" => "float",
        "boolean" => "bool",
        "string" => if format == "date" { "datetime" } else { "str" },
    }
}

fn object_to_python(object_type) {
    object_type
}

if value == "array" {
    if (format == "object") {
        "List[" + object_to_python(type) + "]"
    } else {
        "List[" + simple_type_to_python(format, ()) + "]"
    }
} 
else if value == "object" {
    object_to_python(format)
} 
else {
    simple_type_to_python(value, format)
} 