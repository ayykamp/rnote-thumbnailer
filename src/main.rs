// SPDX-License: GPL-3.0-or-later
// SPDX-Copyright: Hubert Figuière <hub@figuiere.net>

use std::error::Error;
use std::io::Write;

use clap::Parser;
use gettextrs::gettext as i18n;
use image::imageops::FilterType;
use image::{DynamicImage, ImageReader};
use libopenraw::RenderingOptions;
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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let output_size = cli.size.unwrap_or(128);

    let rawfile = libopenraw::rawfile_from_file(cli.input, None)?;
    let size = rawfile
        .thumbnail_sizes()
        .iter()
        .filter(|s| **s >= output_size)
        .fold(
            0,
            |acc, size| if acc == 0 || *size < acc { *size } else { acc },
        );
    let orientation = rawfile.orientation();
    let thumbnail = if size == 0 {
        let options = RenderingOptions::default();
        let rawdata = rawfile.raw_data(false)?;
        let rendered_image: Box<dyn Bitmap> = Box::new(rawdata.rendered_image(options)?);
        rendered_image
    } else {
        let thumbnail: Box<dyn Bitmap> = Box::new(rawfile.thumbnail_for_size(size)?);
        thumbnail
    };
    let width = thumbnail.width();
    let height = thumbnail.height();

    let need_resize = std::cmp::max(thumbnail.width(), thumbnail.height()) > output_size;

    let image_buffer = match thumbnail.data_type() {
        DataType::Jpeg => {
            if cli.jpeg && !need_resize {
                let mut file = std::fs::File::create(cli.output)?;
                let _written = file.write(
                    thumbnail
                        .data8()
                        .unwrap_or_else(|| panic!("{}", &i18n("Couldn't get thumbnail data"))),
                )?;

                return Ok(());
            } else {
                let mut data =
                    std::io::Cursor::new(thumbnail.data8().ok_or(libopenraw::Error::NotFound)?);
                let reader = ImageReader::with_format(&mut data, image::ImageFormat::Jpeg);
                reader
                    .decode()
                    .map(|image| image.into_rgb8())
                    .ok()
                    .map(DynamicImage::ImageRgb8)
            }
        }
        DataType::PixmapRgb8 => image::ImageBuffer::from_raw(
            width,
            height,
            thumbnail
                .data8()
                .ok_or(libopenraw::Error::NotFound)?
                .to_vec(),
        )
        .map(DynamicImage::ImageRgb8),
        DataType::PixmapRgb16 => image::ImageBuffer::from_raw(
            width,
            height,
            thumbnail
                .data16()
                .ok_or(libopenraw::Error::NotFound)?
                .to_vec(),
        )
        .map(DynamicImage::ImageRgb16),
        _ => {
            eprintln!("Unhandled format {:?}", thumbnail.data_type());
            None
        }
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

            image_buffer = DynamicImage::ImageRgba8(image::imageops::resize(
                &image_buffer,
                nwidth as u32,
                nheight as u32,
                FilterType::Nearest,
            ));
        }
        // Exif orientation explainer:
        // https://jdhao.github.io/2019/07/31/image_rotation_exif_info/
        image_buffer = match orientation {
            2 => DynamicImage::ImageRgba8(image::imageops::flip_horizontal(&image_buffer)),
            3 => DynamicImage::ImageRgba8(image::imageops::rotate180(&image_buffer)),
            4 => DynamicImage::ImageRgba8(image::imageops::flip_vertical(&image_buffer)),
            5 | 6 => {
                if orientation == 5 {
                    image::imageops::flip_horizontal_in_place(&mut image_buffer)
                }
                DynamicImage::ImageRgba8(image::imageops::rotate90(&image_buffer))
            }
            7 | 8 => {
                if orientation == 7 {
                    image::imageops::flip_horizontal_in_place(&mut image_buffer)
                }
                DynamicImage::ImageRgba8(image::imageops::rotate270(&image_buffer))
            }
            _ => image_buffer,
        };
        image_buffer.save_with_format(cli.output, format)?;
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::BufRead;

    #[test]
    /// This test verify that the thumbnailer file has all the MIME types
    /// that libopenraw handles.
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
