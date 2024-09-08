use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub package: Package,
    pub dotnet: DotNet,
    pub dev: Dev,
}

#[derive(Deserialize, Debug)]
pub struct Package {
    #[serde(default = "default_package_name")]
    pub name : String,
}

fn default_package_name() -> String {
    "unknown".to_string()
}


#[derive(Deserialize, Debug)]
pub struct DotNet {
    #[serde(default = "default_dotnet_sdk")]
    pub sdk : String,
    #[serde(default = "default_dotnet_target")]
    pub target: String,
    #[serde(default = "default_dotnet_output_type")]
    pub output_exe: bool,
    #[serde(default = "default_dotnet_self_contained")]
    pub self_contained: bool,
}

fn default_dotnet_sdk() -> String {
    "Microsoft.Net.Sdk.il/8.0.0".to_string()
}

fn default_dotnet_target() -> String {
    "net8.0".to_string()
}

fn default_dotnet_output_type() -> bool {
    true
}

fn default_dotnet_self_contained() -> bool {
    true
}


#[derive(Deserialize, Debug)]
pub struct Dev {
    #[serde(default = "default_dev_clean_build")]
    pub clean_build: bool
}

fn default_dev_clean_build() -> bool {
    false
}