use rocket::http::hyper::uri;
use rocket::{*, response::content::RawCss};

use rocket::response::content::RawHtml;
use minijinja::{Environment, context};


#[get("/")]
pub fn index() -> RawHtml<String>{
    println!("Hello world");
    let mut env = Environment::new();
    env.add_template("hello.txt", "{{name}}").unwrap();
    let template = env.get_template("hello.txt").unwrap();
    let render = template.render(context! { name => "World" }).unwrap();
    return RawHtml(render);
}
