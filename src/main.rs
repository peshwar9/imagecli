use clap::arg_enum;
use std::path::{Path, PathBuf};

use structopt::StructOpt;
mod resize;

use resize::{resize, ImgType, Mode, SizeOption};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "imagecli",
    help = "Specify subcommand : 'stats' (to print statistics) OR 'resize' (to resize images)"
)]
enum Opt {
    #[structopt(
        name = "resize",
        help = "Resizes images to specified size. Specify flag --size"
    )]
    Resize {
        #[structopt(long, short)]
        size: SizeOption,
        #[structopt(long)]
        mode: Mode,
        #[structopt(long)]
        imgtype: ImgType,
        #[structopt(long, case_insensitive = true, parse(from_os_str))]
        srcfolder: PathBuf,
        #[structopt(long, case_insensitive = true)]
        file: String,
    },
}

// To run:
// cargo run resize --imgtype png --mode single --size small --srcfolder $PWD/test.jpg --file test.jpg
fn main() {
    match Opt::from_args() {
        Opt::Resize {
            size,
            mode,
            imgtype,
            srcfolder,
            file,
        } => {
            println!(
                "Got Size: {:?},mode: {:?}, imagetype:{:?}, srcfolder:{:?}, file:{:?}",
                size, mode, imgtype, srcfolder, file
            );

            let mut src_folder = srcfolder;

            resize(size, imgtype, mode, &mut src_folder, file);
        }

        _ => println!("Got nothing"),
    }
    //println!("{:?}", matches);
}
