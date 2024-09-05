use std::io::Write;

use clap::Parser;
use image::ImageReader;
use libopenraw::{Bitmap, DataType};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    jpeg: bool,
    #[arg(short, long)]
    size: Option<u32>,
    #[arg(short, long)]
    verbose: bool,

    input: std::ffi::OsString,
    output: std::ffi::OsString,
}

fn main() {
    let cli = Cli::parse();

    let output_size = cli.size.unwrap_or(128);

    let thumbnail = libopenraw::rawfile_from_file(cli.input, None)
        .and_then(|rawfile| rawfile.thumbnail_for_size(output_size)).expect("Thumbnail not found");

    match thumbnail.data_type() {
         DataType::Jpeg => {
             if cli.jpeg {
                 let mut file = std::fs::File::create(cli.output).expect("Can't create output");
                 let written = file.write(thumbnail.data8().expect("Couldn't get thumbnail data")).expect("Can't write output");
                 println!("Wrote {written}");
             } else {
                 let mut data = std::io::Cursor::new(thumbnail.data8().expect("Couldn't get thumbnail data"));
                 let reader = ImageReader::with_format(&mut data, image::ImageFormat::Jpeg);
                 let image_buffer = reader.decode().expect("Couldn't decode").into_rgb8();
                 image_buffer.save_with_format(
                    cli.output,
                    image::ImageFormat::Png,
                )
                .expect("Failed to save to PNG");
             }
         },
        DataType::PixmapRgb8 => {
            let format = if cli.jpeg {
                image::ImageFormat::Jpeg
            } else {
                image::ImageFormat::Png
            };
            image::save_buffer_with_format(
                cli.output,
                thumbnail.data8().expect("Couldn't get thumbnail data"),
                thumbnail.width(),
                thumbnail.height(),
                image::ColorType::Rgb8,
                format,
            )
            .expect("Failed to save");
        }
        _ => {}
    }
}
