use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkyStar {
    pub name: String,
    pub bayer: Option<String>,
    pub constellation: String,
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub magnitude: f64,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectedStar {
    pub name: String,
    pub constellation: String,
    pub magnitude: f64,
    pub color: String,
    pub alt_deg: f64,
    pub az_deg: f64,
    pub x: f64,
    pub y: f64,
    pub is_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstellationLine {
    pub constellation: String,
    pub star_a: String,
    pub star_b: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstellationLabel {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub is_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundStar {
    pub x: f64,
    pub y: f64,
    pub magnitude: f64,
    pub is_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkyProjection {
    pub lat: f64,
    pub lon: f64,
    pub utc_time: String,
    pub lst_deg: f64,
    pub visible_stars: Vec<ProjectedStar>,
    pub background_stars: Vec<BackgroundStar>,
    pub lines: Vec<((f64, f64), (f64, f64), String)>, // (start, end, const_name)
    pub constellation_labels: Vec<ConstellationLabel>,
    pub cardinal_bearings: Vec<CardinalPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardinalPoint {
    pub label: String,
    pub az_deg: f64,
    pub x: f64,
    pub y: f64,
}

/// Calculate Julian Day from Year, Month, Day, UT Hour (fractional)
pub fn calculate_julian_day(year: i32, month: u32, day: u32, ut_hour: f64) -> f64 {
    let (y, m) = if month <= 2 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };

    let a = (y as f64 / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();

    (365.25 * (y as f64 + 4716.0)).floor()
        + (30.6001 * (m as f64 + 1.0)).floor()
        + day as f64
        + ut_hour / 24.0
        + b
        - 1524.5
}

/// Calculate Local Sidereal Time (LST) in degrees [0, 360)
pub fn calculate_lst(jd: f64, lon_deg: f64) -> f64 {
    let d0 = jd - 2451545.0;
    let mut gmst = 280.46061837 + 360.98564736629 * d0;
    gmst = gmst.rem_euclid(360.0);

    let lst = gmst + lon_deg;
    lst.rem_euclid(360.0)
}

/// Convert Equatorial (RA, Dec in deg) to Horizontal (Alt, Az in deg)
pub fn radec_to_altaz(ra_deg: f64, dec_deg: f64, lat_deg: f64, lst_deg: f64) -> (f64, f64) {
    let lat_rad = lat_deg.to_radians();
    let dec_rad = dec_deg.to_radians();
    let ha_rad = (lst_deg - ra_deg).rem_euclid(360.0).to_radians();

    let sin_alt = lat_rad.sin() * dec_rad.sin() + lat_rad.cos() * dec_rad.cos() * ha_rad.cos();
    let alt_rad = sin_alt.clamp(-1.0, 1.0).asin();
    let alt_deg = alt_rad.to_degrees();

    let cos_alt = alt_rad.cos().max(1e-6);
    let cos_az = (dec_rad.sin() - lat_rad.sin() * alt_rad.sin()) / (lat_rad.cos() * cos_alt);
    let sin_az = -dec_rad.cos() * ha_rad.sin() / cos_alt;

    let az_rad = sin_az.atan2(cos_az);
    let az_deg = az_rad.to_degrees().rem_euclid(360.0);

    (alt_deg, az_deg)
}

/// Project Horizontal (Alt, Az) to 2D Dome coordinates centered at (0, 0) with dome radius R
pub fn altaz_to_dome_xy(alt_deg: f64, az_deg: f64, dome_radius: f64) -> (f64, f64) {
    let r = dome_radius * (90.0 - alt_deg).max(0.0) / 90.0;
    let az_rad = az_deg.to_radians();

    // Astronomical zenith-centered dome: North = Top (0°), East = Left (90°), South = Bottom (180°), West = Right (270°)
    let x = -r * az_rad.sin();
    let y = -r * az_rad.cos();

    (x, y)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrightCatalogData {
    pub stars: Vec<SkyStar>,
    pub lines: Vec<ConstellationLine>,
}

/// Load the bright navigational star catalog and constellation stick figures from bright_stars.json
pub fn get_bright_catalog_data() -> BrightCatalogData {
    const RAW_JSON: &str = include_str!("../../../constellation/data/bright_stars.json");
    serde_json::from_str(RAW_JSON).expect("bright_stars.json must be valid JSON")
}

/// Comprehensive Navigational and Asterism Star Catalog (Yale Bright Star & Hipparcos derived)
pub fn get_bright_navigational_catalog() -> Vec<SkyStar> {
    get_bright_catalog_data().stars
}

/// Stick Figures / Vector Geometries for Constellations across the Sky
pub fn get_constellation_lines() -> Vec<ConstellationLine> {
    get_bright_catalog_data().lines
}

/// Deterministic Dense Background Starfield (800+ stars across celestial coordinates)
pub fn generate_background_stars(lat_deg: f64, lst_deg: f64, dome_radius: f64) -> Vec<BackgroundStar> {
    let mut bg_stars = Vec::with_capacity(800);

    // Generate distributed celestial field using spherical fibonacci grid
    let n = 900;
    let golden_ratio = (1.0 + 5.0_f64.sqrt()) / 2.0;

    for i in 0..n {
        let theta = 2.0 * std::f64::consts::PI * (i as f64) / golden_ratio;
        let phi = ((1.0 - 2.0 * (i as f64 + 0.5) / n as f64)).clamp(-1.0, 1.0).asin();

        let ra_deg = theta.to_degrees().rem_euclid(360.0);
        let dec_deg = phi.to_degrees();

        let (alt, az) = radec_to_altaz(ra_deg, dec_deg, lat_deg, lst_deg);
        if alt >= 0.0 {
            let (x, y) = altaz_to_dome_xy(alt, az, dome_radius);
            // Pseudo magnitude from 3.5 to 6.2
            let mag = 3.5 + ((i * 37) % 27) as f64 * 0.1;

            bg_stars.push(BackgroundStar {
                x,
                y,
                magnitude: mag,
                is_visible: true,
            });
        }
    }

    bg_stars
}

pub fn project_sky(
    lat_deg: f64,
    lon_deg: f64,
    year: i32,
    month: u32,
    day: u32,
    ut_hour: f64,
    dome_radius: f64,
) -> SkyProjection {
    let jd = calculate_julian_day(year, month, day, ut_hour);
    let lst = calculate_lst(jd, lon_deg);

    let catalog = get_bright_navigational_catalog();
    let mut visible_stars = Vec::new();
    let mut star_map = std::collections::HashMap::new();

    // Map constellation centroids
    let mut const_centroids: std::collections::HashMap<String, (f64, f64, usize, bool)> = std::collections::HashMap::new();

    for star in &catalog {
        let (alt, az) = radec_to_altaz(star.ra_deg, star.dec_deg, lat_deg, lst);
        let is_visible = alt >= -2.0;
        let (x, y) = altaz_to_dome_xy(alt, az, dome_radius);

        let p_star = ProjectedStar {
            name: star.name.clone(),
            constellation: star.constellation.clone(),
            magnitude: star.magnitude,
            color: star.color.clone(),
            alt_deg: alt,
            az_deg: az,
            x,
            y,
            is_visible,
        };

        star_map.insert(star.name.clone(), (x, y, is_visible, alt));
        visible_stars.push(p_star);

        if is_visible && alt >= 5.0 {
            let entry = const_centroids.entry(star.constellation.clone()).or_insert((0.0, 0.0, 0, true));
            entry.0 += x;
            entry.1 += y;
            entry.2 += 1;
        }
    }

    let line_defs = get_constellation_lines();
    let mut lines = Vec::new();

    for line in line_defs {
        if let (Some(s_a), Some(s_b)) = (star_map.get(&line.star_a), star_map.get(&line.star_b)) {
            if s_a.2 && s_b.2 {
                lines.push(((s_a.0, s_a.1), (s_b.0, s_b.1), line.constellation));
            }
        }
    }

    let mut constellation_labels = Vec::new();
    for (name, (sum_x, sum_y, count, _vis)) in const_centroids {
        if count >= 2 {
            constellation_labels.push(ConstellationLabel {
                name,
                x: sum_x / (count as f64),
                y: sum_y / (count as f64) + 14.0,
                is_visible: true,
            });
        }
    }

    let background_stars = generate_background_stars(lat_deg, lst, dome_radius);

    // Cardinal Points on the Horizon
    let cardinals = vec![
        ("N", 0.0),
        ("NE", 45.0),
        ("E", 90.0),
        ("SE", 135.0),
        ("S", 180.0),
        ("SW", 225.0),
        ("W", 270.0),
        ("NW", 315.0),
    ];

    let cardinal_bearings = cardinals
        .into_iter()
        .map(|(label, az)| {
            let (x, y) = altaz_to_dome_xy(0.0, az, dome_radius);
            CardinalPoint {
                label: label.into(),
                az_deg: az,
                x,
                y,
            }
        })
        .collect();

    let total_minutes = (ut_hour * 60.0).round() as i64;
    let display_hour = (total_minutes / 60).rem_euclid(24) as u32;
    let display_minute = (total_minutes % 60) as u32;

    SkyProjection {
        lat: lat_deg,
        lon: lon_deg,
        utc_time: format!("{:04}-{:02}-{:02} {:02}:{:02} UTC", year, month, day, display_hour, display_minute),
        lst_deg: lst,
        visible_stars,
        background_stars,
        lines,
        constellation_labels,
        cardinal_bearings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julian_day_benchmarks() {
        // Standard astronomical epoch J2000.0: 2000 Jan 1.5 (2000-01-01 12:00:00 UT) = JD 2451545.0
        let jd_j2000 = calculate_julian_day(2000, 1, 1, 12.0);
        assert!((jd_j2000 - 2451545.0).abs() < 1e-5, "J2000 JD mismatch: {}", jd_j2000);

        // Jean Meeus Astronomical Algorithms Example: 1987-01-27 00:00 UT = JD 2446822.5
        let jd_1987 = calculate_julian_day(1987, 1, 27, 0.0);
        assert!((jd_1987 - 2446822.5).abs() < 1e-5, "1987 JD mismatch: {}", jd_1987);

        // 1957 Oct 4.81 UT (Sputnik 1 launch) = JD 2436116.31
        let jd_sputnik = calculate_julian_day(1957, 10, 4, 0.81 * 24.0);
        assert!((jd_sputnik - 2436116.31).abs() < 0.01, "Sputnik JD mismatch: {}", jd_sputnik);
    }

    #[test]
    fn test_sidereal_time_and_lst_benchmarks() {
        // At J2000.0 (JD = 2451545.0), GMST = 280.46061837° (approx 18h 41m 50.5s)
        let gmst_j2000 = calculate_lst(2451545.0, 0.0);
        assert!((gmst_j2000 - 280.4606).abs() < 0.01, "J2000 GMST mismatch: {}", gmst_j2000);

        // Tokyo (Lon +139.6503°): LST should be (GMST + Lon) mod 360
        let lst_tokyo = calculate_lst(2451545.0, 139.6503);
        let expected_tokyo = (280.46061837 + 139.6503) % 360.0;
        assert!((lst_tokyo - expected_tokyo).abs() < 0.01, "Tokyo LST mismatch: {}", lst_tokyo);
    }

    #[test]
    fn test_vega_ephemeris_altaz_ground_truth() {
        // Vega (Alpha Lyrae): RA = 279.234°, Dec = +38.7836°
        let vega_ra = 279.234;
        let vega_dec = 38.7836;

        // Observer at Royal Observatory Greenwich: Lat = 51.4769° N, Lon = 0.0°
        // At epoch J2000.0 (2000-01-01 12:00:00 UT), LST = 280.4606°
        // Hour Angle H = LST - RA = 280.4606 - 279.234 = +1.2266° (near upper meridian transit)
        let lat = 51.4769;
        let lst = 280.4606;
        let (alt, az) = radec_to_altaz(vega_ra, vega_dec, lat, lst);

        // Analytical meridian transit altitude = 90° - Lat + Dec = 90 - 51.4769 + 38.7836 = 77.3067°
        // With H = +1.2266° (past upper meridian), Alt is 77.28° and Azimuth is 184.35° (4.35° west of south)
        assert!((alt - 77.28).abs() < 0.15, "Vega Altitude outside 0.15° ephemeris tolerance: actual = {}", alt);
        assert!((az - 184.35).abs() < 0.15, "Vega Azimuth outside 0.15° ephemeris tolerance: actual = {}", az);
    }

    #[test]
    fn test_polaris_alignment_ground_truth() {
        // Polaris (Alpha Ursae Minoris): RA = 37.95°, Dec = +89.26°
        let polaris_ra = 37.95;
        let polaris_dec = 89.26;

        // Observer in Tokyo: Lat = 35.6762° N, Lon = 139.6503° E
        // Polaris altitude must closely match observer's latitude within declination offset (~0.74°)
        let lat = 35.6762;
        let jd = calculate_julian_day(2026, 9, 30, 13.0);
        let lst = calculate_lst(jd, 139.6503);
        let (alt, az) = radec_to_altaz(polaris_ra, polaris_dec, lat, lst);

        assert!((alt - lat).abs() < 0.8, "Polaris altitude {} deviated from observer latitude {}", alt, lat);
        // Azimuth should point true North (within ~1.5° of 0°/360°)
        let north_deviation = if az > 180.0 { 360.0 - az } else { az };
        assert!(north_deviation < 1.8, "Polaris azimuth {} deviated from North by > 1.8°", az);
    }
}

