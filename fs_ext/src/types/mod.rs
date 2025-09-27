pub mod formats {
    mod json;
    mod toml;
    mod yaml;
    pub use {json::Json, toml::Toml, yaml::Yaml};
}
mod path_kind;

pub use path_kind::PathKind;
