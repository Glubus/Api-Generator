

let val = params[0];
let parts = val.split("/");

let result = "";

for part in parts {
    if part.contains("{") {
        let t_part = "$"+part+"/";
        result += t_part;
    } else {
        result += part+"/";
    }
}

result[0..-1]