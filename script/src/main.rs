use regex::{Captures, Regex};
use std::process::{Command};

fn main() {
    let filename = "../engine/flip/unflip.rs";
    let struct_name = "FlipStack___";
    let global_name = "flip_stack___";

    let file = std::fs::read_to_string(filename).unwrap();
    let declaration = Regex::new(
        r#"static\s+mut\s+([\w_]+):(.*?\s*?)=(.*?);\s*$"#).unwrap();
    #[derive(Debug)]
    struct Declaration<'a> {
        name: &'a str,
        type_: &'a str,
        default_value: &'a str,
        is_pub: bool
    }

    let declarations = file.lines()
        .filter_map(|line| declaration
            .captures(line)
            .map(|captures| {
                Declaration {
                    name: captures.get(1).unwrap().as_str().trim(),
                    type_: captures.get(2).unwrap().as_str().trim(),
                    default_value: captures.get(3).unwrap().as_str().trim(),
                    is_pub: line.trim_start().starts_with("pub")
                }
            }))
        // .inspect(|x| println!("{:#?}", x))
        .collect::<Vec<_>>();

    //fixme don't search fo non-pub stuff in other files
    let orig_file_needle = "(\\b)(".to_string() +
        &declarations.iter().map(|decl| decl.name).collect::<Vec<_>>().join("|") +
        ")(\\b)";

    let other_file_needle = "(\\b)(".to_string() +
        &declarations.iter()
            .filter_map(|decl| if decl.is_pub { Some(decl.name) } else { None })
            .collect::<Vec<_>>()
            .join("|") + ")(\\b)";

    let replacer = Regex::new(&orig_file_needle).unwrap();
    let replacer_for_other_files = Regex::new(&other_file_needle).unwrap();
    let mut struct_declaration = declarations
        .iter()
        .fold((format!("pub struct {} {{", struct_name),
               format!("pub static mut {} : {} = {} {{",global_name, struct_name, struct_name )),

              |(struct_decl, static_decl), declaration| {
                  (struct_decl + (if declaration.is_pub { "\npub " } else { "\n " }) + declaration.name + ": " + declaration.type_ + ",",
                   static_decl + "\n" + declaration.name + ": " + declaration.default_value + ",")
              });
    struct_declaration.0 += "\n}\n";
    struct_declaration.1 += "\n};\n";

    let multi_comma = Regex::new(r#",(?:\s+,)+"#).unwrap();

    let collector = Command::new("sh")
        .arg("-c")
        .arg((declarations.iter().map(|decl| {
            return String::from("(") + decl.name + ")";
        }).fold(String::from("rg \"(_______)"), |acc, name| {
            acc + "|" + &name
        }) + "\" ../ --files-with-matches").as_str())
        .output()
        .unwrap()
        .stdout;
    let rg_output = String::from_utf8(collector).unwrap();
    let new_lines = replace_in_file(&file, &declaration, &replacer,
                                    &mut struct_declaration,
                                    true, global_name, true, &multi_comma);

    std::fs::write(filename, new_lines.join("\n")).unwrap();
    let usages_file_paths = rg_output.lines();
    for usages_file_path in usages_file_paths {
        if usages_file_path == filename || usages_file_path == "../script/src/main.rs" {
            continue;
        }
        if let Ok(file) = std::fs::read_to_string(usages_file_path) {
            let new_lines = replace_in_file(&file, &declaration, &replacer_for_other_files,
                                            &mut struct_declaration,
                                            false, global_name,
                                            false, &multi_comma);
            std::fs::write(usages_file_path, new_lines.join("\n")).unwrap();
        } else {
            eprintln!("file doesn't exist '{}'", usages_file_path)
        }
    }
}

fn replace_in_file(file: &String, declaration: &Regex, replacer: &Regex,
                   struct_declaration: &mut (String, String), shoudl_write_decl: bool,
                   global_name: &str, should_erase_decl:bool, multi_comma: &Regex
) -> Vec<String> {
    // FIXME check for redeclarations of the same symbol as a local

    let mut decl_written = false;
    let mut in_multiline_comment = false;
    let mut new_lines = file.lines()
        .map(|line| {
            let is_use_statament = line.trim_start().starts_with("use ");

            // these technically don't solve all problem but whatever
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
            if declaration.is_match(line) && should_erase_decl {
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
                    let replaced = replacer.replace_all(item, |captures: &Captures| {
                        if is_use_statament {
                            // TODO find and replace comma
                            return "".to_string();
                        }
                        return captures[1].to_string() + global_name +"." + &captures[2] + &captures[3];
                    });
                    multi_comma.replace_all(replaced.as_ref(), |_:&Captures| {
                      return ","
                    }).to_string()
                } else {
                    item.to_string()
                }
            }).collect::<Vec<_>>().join("\"")
        })
        // .inspect(|x| println!("{}", x))
        .collect::<Vec<_>>();
    if !new_lines.last().unwrap().trim().is_empty() {
        new_lines.push("".into());
    }
    new_lines
}
