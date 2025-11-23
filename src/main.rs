use std::{fs, process::{Command}};

use clap::Parser;
use args::Commands;

mod args;

const BOM: &str = "\u{FEFF}";

fn main() {
    let cli = args::Cli::parse();

    match cli.command.unwrap() {
        Commands::Compile { input, output, compile } => {
            let mut code = fs::read_to_string(&input).unwrap();
            if code.starts_with(BOM) {
                code = code.trim_start_matches(BOM).to_string();
            }
            let translated_c = lamplang::translate(&code);
            fs::write(&output, translated_c).expect("Cannot write to output file");
            println!("Your code was successful translated!");
            if compile {
                if cfg!(windows) {
                    let cmd = format!(
                        r#"call ""{}"" x64 && cl /MDd {} /I ./lib/include/ /link /LIBPATH:./lib/build/Debug lamp_lib.lib"#,
                        r"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat",
                        &output
                    );
                    println!("{}", cmd);
                    let command = Command::new("cmd")
                        .args(["/C", &cmd])
                        .output()
                        .expect("Cannot run vcvarsall or cl");
                    println!("{}{}", String::from_utf8_lossy(&command.stdout), String::from_utf8_lossy(&command.stderr));
                    
                } else if cfg!(unix) {
                    let command = Command::new("gcc")
                            .args([&output,
                             "-I",
                             "./lib/include/",
                             "-L",
                             "./lib/build/Debug",
                             "-llamp_lib"]).output().expect("Cannot compile translated c");
                    println!("{}", String::from_utf8_lossy(&command.stdout));
                }
            }
        }
        Commands::Init { .. } => (),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_input_and_variables() {
        let input = "\n
        use io
        var a: number = 2\n
        var b: number = a + 4\n
        var name: string = \"something\"\n
        print(\"{s}\", name)\n
        ".to_string();
        let c_code = lamplang::translate(input.as_ref());
        print!("{}", c_code);
    }
}