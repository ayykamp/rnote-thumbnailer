use std::io::Write;

use clap::Parser;
use image::imageops::FilterType;
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
        .and_then(|rawfile| {
            let size = rawfile
                .thumbnail_sizes()
                .iter()
                .filter(|s| **s >= output_size)
                .fold(
                    0,
                    |acc, size| if acc == 0 || *size < acc { *size } else { acc },
                );
            if size == 0 {
                eprintln!("No thumbnail found");
                return Err(libopenraw::Error::NotFound);
            }
            // XXX fixme it's not the smallest.
            rawfile.thumbnail_for_size(size)
        })
        .expect("Thumbnail not found");

    let width = thumbnail.width();
    let height = thumbnail.height();

    let need_resize = std::cmp::max(thumbnail.width(), thumbnail.height()) > output_size;

    let image_buffer = match thumbnail.data_type() {
        DataType::Jpeg => {
            if cli.jpeg && !need_resize {
                let mut file = std::fs::File::create(cli.output).expect("Can't create output");
                let _written = file
                    .write(thumbnail.data8().expect("Couldn't get thumbnail data"))
                    .expect("Can't write output");

                return;
            } else {
                let mut data =
                    std::io::Cursor::new(thumbnail.data8().expect("Couldn't get thumbnail data"));
                let reader = ImageReader::with_format(&mut data, image::ImageFormat::Jpeg);
                reader.decode().map(|image| image.into_rgb8()).ok()
            }
        }
        DataType::PixmapRgb8 => image::ImageBuffer::from_raw(
            width,
            height,
            thumbnail
                .data8()
                .expect("Couldn't get thumbnail data")
                .to_vec(),
        ),
        _ => None,
    };

    if let Some(mut image_buffer) = image_buffer {
        let format = if cli.jpeg {
            image::ImageFormat::Jpeg
        } else {
            image::ImageFormat::Png
        };

        if need_resize {
            let ratio = if width >= height {
                // Landscape
                width as f64 / output_size as f64
            } else {
                // Portrait
                height as f64 / output_size as f64
            };
            let nwidth = width as f64 / ratio;
            let nheight = height as f64 / ratio;

            image_buffer = image::imageops::resize(
                &image_buffer,
                nwidth as u32,
                nheight as u32,
                FilterType::Nearest,
            );
        }
        image_buffer
            .save_with_format(cli.output, format)
            .expect("Failed to save to {format:?}");
    };
    println!("done");
}

#[cfg(test)]
mod test {
    use std::io::BufRead;

    #[test]
    fn test_mimes() {
        let thumbnailer_file = include_bytes!("../data/raw.thumbnailer");
        let cursor = std::io::Cursor::new(thumbnailer_file);
        let mut reader = std::io::BufReader::new(cursor);
        let mut line = String::default();
        while let Ok(r) = reader.read_line(&mut line) {
            if r == 0 {
                break;
            }

            if line.starts_with("MimeType=") {
                let mut mimes = line
                    .trim_end()
                    .split('=')
                    .skip(1)
                    .next()
                    .unwrap()
                    .split(';')
                    .filter(|s| *s != "")
                    .collect::<Vec<_>>();
                let mut or_mimes = libopenraw::mime_types().to_vec();

                mimes.sort();
                or_mimes.sort();

                assert_eq!(mimes, or_mimes);
            }
            line.clear();
        }
    }
}
