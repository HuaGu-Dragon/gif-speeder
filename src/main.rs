use std::{
    fs::File,
    io::{BufReader, BufWriter, ErrorKind, Write},
    path::PathBuf,
};

use anyhow::Context;
use clap::Parser;
use image::{
    AnimationDecoder, Delay, Frame,
    codecs::gif::{GifDecoder, GifEncoder, Repeat},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    speed: f64,

    #[arg(long, short, default_value_t = 10)]
    min: u32,

    #[arg(long, short, default_value_t = 0)]
    skip: usize,

    #[arg(long, short, default_value_t = 0)]
    repeat: u16,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let input = cli.input;
    let output = cli.output;
    let speed = cli.speed;
    let min_delay = cli.min;
    let init_skip = cli.skip;
    let repeat = cli.repeat;

    let file_out = match File::create_new(&output) {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            print!("{output:?} already exists, do you want to overwrite it? [y/N] ");
            std::io::stdout().flush().context("flush stdout")?;

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .context("Cannot read input")?;

            if input.trim().to_lowercase() == "y" {
                File::create(output).context("Cannot create output file")?
            } else {
                return Ok(());
            }
        }
        Err(e) => {
            anyhow::bail!("Cannot create output file: {}", e);
        }
    };

    let file_in =
        BufReader::new(File::open(&input).with_context(|| format!("Cannot find {input:?}"))?);
    let decoder = GifDecoder::new(file_in).context("create gif decoder")?;

    let frames = decoder.into_frames();

    let file_out = BufWriter::new(file_out);
    let mut encoder = GifEncoder::new(file_out);

    encoder
        .set_repeat(if repeat == 0 {
            Repeat::Infinite
        } else {
            Repeat::Finite(repeat)
        })
        .context("set animation repeat")?;

    let mut skip = 0;
    for (idx, frame) in frames.into_iter().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        } else {
            skip = init_skip;
        }

        let frame = frame.context("Cannot decode frame")?;

        let left = frame.left();
        let top = frame.top();
        let (num, denom) = frame.delay().numer_denom_ms();
        let adjusted_num = ((num as f64 / speed) as u32).max(min_delay);
        println!("[Frame {idx}] delay: {num}/{denom} -> {adjusted_num}/{denom}");

        let new_frame = Frame::from_parts(
            frame.into_buffer(),
            left,
            top,
            Delay::from_numer_denom_ms(adjusted_num, denom),
        );

        encoder
            .encode_frame(new_frame)
            .context("Cannot encode frame")?;
    }

    Ok(())
}
