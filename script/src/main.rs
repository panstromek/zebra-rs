use regex::{Captures, Regex};

fn main() {
    let filename = "../engine/src/zebra.rs";
    let file = std::fs::read_to_string(filename).unwrap();
    let declaration = regex::Regex::new(
        r#"static\s+mut\s+([\w_]+):(.*?\s*?)=(.*?);\s*$"#).unwrap();
    #[derive(Debug)]
    struct Declaration<'a> {
        name: &'a str,
        type_: &'a str,
        default_value: &'a str,
    }

    let declarations = file.lines()
        .filter_map(|line| declaration
            .captures(line)
            .map(|captures| {
                Declaration {
                    name: captures.get(1).unwrap().as_str().trim(),
                    type_: captures.get(2).unwrap().as_str().trim(),
                    default_value: captures.get(3).unwrap().as_str().trim(),
                }
            }))
        // .inspect(|x| println!("{:#?}", x))
        .collect::<Vec<_>>();


    let needle = "(\\b)(".to_string() +
        &declarations.iter().map(|decl| decl.name).collect::<Vec<_>>().join("|") +
        ")(\\b)";
    let replacer = regex::Regex::new(&needle).unwrap();
    let mut struct_declaration = declarations
        .iter()
        .fold(("pub struct Config {".to_string(), "pub static mut config : Config = Config {".to_string()),
              |(struct_decl, static_decl), declaration| {
                  (struct_decl + "\npub " + declaration.name + ": " + declaration.type_ + ",",
                   static_decl + "\n" + declaration.name + ": " + declaration.default_value + ",")
              });
    struct_declaration.0 += "\n}\n";
    struct_declaration.1 += "\n};\n";

    let new_lines = replace_in_file(&file, &declaration, &replacer, &mut struct_declaration, true);
    std::fs::write(filename, new_lines.join("\n")).unwrap();

    let file = std::fs::read_to_string("../legacy-zebra/src/zebra.rs").unwrap();

    let new_lines = replace_in_file(&file, &declaration, &replacer, &mut struct_declaration, false);
    std::fs::write("../legacy-zebra/src/zebra.rs", new_lines.join("\n")).unwrap();
}

fn replace_in_file(file: &String, declaration: &Regex, replacer: &Regex, struct_declaration: &mut (String, String), shoudl_write_decl: bool) -> Vec<String> {
    let mut decl_written = false;
    let mut in_multiline_comment = false;
    let new_lines = file.lines()
        .map(|line| {
            let is_use_statament = line.trim_start().starts_with("use ");

            if line.contains("/*") {
                in_multiline_comment = true;
            }
            if line.contains("*/") {
                in_multiline_comment = false;
                return line.to_string();
            }
            let is_comment = line.trim_start().starts_with("//");
            if is_comment || in_multiline_comment {
                return line.to_string()
            }
            if declaration.is_match(line) {
                return if decl_written || !shoudl_write_decl {
                    "".to_string()
                } else {
                    decl_written = true;
                    struct_declaration.0.clone() + &struct_declaration.1
                }
            }
            line.split("\"").enumerate().map(|(i, item )| {
                if i % 2 == 0 {
                    // not a string
                    replacer.replace_all(item, |captures: &Captures| {
                        if is_use_statament {
                            // TODO find and replace comma
                            return "".to_string();
                        }
                        return captures[1].to_string() + "config." + &captures[2] + &captures[3];
                    }).to_string()
                } else {
                    item.to_string()
                }
            }).collect::<Vec<_>>().join("\"")
        })
        // .inspect(|x| println!("{}", x))
        .collect::<Vec<_>>();
    new_lines
}
