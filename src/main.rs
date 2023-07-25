#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::env;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process;
use std::sync::atomic::AtomicUsize;
use std::time::Instant;
use std::{fmt, usize};
use tree_sitter::{InputEdit, Language, LogType, Parser, Point, Tree};

fn read_file(path: &str) -> io::Result<String> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

// Print error messages to stderr before exiting so programs calling this cli can detect it easily
macro_rules! error_exit {
    ($msg:expr $(,$arg:expr)*) => {{
        eprintln!($msg $(,$arg)*);
        process::exit(1);
    }}
}

fn main() {
    // Vector to store the arguments passed in the command line.
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: <program> <language: string> <source code filepath: string>");
        return;
    }

    // Parse string arguments
    let language_arg = &args[1];
    let sourcepath_arg = &args[2];

    let mut parser = Parser::new();

    match language_arg.as_ref() {
        "rust" => parser
            .set_language(tree_sitter_rust::language())
            .expect("Error loading Rust grammar"),
        "go" => parser
            .set_language(tree_sitter_go::language())
            .expect("Error loading Rust grammar"),
        "javascript" => parser
            .set_language(tree_sitter_javascript::language())
            .expect("Error loading Rust grammar"),
        "python" => parser
            .set_language(tree_sitter_python::language())
            .expect("Error loading Rust grammar"),
        "c" => parser
            .set_language(tree_sitter_c::language())
            .expect("Error loading Rust grammar"),
        "cpp" => parser
            .set_language(tree_sitter_cpp::language())
            .expect("Error loading Rust grammar"),
        "swift" => parser
            .set_language(tree_sitter_swift::language())
            .expect("Error loading Rust grammar"),
        "html" => parser
            .set_language(tree_sitter_html::language())
            .expect("Error loading Rust grammar"),
        &_ => error_exit!("invalid language argument"),
    }

    let source_code = match read_file(sourcepath_arg) {
        Ok(contents) => contents,
        Err(err) => error_exit!("Error reading file: {}", err),
    };

    let tree = parser.parse(source_code, None).unwrap_or_else(|| {
        error_exit!("Error occurred: tree is empty");
    });
    // let root_node = tree.root_node();

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut cursor = tree.walk();

    let mut needs_newline = false;
    let mut indent_level = 0;
    let mut did_visit_children = false;
    loop {
        let node = cursor.node();
        let is_named = node.is_named();
        if did_visit_children {
            if is_named {
                stdout.write(b")").unwrap_or_else(|err| {
                    error_exit!("Error occurred: {}", err);
                });
                needs_newline = true;
            }
            if cursor.goto_next_sibling() {
                did_visit_children = false;
            } else if cursor.goto_parent() {
                did_visit_children = true;
                indent_level -= 1;
            } else {
                break;
            }
        } else {
            if is_named {
                if needs_newline {
                    stdout.write(b"\n").unwrap_or_else(|err| {
                        error_exit!("Error occurred: {}", err);
                    });
                }
                for _ in 0..indent_level {
                    stdout.write(b"  ").unwrap_or_else(|err| {
                        error_exit!("Error occurred: {}", err);
                    });
                }
                let start: Point = node.start_position();
                let end: Point = node.end_position();
                if let Some(field_name) = cursor.field_name() {
                    write!(&mut stdout, "{}: ", field_name).unwrap_or_else(|err| {
                        error_exit!("Error occurred: {}", err);
                    });
                }
                write!(
                    &mut stdout,
                    "({} [{}, {}] - [{}, {}]",
                    node.kind(),
                    start.row,
                    start.column,
                    end.row,
                    end.column
                )
                .unwrap_or_else(|err| {
                    error_exit!("Error occurred: {}", err);
                });
                needs_newline = true;
            }
            if cursor.goto_first_child() {
                did_visit_children = false;
                indent_level += 1;
            } else {
                did_visit_children = true;
            }
        }
    }
    cursor.reset(tree.root_node());
    println!("");

    let mut first_error = None;
    loop {
        let node = cursor.node();
        if node.has_error() {
            if node.is_error() || node.is_missing() {
                first_error = Some(node);
                break;
            } else {
                if !cursor.goto_first_child() {
                    break;
                }
            }
        } else if !cursor.goto_next_sibling() {
            break;
        }
    }

    if first_error.is_some() {
        if let Some(node) = first_error {
            let start = node.start_position();
            let end = node.end_position();
            write!(&mut stdout, "\t(").unwrap_or_else(|err| {
                error_exit!("Error occurred: {}", err);
            });
            if node.is_missing() {
                if node.is_named() {
                    write!(&mut stdout, "MISSING {}", node.kind()).unwrap_or_else(|err| {
                        error_exit!("Error occurred: {}", err);
                    });
                } else {
                    write!(
                        &mut stdout,
                        "MISSING \"{}\"",
                        node.kind().replace("\n", "\\n")
                    )
                    .unwrap_or_else(|err| {
                        error_exit!("Error occurred: {}", err);
                    });
                }
            } else {
                write!(&mut stdout, "{}", node.kind()).unwrap_or_else(|err| {
                    error_exit!("Error occurred: {}", err);
                });
            }
            write!(
                &mut stdout,
                " [{}, {}] - [{}, {}])",
                start.row, start.column, end.row, end.column
            )
            .unwrap_or_else(|err| {
                error_exit!("Error occurred: {}", err);
            });
            write!(&mut stdout, "\n").unwrap_or_else(|err| {
                error_exit!("Error occurred: {}", err);
            });
            error_exit!("Error occurred");
        }
    }
}
