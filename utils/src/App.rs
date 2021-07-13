use std::path::PathBuf;

#[derive(Debug)]
pub struct App {
    home_dir: PathBuf,
    novus_dir: PathBuf,
    args: Vec<String>,
    flags: Vec<String>,
    config_dir: PathBuf,
}

impl App {
    pub fn initialize() -> Self {
        let cli_args: Vec<String> = std::env::args().collect();
        let home_dir = std::path::PathBuf::from(std::env::var("HOME").unwrap());
        let novus_dir = home_dir.join(".novus");
        let config_dir = home_dir.join(".config/novus");
        std::fs::create_dir_all(config_dir.clone())
            .unwrap_or_else(|e| println!("error: Could not create directory {:?}", e));
        std::fs::create_dir_all(novus_dir.clone())
            .unwrap_or_else(|e| println!("error: Could not create directory {:?}", e));
        let mut args: Vec<String> = Vec::new();
        let mut flag: Vec<String> = Vec::new();
        for arg in cli_args.into_iter().skip(1) {
            if arg.starts_with("--") || arg.starts_with('-') {
                flag.push(arg);
            } else {
                args.push(arg);
            }
        }
        Self {
            args,
            flags: flag,
            home_dir,
            config_dir,
            novus_dir,
        }
    }
    pub fn has_flag(&self, flags: &[&str]) -> bool {
        self.flags
            .iter()
            .any(|flag| flags.iter().any(|search_flag| flag == search_flag))
    }
}
