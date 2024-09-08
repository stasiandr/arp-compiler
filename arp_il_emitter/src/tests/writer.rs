#![cfg(test)]

use std::{fs::{self, File}, io::{BufWriter, Write}, process::Command};

use arp_ast_processor::{
    build_multiple_sources,
    types::file::ArpFile,
};
use arp_types::sources::Source;
use tempfile::TempDir;

use crate::{emitter::Emitter, file_writer::write_tokens_to_file};


fn test_write_and_compile<S: Into<String>>(input: S, dir: &TempDir) -> String {
    let sources = [Source::new_inline("Main.arp", input)];
    let ast = build_multiple_sources(&sources).unwrap();
    let node = ast
        .get_child_of_kind::<ArpFile, _>(ast.get_root_index())
        .unwrap();

    let tokens = Emitter::new().emit_node(&ast, node.as_weak()).unwrap();

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
pub fn test_compile() {
    let temp_dir = TempDir::new().unwrap();

    let output = test_write_and_compile("fn main() { let x = 1; }", &temp_dir);

    println!("{}", output);
}


#[test]
pub fn test_expressions() {
    let temp_dir = TempDir::new().unwrap();

    let output = test_write_and_compile("from extern System.Console.dll import System.Console fn main() { System.Console.WriteLine(1); }", &temp_dir);

    assert_eq!(output, "1\n");
}

#[test]
pub fn test_complex_expression() {
    let temp_dir = TempDir::new().unwrap();

    let output = test_write_and_compile("from extern System.Console.dll import System.Console fn main() { System.Console.WriteLine(1 + 2 * 3 / (1 - 4)); }", &temp_dir);

    assert_eq!(output, "-1\n");
}


#[test]
pub fn test_boolean() {
    let temp_dir = TempDir::new().unwrap();

    let output = test_write_and_compile("from extern System.Console.dll import System.Console fn main() { System.Console.WriteLine(!true); }", &temp_dir);

    assert_eq!(output, "False\n");
}

#[test]
pub fn test_greater() {
    let temp_dir = TempDir::new().unwrap();

    let output = test_write_and_compile("from extern System.Console.dll import System.Console fn main() { System.Console.WriteLine(false == (3 > 1) and true); }", &temp_dir);

    assert_eq!(output, "False\n");
}


#[test]
pub fn test_blok_scope() {
    let temp_dir = TempDir::new().unwrap();

    let output = test_write_and_compile("from extern System.Console.dll import System.Console fn main() { { let x = 1; } let x = 2; System.Console.WriteLine(x); }", &temp_dir);

    assert_eq!(output, "2\n");
}



#[test]
pub fn test_class() {
    let temp_dir = TempDir::new().unwrap();

    let output = test_write_and_compile("
    from extern System.Console.dll import System.Console 

    class MyClass {
        my_field: int32
    }

    fn main() { 
        let inst = MyClass { my_field: 1 };
        System.Console.WriteLine(inst.my_field);
    }
    ", &temp_dir);

    assert_eq!(output, "1\n");
}


#[test]
pub fn test_impl() {
    let temp_dir = TempDir::new().unwrap();

    let output = test_write_and_compile("
    from extern System.Console.dll import System.Console 

    class MyClass {
        my_field: int32
    }

    impl MyClass {
        fn log(this, sum_op: int32) {
            System.Console.WriteLine(this.my_field + sum_op);
        }

        fn static_log() {
            System.Console.WriteLine(\"hello\");
        }
    }

    fn main() { 
        let inst = MyClass { my_field: 1 };
        inst.log(2);
        MyClass.static_log();
    }
    ", &temp_dir);

    assert_eq!(output, "3\nhello\n");
}


#[test]
pub fn test_assignment() {
    let temp_dir = TempDir::new().unwrap();

    let output = test_write_and_compile("
    from extern System.Console.dll import System.Console 

    class MyClass {
        my_field: int32
    }
    fn main() { 
        let inst = MyClass { my_field: 1 };
        let x = 2;
        x = 3;
        inst.my_field = x;

        Console.WriteLine(inst.my_field);
    }
    ", &temp_dir);

    assert_eq!(output, "3\n");
}


#[test]
pub fn test_control_statements() {
    let temp_dir = TempDir::new().unwrap();

    let output = test_write_and_compile("
    from extern System.Console.dll import System.Console 

    class MyClass {
        my_field: int32
    }

    impl MyClass {
    
        fn get(this) -> int32 {
            return this.my_field;
        }
    }

    fn main() { 
        let inst = MyClass { my_field: 1 };
        
        if inst.get() == 2 {
            Console.WriteLine(3);
        } else {
            Console.Write(4);
        }

        let i = 5;
        while i < 10 {
            Console.Write(1); // TODO type resolver failed to resolve Write type
            i = i + 1;
        }
    }
    ", &temp_dir);

    assert_eq!(output, "411111");
}

