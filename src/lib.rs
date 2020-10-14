use std::path::PathBuf;
use std::fs::File;
use std::fs;
use std::io;
use env_logger;
use log::*;
use unzip::Unzipper;
use walkdir::WalkDir;

pub struct DataSource {
    pub name: &'static str,
    pub url: &'static str
}

#[derive(Debug)]
pub struct SampleData {
    pub root: PathBuf,
    pub num_files: u64,
    pub size: u64,
    pub archive: PathBuf
}

impl SampleData {
    /// Delete all files pulled in by this SampleData
    pub fn remove(&self) -> io::Result<()>{
        fs::remove_dir_all(&self.root)?;
        fs::remove_file(&self.archive)
    }
}

const KERNEL: DataSource = DataSource {
    name: "Linux_Kernel",
    url: "https://github.com/torvalds/linux/archive/v5.9.zip"
};


const CARGO: DataSource = DataSource {
    name: "Cargo_sources",
    url: "https://github.com/rust-lang/cargo/archive/0.47.0.zip"
};


fn download_and_unpack(ds: DataSource) -> Result<SampleData, String> {
    std::fs::create_dir_all(ds.name);

    let archive = format!("{}.zip", ds.name);
    let mut num_files = 0;
    let mut size = 0;


    if !std::path::Path::new(&archive).is_file() {
        info!("Downloading {:?}", ds.url);

        let mut resp = reqwest::blocking::get(ds.url).map_err(|e| format!("{:?}", e))?;
        let mut out = File::create(&archive).map_err(|e| format!("{:?}", e))?;
        std::io::copy(&mut resp, &mut out).map_err(|e| format!("{:?}", e))?;
    } else {
        info!("Did not download, archive already present");
    }

    info!("Unzipping...");

    Unzipper::new(File::open(&archive).unwrap(), ds.name).unzip().map_err(|e| format!("{:?}", e))?;
    info!("Sample data ready. Gathering stats...");

    for entry in WalkDir::new(ds.name).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
        num_files +=1;
        if let Ok(meta) = entry.metadata() {
            size += meta.len();
        }
    }

    data_path(ds.name).ok_or("Could not get data dir".to_string()).map(|x| SampleData {
        root: x,
        archive: std::path::PathBuf::from(&archive),
        num_files,
        size
    })
}

fn setup() {
    std::env::set_var("RUST_LOG", "INFO");
    let _ = env_logger::builder().try_init();
}


fn data_path(data_dir: &str) -> Option<PathBuf> {
    PathBuf::from(file!()).parent()
    .map(|p| p.parent())
    .flatten()
    .map(|p| p.join(data_dir))
}

/// Linux kernel sources (~75k files, ~910MB)
pub fn linux_kernel() -> Result<SampleData, String>{
    setup();
    download_and_unpack(KERNEL)
}

/// Cargo sources (610 files, ~5MB)
pub fn cargo_sources() -> Result<SampleData, String>{
    setup();
    download_and_unpack(CARGO)
}

#[test]
fn test_kernel() {
    std::env::set_var("RUST_LOG", "INFO");
    let _ = env_logger::builder().try_init();
    //info!("{:?}", linux_kernel());
    info!("{:?}", cargo_sources());
}