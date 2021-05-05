use std::env;

fn main() {
    println!("<ARGS>");
    for arg in env::args() {
        println!("{}", arg);
    }
    println!("</ARGS>");
    println!("<ENV>");
    for env_var in env::vars() {
        println!("{}={}", env_var.0, env_var.1);
    }
    println!("</ENV>");
}
