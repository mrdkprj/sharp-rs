use crate::{
    common::{remove_alpha, ImageType},
    input::open_input,
    Sharp,
};
use libvips::{
    bindings::vips_interpretation_max_alpha,
    operator::Index,
    ops::{Access, Interpretation},
    v_value,
    voption::VOption,
    Result, VipsImage,
};

#[derive(Debug, Clone, Default)]
pub struct ChannelStats {
    pub min: i32,
    pub max: i32,
    pub sum: f64,
    pub squares_sum: f64,
    pub mean: f64,
    pub stdev: f64,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub channel_stats: Vec<ChannelStats>,
    pub is_opaque: bool,
    pub entropy: f64,
    pub sharpness: f64,
    pub dominant_red: f64,
    pub dominant_green: f64,
    pub dominant_blue: f64,
}

const STAT_MIN_INDEX: i32 = 0;
const STAT_MAX_INDEX: i32 = 1;
const STAT_SUM_INDEX: i32 = 2;
const STAT_SQ_SUM_INDEX: i32 = 3;
const STAT_MEAN_INDEX: i32 = 4;
const STAT_STDEV_INDEX: i32 = 5;
const STAT_MINX_INDEX: i32 = 6;
const STAT_MINY_INDEX: i32 = 7;
const STAT_MAXX_INDEX: i32 = 8;
const STAT_MAXY_INDEX: i32 = 9;

impl Sharp {
    pub fn stats(&self) -> Result<Stats> {
        let _guard = crate::util::VipsGuard;

        let mut input_option = self.options.input.clone();
        input_option.access = Access::Random;

        let (image, image_type) = open_input(&input_option)?;

        let mut stats = Stats {
            is_opaque: true,
            ..Default::default()
        };

        if image_type == ImageType::UNKNOWN {
            return Ok(stats);
        }

        let stats_image = image.stats()?;
        let bands = image.get_bands();

        for b in 1..bands + 1 {
            let cstats = ChannelStats {
                min: *(stats_image.getpoint(STAT_MIN_INDEX, b)?.first().unwrap_or(&0.0)) as _,
                max: *(stats_image.getpoint(STAT_MAX_INDEX, b)?.first().unwrap_or(&0.0)) as _,
                sum: *(stats_image.getpoint(STAT_SUM_INDEX, b)?.first().unwrap_or(&0.0)),
                squares_sum: *(stats_image.getpoint(STAT_SQ_SUM_INDEX, b)?.first().unwrap_or(&0.0)) as _,
                mean: *(stats_image.getpoint(STAT_MEAN_INDEX, b)?.first().unwrap_or(&0.0)),
                stdev: *(stats_image.getpoint(STAT_STDEV_INDEX, b)?.first().unwrap_or(&0.0)) as _,
                min_x: *(stats_image.getpoint(STAT_MINX_INDEX, b)?.first().unwrap_or(&0.0)) as _,
                min_y: *(stats_image.getpoint(STAT_MINY_INDEX, b)?.first().unwrap_or(&0.0)) as _,
                max_x: *(stats_image.getpoint(STAT_MAXX_INDEX, b)?.first().unwrap_or(&0.0)) as _,
                max_y: *(stats_image.getpoint(STAT_MAXY_INDEX, b)?.first().unwrap_or(&0.0)) as _,
            };
            stats.channel_stats.push(cstats);
        }

        // Image is not opaque when alpha layer is present and contains a non-mamixa value
        if image.image_hasalpha() {
            let min_alpha = *(stats_image.getpoint(STAT_MIN_INDEX, bands)?.first().unwrap_or(&0.0));
            if min_alpha != unsafe { vips_interpretation_max_alpha(image.get_interpretation()? as _) } {
                stats.is_opaque = false;
            }
        }

        // Convert to greyscale
        let greyscale = image.colourspace(Interpretation::BW)?.at(0);
        // Estimate entropy via histogram of greyscale value frequency
        stats.entropy = greyscale.hist_find()?.hist_entropy()?.abs();

        // Estimate sharpness via standard deviation of greyscale laplacian
        if image.get_width() > 1 || image.get_height() > 1 {
            let laplacian = VipsImage::new_matrixv(3, 3, &[0.0, 1.0, 0.0, 1.0, -4.0, 1.0, 0.0, 1.0, 0.0])?;
            laplacian.set_double("scale", 9.0);
            stats.sharpness = greyscale.conv(&laplacian)?.deviate()?;
        }

        // Most dominant sRGB colour via 4096-bin 3D histogram
        let hist = remove_alpha(image)?.colourspace(Interpretation::Srgb)?.hist_find_ndim_with_opts(VOption::new().set("bins", v_value!(16)))?;
        let maxpos = hist.maxpos()?;
        let dx = maxpos.0;
        let dy = maxpos.1;
        let pel = hist.getpoint(dx as _, dy as _)?;
        let max_val = pel.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let dz = pel.iter().position(|&x| x == max_val).unwrap() as f64;
        stats.dominant_red = dx * 16.0 + 8.0;
        stats.dominant_green = dy * 16.0 + 8.0;
        stats.dominant_blue = dz * 16.0 + 8.0;

        Ok(stats)
    }
}
