let path = params[0];

fn convert_path_to_flask(path) {
    let parts = path.split("/");
    let result = [];
    

    for part in parts {
        if part.starts_with("{") && part.ends_with("}") {
            let param_name = part[1..-1];
            result.push("<int:" + param_name + ">");
        } else {
            result.push(part);
        }
    }
    
    result.reduce(|a, b|  a + "/" + b, "")[1..]
}

convert_path_to_flask(path)
