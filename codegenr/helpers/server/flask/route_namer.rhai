let val = params[0];

let parts = val.split("/");

let name = "";

for part in parts {
    if part.starts_with("{") {
        let part_name = part[1..-1];
        part_name[0].make_lower();
        name += "by_" + part_name + "_";
    } else if part != "" {
        part[0].make_lower();
        name += part+"_";
    }
}

name[0..-1]