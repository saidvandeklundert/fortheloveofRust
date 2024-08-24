///  cargo run -- --help
use clap::{arg, Command};
use termimad::crossterm::style::{Attribute::*, Color::*};
use termimad::*;
use termimad::crossterm::style::Color::*;
use termimad::*;

static MD_TABLE: &str = r#"
|:-:|:-:|-
|**feature**|**supported**|**details**|
|-:|:-:|-
| tables | yes | pipe based, with or without alignments
| italic, bold | yes | star based |
| inline code | yes | `with backquotes` (it works in tables too)
| code bloc | yes |with tabs or code fences
| syntax coloring | no |
| crossed text |  ~~not yet~~ | wait... now it works `~~like this~~`
| horizontal rule | yes | Use 3 or more dashes (`---`)
| lists | yes|* unordered lists supported
|  | |* ordered lists *not* supported
| quotes |  yes |> What a wonderful time to be alive!
| links | no | (but your terminal already handles raw URLs)
|-
*Run this example again in a terminal with a different width*
"#;
fn show(skin: &MadSkin, src: &str) {
    println!(" Raw       : {}", &src);
    println!(" Formatted : {}\n", skin.inline(src));
}

fn show_some(skin: &MadSkin) {
    show(skin, "*Hey* **World!** Here's `some(code)`");
    show(skin, "some *nested **style***");
}
fn main() {
    println!("\n");
    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fgbg(Magenta, rgb(30, 30, 40));
    skin.paragraph.align = Alignment::Center;
    skin.table.align = Alignment::Center;
    let (width, _) = terminal_size();
    let mut markdown = format!("Available width: *{}*", width);
    markdown.push_str(MD_TABLE);
    println!("{}", skin.term_text(&markdown));
    println!("\n");    
    println!();
    println!("\nWith the default skin:\n");
    let mut skin = MadSkin::default();
    show_some(&skin);
    println!("\nWith a customized skin:\n");
    skin.bold.set_fg(Yellow);
    skin.italic = CompoundStyle::with_bg(DarkBlue);
    skin.inline_code.add_attr(Reverse);
    show_some(&skin);

    let mut skin = MadSkin::default();
    skin.bold.set_fg(Yellow);
    skin.print_inline("*Hey* **World!** Here's `some(code)`");
    skin.paragraph.set_fgbg(Magenta, rgb(30, 30, 40));
    skin.italic.add_attr(Underlined);
    skin.italic.add_attr(OverLined);
    println!(
        "\nand now {}\n",
        skin.inline("a little *too much* **style!** (and `some(code)` too)")
    );    
    let matches = Command::new("SONiC Rust CLI")
        .version("1.0")
        .about("Display state of SONiC devices")
        .arg(arg!(--two <VALUE>).required(true))
        .arg(arg!(--one <VALUE>).required(true))
        .get_matches();

    println!(
        "two: {:?}",
        matches.get_one::<String>("two").expect("required")
    );
    println!(
        "one: {:?}",
        matches.get_one::<String>("one").expect("required")
    );
}