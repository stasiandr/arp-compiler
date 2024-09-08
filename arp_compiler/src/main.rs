pub mod compile_single_file;
pub mod errors;
pub mod assembly;

use std::{fs::{self, File}, io::{BufWriter, Write}, path::PathBuf};

use arp_ast_processor::build_multiple_sources;
use arp_il_emitter::{emitter::Emitter, file_writer::write_tokens_to_file};
use arp_types::sources::Source;
use assembly::Project;
use clap::{Parser, Subcommand};
use errors::CompilerError;
use tempfile::TempDir;

#[derive(Parser)]
#[command(name = "ARP Compiler")]
#[command(about = "A compiler for the ARP language", long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    #[arg(long, default_value = "true")]
    generate_debug_info: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        #[arg(long, default_value = "true")]
        dotnet_publish: bool,
    },
    Run,
    Lsp,
}


fn main() -> Result<(), CompilerError> {
    let args = Args::parse();
    let project = Project::new(&args.path, args.generate_debug_info).expect("Failed to find project");

    if project.config.dev.clean_build && project.build_path().exists() {
        fs::remove_dir_all(project.build_path())?;
    }

    

    match args.command {
        Commands::Compile { .. } => {
            todo!()
        },
        Commands::Run => {
            let temp_dir = TempDir::new()?;

            let sources = project.load_sources()?;
            
            let output = test_write_and_run_multi_source(&sources, &temp_dir);
            print!("{}", output);
        },
        Commands::Lsp => {
            arp_lsp::run_server();
        },
    };

    Ok(())
}

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
    let build_proc = std::process::Command::new("dotnet")
        .args(["build", &path.to_string_lossy()])
        .output()
        .unwrap();

    if !build_proc.status.success() { 
        println!("{}\n\n", String::from_utf8(build_proc.stdout).unwrap());
        println!("{}\n\n", String::from_utf8(build_proc.stderr).unwrap());
    }


    let output = std::process::Command::new("dotnet")
        .args(["run", "--no-build", "--project", &path.to_string_lossy()])
        .output()
        .unwrap();

    if !output.status.success() {
        eprintln!("StdOut:\n{}", String::from_utf8(output.stdout).unwrap());
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    }
    
    String::from_utf8(output.stdout).unwrap()
}