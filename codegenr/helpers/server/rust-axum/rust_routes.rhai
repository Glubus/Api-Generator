let r = params[0];
let r_name = params[1];
let r_name_stripped = params[2];
r_name_stripped.replace("{", "");
r_name_stripped.replace("}", "");
r_name_stripped.replace("/", "_");
let routes = [];

if r.get != () {
    routes.push(".route(\"" + r_name + "\", get(get" + r_name_stripped + "))");
}
if r.post != () {
    routes.push(".route(\"" + r_name + "\", post(post" + r_name_stripped + "))");
}
if r.put != () {
    routes.push(".route(\"" + r_name + "\", put(put" + r_name_stripped + "))");            
}
if r.delete != () {
    routes.push(".route(\"" + r_name + "\", delete(delete" + r_name_stripped + "))");
}

routes
