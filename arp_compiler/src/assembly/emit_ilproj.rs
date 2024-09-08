use std::io::Write;
use std::{fs, io::BufWriter};

use crate::errors::CompilerError;

use super::Project;



impl Project {
    
    pub fn emit_ilproj(&self) -> Result<(), CompilerError> {

        let file_path = self.build_path_for_file(self.config.package.name.clone() + ".ilproj");

        fs::create_dir_all(file_path.parent().ok_or(CompilerError::Custom("Parent folder not found".to_string()))?)?;
        let file = fs::File::create(file_path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "<Project Sdk=\"{}\">", self.config.dotnet.sdk)?;

        writeln!(writer, "  <ItemGroup>")?;
        writeln!(writer, "    <Reference Include=\"arp-standard-library\" />")?;
        writeln!(writer, "  </ItemGroup>")?;
        
        
        writeln!(writer, "  <PropertyGroup>")?;
        writeln!(writer, "    <OutputType>{}</OutputType>", if self.config.dotnet.output_exe { "Exe" } else { "Dll" })?;
        writeln!(writer, "    <TargetFramework>{}</TargetFramework>", self.config.dotnet.target)?;

        if self.config.dotnet.self_contained {
            writeln!(writer, "    <PublishSingleFile>true</PublishSingleFile>")?;
            writeln!(writer, "    <SelfContained>True</SelfContained>")?;
        }

        writeln!(writer, "  </PropertyGroup>")?;
        writeln!(writer, "</Project>")?;
        
        Ok(())
    }
}

// Example: 
// <Project Sdk="Microsoft.Net.Sdk.il/8.0.0">
//   <ItemGroup>
//     <Reference Include="std" />
//   </ItemGroup>
//   <PropertyGroup>
//     <OutputType>Dll</OutputType>
//     <TargetFramework>net8.0</TargetFramework>
//     <PublishSingleFile>true</PublishSingleFile>
//     <SelfContained>True</SelfContained>
//   </PropertyGroup>
// </Project>
