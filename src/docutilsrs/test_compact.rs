use docutilsrs::{html5, parse_rst, Doctree};
use docutilsrs::cli::{CommonOptions, Html5Options};
fn main() {
    let rst = r#"
* item 1
* item 2
"#;
    let tree = parse_rst(rst);
    let html = html5(&tree, &Html5Options::default(), &CommonOptions::default());
    println!("{}", html);
}
