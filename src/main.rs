// SPDX-License: GPL-3.0-or-later
// SPDX-Copyright:  ayykamp <kamp@ayyy.dev>

use anyhow::Context;
use clap::Parser;
use image::imageops::FilterType;
use image::DynamicImage;
use rnote_engine::engine::export::{SelectionExportFormat, SelectionExportPrefs};
use rnote_engine::engine::EngineSnapshot;
use rnote_engine::Engine;
use std::fs::File;
use std::io::Read;

///    rnote-thumbnailer{n}{n}
///    Generate a Png image from a given Rnote file.
#[derive(Parser)]
struct Cli {
    /// Size of the exported image in pixels (for the longer side).
    #[arg(short, long, default_value_t = 128)]
    size: u32,
    /// Enable verbose logging.
    #[arg(short, long)]
    verbose: bool,

    /// Path of the input Rnote file
    input: std::ffi::OsString,
    /// Path of the ouput image
    output: std::ffi::OsString,
}

fn main() -> anyhow::Result<()> {
    smol::block_on(async { run().await })
}

pub(crate) async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let output_size = cli.size;

    let mut engine = Engine::default();
    let mut rnote_file_bytes = vec![];

    let mut fh = File::open(cli.input)?;
    fh.read_to_end(&mut rnote_file_bytes)?;
    let engine_snapshot = EngineSnapshot::load_from_rnote_bytes(rnote_file_bytes).await?;

    // We dont care about the return values of these functions
    let _ = engine.load_snapshot(engine_snapshot);
    let _ = engine.select_all_strokes();

    let prefs = SelectionExportPrefs {
        export_format: SelectionExportFormat::Png,
        ..Default::default()
    };
    let export_bytes = engine
        .export_selection(Some(prefs))
        .await??
        .context("Exporting selection failed, no strokes selected.")?;

    let mut image = image::load_from_memory(&export_bytes)?;
    let (width, height) = (image.width(), image.height());

    if std::cmp::max(width, height) > output_size {
        let ratio = if width >= height {
            // Landscape
            width as f64 / output_size as f64
        } else {
            // Portrait
            height as f64 / output_size as f64
        };
        let nwidth = width as f64 / ratio;
        let nheight = height as f64 / ratio;
        image = DynamicImage::from(image::imageops::resize(
            &image,
            nwidth as u32,
            nheight as u32,
            FilterType::Nearest,
        ));
    }

    image.save(cli.output)?;
    Ok(())
}
