use image::ImageFormat;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::{fmt, fs};
struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

#[derive(Debug)]
pub enum SizeOption {
    Small,
    Medium,
    Large,
}

#[derive(Debug)]
pub enum ImgType {
    PNG,
    JPG,
}

#[derive(Debug)]
pub enum Mode {
    Single,
    All,
}

impl FromStr for SizeOption {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "small" => Ok(SizeOption::Small),
            "medium" => Ok(SizeOption::Medium),
            "large" => Ok(SizeOption::Large),
            _ => Ok(SizeOption::Small),
        }
    }
}

impl FromStr for ImgType {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "small" => Ok(ImgType::PNG),
            "medium" => Ok(ImgType::JPG),
            _ => Ok(ImgType::PNG),
        }
    }
}
impl FromStr for Mode {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(Mode::Single),
            "medium" => Ok(Mode::All),
            _ => Ok(Mode::All),
        }
    }
}
impl AsRef<str> for SizeOption {
    fn as_ref(&self) -> &str {
        match &self {
            SizeOption::Small => "small",
            SizeOption::Medium => "medium",
            SizeOption::Large => "large",
        }
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}
fn print_type_of<T>(_: &T) {
    println!("type is {}", std::any::type_name::<T>())
}
pub fn resize(
    size: SizeOption,
    _img_type: ImgType,
    mode: Mode,
    src_folder: &mut PathBuf,
    file_name: String,
) {
    let size = match size {
        SizeOption::Small => 200,
        SizeOption::Medium => 400,
        SizeOption::Large => 800,
    };
    let src_folder_clone = src_folder.clone();
    let mut file_name = src_folder_clone.file_stem().unwrap().to_str();
    let stem = match file_name {
        None => "",
        Some(os_str) => os_str,
    };
    println!("Stem is {:?}", stem);
    print_type_of(&stem);
    let fina = format!("{}-{:?}.png", stem.to_string(), size);
    println!("Fina is {:?}", fina);
    src_folder.pop();
    src_folder.push("tmp/tmp.png");
    src_folder.set_file_name(fina);
    let dest_folder = src_folder.clone();
    println!("src folder 1 is {:?}", src_folder);
    let img = image::open(src_folder_clone).unwrap();
    let timer = Instant::now();
    let scaled = img.thumbnail(size, size);
    println!("Thumbnailed to {} in {}", size, Elapsed::from(&timer));
    let mut output = fs::File::create(dest_folder).unwrap();
    scaled.write_to(&mut output, ImageFormat::Png).unwrap();
}
