use clap::arg_enum;
use std::path::PathBuf;

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
        name = "options",
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
/*#[derive(StructOpt, Debug)]
struct ResizeOptions {
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
}*/

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
            resize(size, imgtype, mode, &mut src_folder, file)
        }

        _ => println!("Got nothing"),
    }
    //println!("{:?}", matches);
}
