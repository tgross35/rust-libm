use std::env;
use std::f32::consts::PI;
use std::path::Path;

use plotters::prelude::*;

macro_rules! unary_op {
    (   $out_dir:ident,
        $($fn_name:ident, $xrange:expr, $yrange:expr;)+
    ) => {$(
        paste::paste!{
            let out_file = $out_dir.join(concat!(stringify!($fn_name), ".svg"));
            let func = libm::[< $fn_name f >];
            let xrange: (f32, f32) = $xrange;
            let yrange: (f32, f32) = $yrange;
            let scale = 1000.0f32;
            let inputs = (($xrange.0 * scale) as i32)..=(($xrange.1 * scale) as i32);

            let root = SVGBackend::new(&out_file, (640, 480)).into_drawing_area();
            // root.fill(&WHITE)?;
            root.fill(&TRANSPARENT)?;
            let mut chart = ChartBuilder::on(&root)
                .caption(
                    concat!("y = ", stringify!($fn_name), "(x)"), ("sans-serif", 24, FontStyle::Italic).into_font()
                )
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(xrange.0..xrange.1, yrange.0..yrange.1)?;

            chart.configure_mesh().draw()?;

            chart
                .draw_series(LineSeries::new(
                    inputs.map(|x| x as f32 / scale).map(|x| (x, func(x))),
                    &RED,
                ))?;

            root.present()?;
        }
    )*};
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_env = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_env);
    let out_dir = manifest_dir.join("../../etc/plots");

    // TODO: smooth line, make thicker, fewer grid marks, allow overriding names,
    // thicker lines at axes, combine bessel functions
    unary_op! {
        out_dir,
        acos, (-1.2, 1.2), (-0.5, PI + 0.5);
        acosh, (-0.1, 4.0), (-0.5, 3.0);
        asin, (-1.2, 1.2), (-2.0, 2.0);
        asinh, (-5.0, 3.0), (-5.0, 3.0);
        atan, (-4.0, 4.0), (-2.0, 2.0);
        // atan2, (-2.0, 2.0), (-2.0, 2.0);
        atanh, (-4.0, 4.0), (-4.0, 4.0);
        cbrt, (-4.0, 4.0), (-2.5, 2.5);
        ceil, (-4.0, 4.0), (-4.0, 4.0);
        // copysign, (-2.0, 2.0), (-2.0, 2.0);
        cos, (-5.0, 5.0), (-1.2, 1.2);
        cosh, (-4.0, 4.0), (-0.5, 5.5);
        erf, (-3.5, 3.5), (-1.2, 1.2);
        erfc, (-3.5, 3.5), (-0.2, 2.2);
        exp, (-6.0, 2.0), (-0.5, 6.0);
        exp10, (-4.0, 2.0), (-0.5, 6.0);
        exp2, (-10.0, 4.0), (-0.5, 6.0);
        // TODO: plot exp on the same plot
        expm1, (-5.0, 5.0), (-1.2, 1.2);
        fabs, (-5.0, 5.0), (-0.2, 5.2);
        // fdim, (-5.0, 5.0), (-1.2, 1.2);
        floor, (-4.0, 4.0), (-4.0, 4.0);
        // fma, (-5.0, 5.0), (-1.2, 1.2);
        // fmax, (-5.0, 5.0), (-1.2, 1.2);
        // fmin, (-5.0, 5.0), (-1.2, 1.2);
        // fmod, (-5.0, 5.0), (-1.2, 1.2);
        // hypot, (-5.0, 5.0), (-1.2, 1.2);
        // ilogb, (-5.0, 5.0), (-1.2, 1.2);
        j0, (-10.0, 10.0), (-0.6, 1.2);
        j1, (-10.0, 10.0), (-0.6, 1.2);
        // jn, (-5.0, 5.0), (-1.2, 1.2);
        // ldexp, (-5.0, 5.0), (-1.2, 1.2);
        lgamma, (-2.0, 5.0), (-1.2, 5.2);
        log, (-5.0, 5.0), (-1.2, 1.2);
        log10, (-5.0, 5.0), (-1.2, 1.2);
        log1p, (-5.0, 5.0), (-1.2, 1.2);
        log2, (-5.0, 5.0), (-1.2, 1.2);
        // modf, (-5.0, 5.0), (-1.2, 1.2);
        // nextafter, (-5.0, 5.0), (-1.2, 1.2);
        // pow, (-5.0, 5.0), (-1.2, 1.2);
        // remainder, (-5.0, 5.0), (-1.2, 1.2);
        // remquo, (-5.0, 5.0), (-1.2, 1.2);
        rint, (-5.0, 5.0), (-1.2, 1.2);
        round, (-5.0, 5.0), (-1.2, 1.2);
        // scalbn, (-5.0, 5.0), (-1.2, 1.2);
        sin, (-5.0, 5.0), (-1.2, 1.2);
        sinh, (-4.0, 4.0), (-4.0, 4.0);
        sqrt, (-5.0, 5.0), (-1.2, 1.2);
        tan, (-4.0, 4.0), (-4.0, 4.0);
        tanh, (-4.0, 4.0), (-4.0, 4.0);
        tgamma, (-5.0, 5.0), (-1.2, 1.2);
        trunc, (-4.0, 4.0), (-4.0, 4.0);
        y0, (-5.0, 5.0), (-1.2, 1.2);
        y1, (-5.0, 5.0), (-1.2, 1.2);
        // yn, (-5.0, 5.0), (-1.2, 1.2);
    }

    Ok(())
}
