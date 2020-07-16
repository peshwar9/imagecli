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
        #[structopt(long, case_insensitive = true, parse(from_os_str))]
        destfolder: PathBuf,
        #[structopt(long, case_insensitive = true)]
        file: String,
    },
}

// To run:
// cargo run resize --imgtype png --mode single --size small --srcfolder /Users/prabhueshwarla/rust/author/packt/code/c4/imagecli/test.jpg --file test.jpg --destfolder /Users/prabhueshwarla/rust/author/packt/code/c4/imagecli/tmp/test-small.png
fn main() {
    match Opt::from_args() {
        Opt::Resize {
            size,
            mode,
            imgtype,
            srcfolder,
            destfolder,
            file,
        } => {
            println!(
                "Got Size: {:?},mode: {:?}, imagetype:{:?}, srcfolder:{:?}, destfolder:{:?},file:{:?}",
                size, mode, imgtype, srcfolder, destfolder, file
            );

            let mut src_folder = srcfolder;
            let mut dest_folder = destfolder;
            resize(size, imgtype, mode, &mut src_folder, &mut dest_folder, file);
        }

        _ => println!("Got nothing"),
    }
    //println!("{:?}", matches);
}
