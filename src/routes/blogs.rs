use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;
use std::collections::HashMap;

use utils;
use db_conn::DbConn;
use models::blogs::*;
use models::instance::Instance;
use models::user::User;

#[get("/~/<name>")]
fn details(name: String) -> String {
    format!("Welcome on ~{}", name)
}

#[get("/blogs/new")]
fn new(_user: User) -> Template {
    Template::render("blogs/new", HashMap::<String, i32>::new())
}

#[derive(FromForm)]
struct NewBlogForm {
    pub title: String
}

#[post("/blogs/new", data = "<data>")]
fn create(conn: DbConn, data: Form<NewBlogForm>, _user: User) -> Redirect {
    let inst = Instance::get_local(&*conn).unwrap();
    let form = data.get();
    let slug = utils::make_actor_id(form.title.to_string());

    Blog::insert(&*conn, NewBlog {
        actor_id: slug.to_string(),
        title: form.title.to_string(),
        summary: String::from(""),
        outbox_url: Blog::compute_outbox(slug.to_string(), inst.public_domain.to_string()),
        inbox_url: Blog::compute_inbox(slug.to_string(), inst.public_domain.to_string()),
        instance_id: inst.id
    });
    
    Redirect::to(format!("/~/{}", slug).as_str())
}
