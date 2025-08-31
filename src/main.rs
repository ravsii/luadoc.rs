use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
struct Field {
    name: String,
    optional: bool,
    ty: String,
    doc: Vec<String>,
}

#[derive(Debug)]
struct Method {
    name: String,
    is_instance: bool,
    params: Vec<(String, String)>, // (name, type)
    returns: Vec<String>,
    doc: Vec<String>,
}

#[derive(Debug)]
struct ClassDoc {
    name: String,
    fields: Vec<Field>,
    methods: Vec<Method>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }

    let dir = &args[1];
    let class_re = Regex::new(r"@class\s+([\w:]+)").unwrap();
    let field_re = Regex::new(r"@field\s+(\w+\??)\s+(.+)").unwrap();
    let param_re = Regex::new(r"@param\s+(\w+)\??\s+(.+)").unwrap();
    let return_re = Regex::new(r"@return\s+(.+)").unwrap();
    let func_re = Regex::new(r"function\s+(\w+)([:.])(\w+)").unwrap();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "lua" {
                    process_file(
                        entry.path(),
                        &class_re,
                        &field_re,
                        &param_re,
                        &return_re,
                        &func_re,
                    );
                }
            }
        }
    }
}

fn process_file(
    path: &Path,
    class_re: &Regex,
    field_re: &Regex,
    param_re: &Regex,
    return_re: &Regex,
    func_re: &Regex,
) {
    let content = fs::read_to_string(path).unwrap_or_default();
    let mut current_class: Option<ClassDoc> = None;
    let mut last_doc_lines: Vec<String> = Vec::new();
    let mut pending_params: Vec<(String, String)> = Vec::new();
    let mut pending_returns: Vec<String> = Vec::new();

    for line in content.lines() {
        let line = line.trim_start();

        if let Some(cap) = class_re.captures(line) {
            if let Some(class) = current_class.take() {
                print_class(path, &class);
            }
            current_class = Some(ClassDoc {
                name: cap[1].to_string(),
                fields: Vec::new(),
                methods: Vec::new(),
            });
            last_doc_lines.clear();
        } else if let Some(cap) = field_re.captures(line) {
            if let Some(class) = &mut current_class {
                let raw_name = &cap[1];
                let optional = raw_name.ends_with('?');
                let name = raw_name.trim_end_matches('?').to_string();
                let ty = cap[2].trim().to_string();

                class.fields.push(Field {
                    name,
                    optional,
                    ty,
                    doc: last_doc_lines.clone(),
                });
            }
            last_doc_lines.clear();
        } else if let Some(cap) = param_re.captures(line) {
            pending_params.push((cap[1].to_string(), cap[2].trim().to_string()));
        } else if let Some(cap) = return_re.captures(line) {
            pending_returns.push(cap[1].trim().to_string());
        } else if let Some(cap) = func_re.captures(line) {
            if let Some(class) = &mut current_class {
                let _class_name = &cap[1]; // e.g. "Timer"
                let sep = &cap[2]; // "." or ":"
                let method_name = &cap[3]; // e.g. "new" or "remaining"

                class.methods.push(Method {
                    name: method_name.to_string(),
                    is_instance: sep == ":",
                    params: pending_params.clone(),
                    returns: pending_returns.clone(),
                    doc: last_doc_lines.clone(),
                });
            }
            last_doc_lines.clear();
            pending_params.clear();
            pending_returns.clear();
        } else if line.starts_with("---") {
            let doc = line.trim_start_matches('-').trim();
            if !doc.is_empty() {
                last_doc_lines.push(doc.to_string());
            }
        } else {
            last_doc_lines.clear();
            pending_params.clear();
            pending_returns.clear();
        }
    }

    if let Some(class) = current_class {
        print_class(path, &class);
    }
}

fn print_class(_path: &Path, class: &ClassDoc) {
    println!("# Class {}\n", class.name);

    // Fields
    if !class.fields.is_empty() {
        println!("## Fields\n");
        println!("| Name | Type | Description |");
        println!("|------|------|-------------|");
        for f in &class.fields {
            let name = format!("{}{}", f.name, if f.optional { "?" } else { "" });
            let desc = if f.doc.is_empty() {
                "".to_string()
            } else {
                f.doc.join(" ")
            };
            println!("| {} | `{}` | {} |", name, f.ty, desc);
        }
        println!();
    }

    // Methods
    if !class.methods.is_empty() {
        println!("## Methods\n");
        println!("| Name | Params | Returns | Description |");
        println!("|------|--------|---------|-------------|");
        for m in &class.methods {
            let name = format!("{}{}", if m.is_instance { ":" } else { "." }, m.name);
            let params = if m.params.is_empty() {
                "".to_string()
            } else {
                m.params
                    .iter()
                    .map(|(p, t)| format!("{}: `{}`", p, t))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let returns = if m.returns.is_empty() {
                "".to_string()
            } else {
                m.returns.join(", ")
            };
            let desc = if m.doc.is_empty() {
                "".to_string()
            } else {
                m.doc.join(" ")
            };
            println!("| {} | {} | {} | {} |", name, params, returns, desc);
        }
        println!();
    }
}
