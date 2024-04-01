use std::io::{self, BufWriter, Result, Write};

use ray_tracing_one_weekend::color::Color;

fn main() -> Result<()> {
    let image_width: usize = 256;
    let image_height: usize = 256;

    let mut writer = BufWriter::new(io::stdout().lock());

    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", image_width, image_height)?;
    writeln!(writer, "255")?;

    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", (image_height - j));
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0 as f64;
            let color = Color::new(r, g, b);

            color.write_ppm(&mut writer)?;
        }
    }

    eprintln!("Done.");

    Ok(())
}
