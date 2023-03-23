use clap::{App, Arg};
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let app = App::new("SearchR CLI")
        .version("1.0")
        .author("Simon-Olivier Desautels")
        .about("A cli for creating the command use to rename all the reference of url in a wordpress DB")
        .arg(Arg::new("old_url").required(true).index(1))
        .arg(Arg::new("new_url").required(true).index(2))
        .get_matches();

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let new_url = String::from(app.value_of("new_url").unwrap_or(""));
    let old_url = String::from(app.value_of("old_url").unwrap_or(""));
    let template = format!(
        "wp search-replace \"{}\" \"{}\" --all-tables --verbose --precise --dry-run",
        old_url, new_url
    );

    ctx.set_contents(template.to_owned()).unwrap();
}
