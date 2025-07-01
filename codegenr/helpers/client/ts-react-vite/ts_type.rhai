// rust_type.rhai
let value = params[0];
let format = if params.len() > 1 { params[1] } else { () };
let type = if params.len() > 2 { params[2] } else { () };


fn simple_type_to_ts(value,format) {
    switch value {
        "integer" => "number",
        "number" => "number",
        "boolean" => "boolean",
        "string" => if format == "date" { "Date" } else { "string" },
    }
}

fn object_to_ts(object_type) {
    object_type  
}

if value == "array" {
    if (format == "object") {
        "Array<" + object_to_ts(type) + ">"
    } else {
        "Array<" + simple_type_to_ts(format,()) + ">"
    }
} 
else if value == "object" {
    object_to_ts(format)
} 
else
 {
    simple_type_to_ts(value,format)
}
