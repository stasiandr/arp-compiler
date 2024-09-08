use std::{fs::{self, File}, io::{BufWriter, Write}, process::Command};

use arp_ast_processor::build_multiple_sources;
use arp_types::sources::Source;
use tempfile::TempDir;

use crate::{emitter::Emitter, file_writer::write_tokens_to_file};

fn test_write_and_run_multi_source(sources: &[Source], dir: &TempDir) -> String {
    let ast = build_multiple_sources(sources).unwrap();
    let node = ast.get_root_index();

    let tokens = Emitter::new().emit_node(&ast, node).unwrap();

    // let dir = std::path::PathBuf::from("/Users/stas/Desktop/test/");
    // let path = dir.as_path();
    let path = dir.path();

    let file_path = path.join("main.il");
    write_tokens_to_file(&file_path, &tokens).unwrap();

    let project_file = File::create(path.join("project.ilproj")).unwrap();
    let mut buf_writer = BufWriter::new(project_file);
    write!(buf_writer, r#"
<Project Sdk="Microsoft.Net.Sdk.il/8.0.0">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
  </PropertyGroup>
</Project>
"#).unwrap();

    buf_writer.flush().unwrap();

// dotnet build --nologo -v q --property WarningLevel=0 /clp:ErrorsOnly && dotnet run --no-build
    let build_proc = Command::new("dotnet")
        .args(["build", &path.to_string_lossy()])
        .output()
        .unwrap();

    if !build_proc.status.success() { 
        println!("{}\n\n", String::from_utf8(build_proc.stdout).unwrap());
        println!("{}\n\n", String::from_utf8(build_proc.stderr).unwrap());
    }


    let output = Command::new("dotnet")
        .args(["run", "--no-build", "--project", &path.to_string_lossy()])
        .output()
        .unwrap();


    println!("File:\n{}", fs::read_to_string(file_path).unwrap());

    if !output.status.success() {
        eprintln!("StdOut:\n{}", String::from_utf8(output.stdout).unwrap());
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    }
    
    String::from_utf8(output.stdout).unwrap()
}




#[test]
fn simple() {
    let temp_dir = TempDir::new().unwrap();

    let sources = [
        Source::new_inline("Main.arp", "fn main() { let x = 1; }"),
        Source::new_inline("Other.arp", "class MyClass { }"),
    ];

    test_write_and_run_multi_source(&sources, &temp_dir);
}

#[test]
fn import() {
    let temp_dir = TempDir::new().unwrap();

    let sources = [
        Source::new_inline("Main.arp", "from Other import MyClass from extern System.Console.dll import System.Console  fn main() { let x = MyClass { my_field: 1 }; Console.Write(x.my_field); }"),
        Source::new_inline("Other.arp", "class MyClass { my_field: int32 }"),
    ];

    let output = test_write_and_run_multi_source(&sources, &temp_dir);

    assert_eq!(output, "1");
}