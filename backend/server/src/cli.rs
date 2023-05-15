use clap::{arg, command, Parser};

/// Smart home server that serves data measured from the sensors
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Interface and port on which the server should listen
    #[arg(long, default_value_t = String::from("0.0.0.0:8080"))]
    listening_address: String,

    /// Path to the static front-end files
    #[arg(long, default_value_t = String::from("/var/www/smart-home-server/"))]
    web_files_path: String,

    /// Name of the index file
    #[arg(long, default_value_t = String::from("index.html"))]
    index_filename: String,
}

impl Args {
    pub fn get_listening_address(&self) -> &str {
        &self.listening_address
    }

    pub fn get_web_files_path(&self) -> &str {
        &self.web_files_path
    }

    pub fn get_index_filename(&self) -> &str {
        &self.index_filename
    }
}
