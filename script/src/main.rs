use regex::{Captures, Regex};

fn main() {
    let filename = "../engine/src/globals.rs";
    let struct_name = "BoardState";
    let global_name = "board_state";

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
        .fold((format!("pub struct {} {{", struct_name),
               format!("pub static mut {} : {} = {} {{",global_name, struct_name, struct_name )),

              |(struct_decl, static_decl), declaration| {
                  (struct_decl + "\npub " + declaration.name + ": " + declaration.type_ + ",",
                   static_decl + "\n" + declaration.name + ": " + declaration.default_value + ",")
              });
    struct_declaration.0 += "\n}\n";
    struct_declaration.1 += "\n};\n";

    let multi_comma = regex::Regex::new(r#",(?:\s+,)+"#).unwrap();

    let new_lines = replace_in_file(&file, &declaration, &replacer,
                                    &mut struct_declaration,
                                    true, global_name, true, &multi_comma);
    std::fs::write(filename, new_lines.join("\n")).unwrap();
    // TODO automate with "rg" and don't forget to remove the original file
    //  rg "(pv___)|(pv_depth___)|(board___)" --files-with-matches
    let usages_file_paths: &[&'static str] = &[
        "../scrzebra/src/scrzebra.rs",
    "../engine/src/getcoeff.rs",
    "../engine/src/end.rs",
    "../engine/src/eval.rs",
    "../engine/src/osfbook.rs",
    "../engine/src/zebra.rs",
    "../engine/src/globals.rs",
    "../engine/src/midgame.rs",
    "../engine/src/moves.rs",
    "../engine/src/game.rs",
    "../enddev/src/enddev.rs",
    "../legacy-zebra/src/game.rs",
    "../legacy-zebra/src/zebra.rs",
    "../legacy-zebra/src/osfbook.rs",
    "../practice/src/practice.rs",
    "../engine/src/search.rs",

    // "../legacy-zebra/src/zebra.rs",
        // "../legacy-zebra/src/display.rs",
        // "../legacy-zebra/src/error.rs",
        // "../legacy-zebra/src/game.rs",
        // "../legacy-zebra/src/getcoeff.rs",
        // "../legacy-zebra/src/learn.rs",
        // "../legacy-zebra/src/main.rs",
        // "../legacy-zebra/src/osfbook.rs",
        // "../legacy-zebra/src/safemem.rs",
        // "../legacy-zebra/src/thordb.rs",
        // "../engine/src/cntflip.rs",
        // "../engine/src/counter.rs",
        // "../engine/src/end.rs",
        // "../engine/src/error.rs",
        // "../engine/src/eval.rs",
        // // "../engine/src/game.rs",
        // "../engine/src/getcoeff.rs",
        // "../engine/src/globals.rs",
        // "../engine/src/hash.rs",
        // "../engine/src/learn.rs",
        // "../engine/src/midgame.rs",
        // "../engine/src/moves.rs",
        // "../engine/src/myrandom.rs",
        // "../engine/src/opname.rs",
        // "../engine/src/osfbook.rs",
        // "../engine/src/probcut.rs",
        // "../engine/src/search.rs",
        // "../engine/src/stable.rs",
        // "../engine/src/stubs.rs",
        // "../engine/src/thordb.rs",
        // "../engine/src/timer.rs",
        // "../engine/src/zebra.rs",
    ];
    for usages_file_path in usages_file_paths.iter() {
        let file = std::fs::read_to_string(usages_file_path).unwrap();
        let new_lines = replace_in_file(&file, &declaration, &replacer,
                                        &mut struct_declaration,
                                        false, global_name,
                                        false, &multi_comma);
        std::fs::write(usages_file_path, new_lines.join("\n")).unwrap();
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
