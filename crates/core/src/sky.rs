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

/// Comprehensive Navigational and Asterism Star Catalog (Yale Bright Star & Hipparcos derived)
pub fn get_bright_navigational_catalog() -> Vec<SkyStar> {
    vec![
        // URSA MINOR (Little Dipper)
        SkyStar { name: "Polaris".into(), bayer: Some("Alpha UMi".into()), constellation: "Ursa Minor".into(), ra_deg: 37.95, dec_deg: 89.26, magnitude: 1.98, color: "#fef3c7".into() },
        SkyStar { name: "Kochab".into(), bayer: Some("Beta UMi".into()), constellation: "Ursa Minor".into(), ra_deg: 222.68, dec_deg: 74.16, magnitude: 2.08, color: "#fed7aa".into() },
        SkyStar { name: "Pherkad".into(), bayer: Some("Gamma UMi".into()), constellation: "Ursa Minor".into(), ra_deg: 230.18, dec_deg: 71.83, magnitude: 3.05, color: "#fef3c7".into() },
        SkyStar { name: "Yildun".into(), bayer: Some("Delta UMi".into()), constellation: "Ursa Minor".into(), ra_deg: 262.70, dec_deg: 86.59, magnitude: 4.35, color: "#ffffff".into() },
        SkyStar { name: "Urodelus".into(), bayer: Some("Epsilon UMi".into()), constellation: "Ursa Minor".into(), ra_deg: 254.44, dec_deg: 82.04, magnitude: 4.21, color: "#fed7aa".into() },
        SkyStar { name: "Ahfa al Farkadain".into(), bayer: Some("Zeta UMi".into()), constellation: "Ursa Minor".into(), ra_deg: 237.28, dec_deg: 77.79, magnitude: 4.29, color: "#ffffff".into() },
        SkyStar { name: "Anwar al Farkadain".into(), bayer: Some("Eta UMi".into()), constellation: "Ursa Minor".into(), ra_deg: 244.60, dec_deg: 75.76, magnitude: 4.95, color: "#ffffff".into() },
        
        // URSA MAJOR (Big Dipper / Great Bear)
        SkyStar { name: "Dubhe".into(), bayer: Some("Alpha UMa".into()), constellation: "Ursa Major".into(), ra_deg: 165.93, dec_deg: 61.75, magnitude: 1.79, color: "#fdba74".into() },
        SkyStar { name: "Merak".into(), bayer: Some("Beta UMa".into()), constellation: "Ursa Major".into(), ra_deg: 165.46, dec_deg: 56.38, magnitude: 2.37, color: "#ffffff".into() },
        SkyStar { name: "Phecda".into(), bayer: Some("Gamma UMa".into()), constellation: "Ursa Major".into(), ra_deg: 178.46, dec_deg: 53.69, magnitude: 2.44, color: "#ffffff".into() },
        SkyStar { name: "Megrez".into(), bayer: Some("Delta UMa".into()), constellation: "Ursa Major".into(), ra_deg: 183.86, dec_deg: 57.03, magnitude: 3.31, color: "#ffffff".into() },
        SkyStar { name: "Alioth".into(), bayer: Some("Epsilon UMa".into()), constellation: "Ursa Major".into(), ra_deg: 193.51, dec_deg: 55.96, magnitude: 1.77, color: "#ffffff".into() },
        SkyStar { name: "Mizar".into(), bayer: Some("Zeta UMa".into()), constellation: "Ursa Major".into(), ra_deg: 200.98, dec_deg: 54.92, magnitude: 2.23, color: "#ffffff".into() },
        SkyStar { name: "Alcor".into(), bayer: Some("80 UMa".into()), constellation: "Ursa Major".into(), ra_deg: 201.30, dec_deg: 54.99, magnitude: 3.99, color: "#ffffff".into() },
        SkyStar { name: "Alkaid".into(), bayer: Some("Eta UMa".into()), constellation: "Ursa Major".into(), ra_deg: 206.89, dec_deg: 49.31, magnitude: 1.86, color: "#bae6fd".into() },
        SkyStar { name: "Talitha".into(), bayer: Some("Iota UMa".into()), constellation: "Ursa Major".into(), ra_deg: 134.79, dec_deg: 48.04, magnitude: 3.12, color: "#ffffff".into() },
        SkyStar { name: "Tania Australis".into(), bayer: Some("Mu UMa".into()), constellation: "Ursa Major".into(), ra_deg: 154.57, dec_deg: 41.50, magnitude: 3.06, color: "#f87171".into() },

        // ORION (The Hunter)
        SkyStar { name: "Betelgeuse".into(), bayer: Some("Alpha Ori".into()), constellation: "Orion".into(), ra_deg: 88.79, dec_deg: 7.41, magnitude: 0.50, color: "#f87171".into() },
        SkyStar { name: "Rigel".into(), bayer: Some("Beta Ori".into()), constellation: "Orion".into(), ra_deg: 78.63, dec_deg: -8.20, magnitude: 0.13, color: "#93c5fd".into() },
        SkyStar { name: "Bellatrix".into(), bayer: Some("Gamma Ori".into()), constellation: "Orion".into(), ra_deg: 81.28, dec_deg: 6.35, magnitude: 1.64, color: "#bae6fd".into() },
        SkyStar { name: "Saiph".into(), bayer: Some("Kappa Ori".into()), constellation: "Orion".into(), ra_deg: 86.94, dec_deg: -9.67, magnitude: 2.07, color: "#93c5fd".into() },
        SkyStar { name: "Alnitak".into(), bayer: Some("Zeta Ori".into()), constellation: "Orion".into(), ra_deg: 85.19, dec_deg: -1.94, magnitude: 1.77, color: "#bae6fd".into() },
        SkyStar { name: "Alnilam".into(), bayer: Some("Epsilon Ori".into()), constellation: "Orion".into(), ra_deg: 84.05, dec_deg: -1.20, magnitude: 1.69, color: "#bae6fd".into() },
        SkyStar { name: "Mintaka".into(), bayer: Some("Delta Ori".into()), constellation: "Orion".into(), ra_deg: 83.00, dec_deg: -0.30, magnitude: 2.23, color: "#bae6fd".into() },
        SkyStar { name: "Meissa".into(), bayer: Some("Lambda Ori".into()), constellation: "Orion".into(), ra_deg: 83.78, dec_deg: 9.93, magnitude: 3.39, color: "#93c5fd".into() },

        // CASSIOPEIA (The Queen)
        SkyStar { name: "Schedar".into(), bayer: Some("Alpha Cas".into()), constellation: "Cassiopeia".into(), ra_deg: 10.13, dec_deg: 56.54, magnitude: 2.24, color: "#fed7aa".into() },
        SkyStar { name: "Caph".into(), bayer: Some("Beta Cas".into()), constellation: "Cassiopeia".into(), ra_deg: 2.29, dec_deg: 59.15, magnitude: 2.28, color: "#ffffff".into() },
        SkyStar { name: "Gamma Cas".into(), bayer: Some("Gamma Cas".into()), constellation: "Cassiopeia".into(), ra_deg: 14.18, dec_deg: 60.72, magnitude: 2.47, color: "#bae6fd".into() },
        SkyStar { name: "Ruchbah".into(), bayer: Some("Delta Cas".into()), constellation: "Cassiopeia".into(), ra_deg: 21.45, dec_deg: 60.23, magnitude: 2.68, color: "#ffffff".into() },
        SkyStar { name: "Segin".into(), bayer: Some("Epsilon Cas".into()), constellation: "Cassiopeia".into(), ra_deg: 26.10, dec_deg: 63.67, magnitude: 3.35, color: "#bae6fd".into() },
        SkyStar { name: "Achird".into(), bayer: Some("Eta Cas".into()), constellation: "Cassiopeia".into(), ra_deg: 12.27, dec_deg: 57.82, magnitude: 3.44, color: "#fef08a".into() },

        // CRUX (Southern Cross) & CENTAURUS
        SkyStar { name: "Acrux".into(), bayer: Some("Alpha Cru".into()), constellation: "Crux".into(), ra_deg: 186.65, dec_deg: -63.10, magnitude: 0.77, color: "#93c5fd".into() },
        SkyStar { name: "Gacrux".into(), bayer: Some("Gamma Cru".into()), constellation: "Crux".into(), ra_deg: 187.79, dec_deg: -57.11, magnitude: 1.64, color: "#f87171".into() },
        SkyStar { name: "Mimosa".into(), bayer: Some("Beta Cru".into()), constellation: "Crux".into(), ra_deg: 191.93, dec_deg: -59.69, magnitude: 1.25, color: "#93c5fd".into() },
        SkyStar { name: "Imai".into(), bayer: Some("Delta Cru".into()), constellation: "Crux".into(), ra_deg: 183.79, dec_deg: -58.75, magnitude: 2.78, color: "#bae6fd".into() },
        SkyStar { name: "Alpha Centauri".into(), bayer: Some("Alpha Cen".into()), constellation: "Centaurus".into(), ra_deg: 219.90, dec_deg: -60.83, magnitude: -0.01, color: "#fef08a".into() },
        SkyStar { name: "Hadar".into(), bayer: Some("Beta Cen".into()), constellation: "Centaurus".into(), ra_deg: 210.96, dec_deg: -60.37, magnitude: 0.61, color: "#bae6fd".into() },
        SkyStar { name: "Menkent".into(), bayer: Some("Theta Cen".into()), constellation: "Centaurus".into(), ra_deg: 211.60, dec_deg: -36.37, magnitude: 2.06, color: "#fed7aa".into() },

        // CANIS MAJOR & CANIS MINOR
        SkyStar { name: "Sirius".into(), bayer: Some("Alpha CMa".into()), constellation: "Canis Major".into(), ra_deg: 101.29, dec_deg: -16.72, magnitude: -1.46, color: "#ffffff".into() },
        SkyStar { name: "Adhara".into(), bayer: Some("Epsilon CMa".into()), constellation: "Canis Major".into(), ra_deg: 104.66, dec_deg: -28.97, magnitude: 1.50, color: "#93c5fd".into() },
        SkyStar { name: "Wezen".into(), bayer: Some("Delta CMa".into()), constellation: "Canis Major".into(), ra_deg: 107.10, dec_deg: -26.39, magnitude: 1.83, color: "#fef08a".into() },
        SkyStar { name: "Mirzam".into(), bayer: Some("Beta CMa".into()), constellation: "Canis Major".into(), ra_deg: 95.68, dec_deg: -17.96, magnitude: 1.98, color: "#bae6fd".into() },
        SkyStar { name: "Aludra".into(), bayer: Some("Eta CMa".into()), constellation: "Canis Major".into(), ra_deg: 111.02, dec_deg: -29.30, magnitude: 2.45, color: "#93c5fd".into() },
        SkyStar { name: "Procyon".into(), bayer: Some("Alpha CMi".into()), constellation: "Canis Minor".into(), ra_deg: 114.83, dec_deg: 5.22, magnitude: 0.38, color: "#fef3c7".into() },
        SkyStar { name: "Gomeisa".into(), bayer: Some("Beta CMi".into()), constellation: "Canis Minor".into(), ra_deg: 111.79, dec_deg: 8.29, magnitude: 2.89, color: "#bae6fd".into() },

        // SUMMER TRIANGLE & CYGNUS & LYRA & AQUILA
        SkyStar { name: "Vega".into(), bayer: Some("Alpha Lyr".into()), constellation: "Lyra".into(), ra_deg: 279.23, dec_deg: 38.78, magnitude: 0.03, color: "#bae6fd".into() },
        SkyStar { name: "Sheliak".into(), bayer: Some("Beta Lyr".into()), constellation: "Lyra".into(), ra_deg: 282.52, dec_deg: 33.36, magnitude: 3.52, color: "#bae6fd".into() },
        SkyStar { name: "Sulafat".into(), bayer: Some("Gamma Lyr".into()), constellation: "Lyra".into(), ra_deg: 283.60, dec_deg: 32.69, magnitude: 3.25, color: "#bae6fd".into() },
        SkyStar { name: "Deneb".into(), bayer: Some("Alpha Cyg".into()), constellation: "Cygnus".into(), ra_deg: 310.36, dec_deg: 45.28, magnitude: 1.25, color: "#ffffff".into() },
        SkyStar { name: "Sadr".into(), bayer: Some("Gamma Cyg".into()), constellation: "Cygnus".into(), ra_deg: 305.56, dec_deg: 40.26, magnitude: 2.23, color: "#ffffff".into() },
        SkyStar { name: "Gienah Cyg".into(), bayer: Some("Epsilon Cyg".into()), constellation: "Cygnus".into(), ra_deg: 311.55, dec_deg: 33.97, magnitude: 2.48, color: "#fed7aa".into() },
        SkyStar { name: "Albireo".into(), bayer: Some("Beta Cyg".into()), constellation: "Cygnus".into(), ra_deg: 292.68, dec_deg: 27.96, magnitude: 3.05, color: "#fde047".into() },
        SkyStar { name: "Delta Cyg".into(), bayer: Some("Delta Cyg".into()), constellation: "Cygnus".into(), ra_deg: 296.24, dec_deg: 45.13, magnitude: 2.87, color: "#ffffff".into() },
        SkyStar { name: "Altair".into(), bayer: Some("Alpha Aql".into()), constellation: "Aquila".into(), ra_deg: 297.70, dec_deg: 8.87, magnitude: 0.77, color: "#ffffff".into() },
        SkyStar { name: "Tarazed".into(), bayer: Some("Gamma Aql".into()), constellation: "Aquila".into(), ra_deg: 296.54, dec_deg: 10.61, magnitude: 2.72, color: "#fed7aa".into() },
        SkyStar { name: "Alshain".into(), bayer: Some("Beta Aql".into()), constellation: "Aquila".into(), ra_deg: 298.83, dec_deg: 6.41, magnitude: 3.71, color: "#fef08a".into() },

        // TAURUS & AURIGA & BOÖTES & CORONA BOREALIS
        SkyStar { name: "Aldebaran".into(), bayer: Some("Alpha Tau".into()), constellation: "Taurus".into(), ra_deg: 68.98, dec_deg: 16.51, magnitude: 0.85, color: "#f97316".into() },
        SkyStar { name: "Elnath".into(), bayer: Some("Beta Tau".into()), constellation: "Taurus".into(), ra_deg: 81.57, dec_deg: 28.61, magnitude: 1.65, color: "#bae6fd".into() },
        SkyStar { name: "Alcyone".into(), bayer: Some("Eta Tau".into()), constellation: "Taurus".into(), ra_deg: 56.87, dec_deg: 24.11, magnitude: 2.87, color: "#93c5fd".into() },
        SkyStar { name: "Capella".into(), bayer: Some("Alpha Aur".into()), constellation: "Auriga".into(), ra_deg: 79.17, dec_deg: 46.00, magnitude: 0.08, color: "#fef08a".into() },
        SkyStar { name: "Menkalinan".into(), bayer: Some("Beta Aur".into()), constellation: "Auriga".into(), ra_deg: 89.88, dec_deg: 44.95, magnitude: 1.90, color: "#ffffff".into() },
        SkyStar { name: "Arcturus".into(), bayer: Some("Alpha Boo".into()), constellation: "Bootes".into(), ra_deg: 213.92, dec_deg: 19.18, magnitude: -0.05, color: "#fdba74".into() },
        SkyStar { name: "Izar".into(), bayer: Some("Epsilon Boo".into()), constellation: "Bootes".into(), ra_deg: 221.25, dec_deg: 27.07, magnitude: 2.37, color: "#f97316".into() },
        SkyStar { name: "Muphrid".into(), bayer: Some("Eta Boo".into()), constellation: "Bootes".into(), ra_deg: 208.74, dec_deg: 18.39, magnitude: 2.68, color: "#fef08a".into() },
        SkyStar { name: "Alphecca".into(), bayer: Some("Alpha CrB".into()), constellation: "Corona Borealis".into(), ra_deg: 233.67, dec_deg: 26.71, magnitude: 2.22, color: "#ffffff".into() },

        // SCORPIO & SAGITTARIUS (The Heart of the Galaxy)
        SkyStar { name: "Antares".into(), bayer: Some("Alpha Sco".into()), constellation: "Scorpio".into(), ra_deg: 247.35, dec_deg: -26.43, magnitude: 1.06, color: "#ef4444".into() },
        SkyStar { name: "Shaula".into(), bayer: Some("Lambda Sco".into()), constellation: "Scorpio".into(), ra_deg: 263.40, dec_deg: -37.10, magnitude: 1.62, color: "#93c5fd".into() },
        SkyStar { name: "Sargas".into(), bayer: Some("Theta Sco".into()), constellation: "Scorpio".into(), ra_deg: 264.33, dec_deg: -42.99, magnitude: 1.86, color: "#fde047".into() },
        SkyStar { name: "Dschubba".into(), bayer: Some("Delta Sco".into()), constellation: "Scorpio".into(), ra_deg: 240.08, dec_deg: -22.62, magnitude: 2.29, color: "#bae6fd".into() },
        SkyStar { name: "Graffias".into(), bayer: Some("Beta Sco".into()), constellation: "Scorpio".into(), ra_deg: 241.36, dec_deg: -19.81, magnitude: 2.56, color: "#bae6fd".into() },
        SkyStar { name: "Lesath".into(), bayer: Some("Upsilon Sco".into()), constellation: "Scorpio".into(), ra_deg: 262.69, dec_deg: -37.29, magnitude: 2.70, color: "#93c5fd".into() },
        SkyStar { name: "Kaus Australis".into(), bayer: Some("Epsilon Sgr".into()), constellation: "Sagittarius".into(), ra_deg: 276.04, dec_deg: -34.38, magnitude: 1.79, color: "#bae6fd".into() },
        SkyStar { name: "Nunki".into(), bayer: Some("Sigma Sgr".into()), constellation: "Sagittarius".into(), ra_deg: 283.82, dec_deg: -26.30, magnitude: 2.05, color: "#93c5fd".into() },
        SkyStar { name: "Ascella".into(), bayer: Some("Zeta Sgr".into()), constellation: "Sagittarius".into(), ra_deg: 285.42, dec_deg: -29.88, magnitude: 2.60, color: "#ffffff".into() },
        SkyStar { name: "Kaus Media".into(), bayer: Some("Delta Sgr".into()), constellation: "Sagittarius".into(), ra_deg: 275.24, dec_deg: -29.83, magnitude: 2.72, color: "#fed7aa".into() },
        SkyStar { name: "Kaus Borealis".into(), bayer: Some("Lambda Sgr".into()), constellation: "Sagittarius".into(), ra_deg: 276.99, dec_deg: -25.42, magnitude: 2.82, color: "#fed7aa".into() },
        SkyStar { name: "Alnasl".into(), bayer: Some("Gamma Sgr".into()), constellation: "Sagittarius".into(), ra_deg: 271.45, dec_deg: -30.42, magnitude: 2.98, color: "#fed7aa".into() },

        // LEO & VIRGO & LIBRA & CANCER & GEMINI
        SkyStar { name: "Regulus".into(), bayer: Some("Alpha Leo".into()), constellation: "Leo".into(), ra_deg: 152.09, dec_deg: 11.97, magnitude: 1.36, color: "#bae6fd".into() },
        SkyStar { name: "Denebola".into(), bayer: Some("Beta Leo".into()), constellation: "Leo".into(), ra_deg: 177.26, dec_deg: 14.57, magnitude: 2.14, color: "#ffffff".into() },
        SkyStar { name: "Algieba".into(), bayer: Some("Gamma Leo".into()), constellation: "Leo".into(), ra_deg: 154.99, dec_deg: 19.84, magnitude: 2.01, color: "#fde047".into() },
        SkyStar { name: "Zosma".into(), bayer: Some("Delta Leo".into()), constellation: "Leo".into(), ra_deg: 168.53, dec_deg: 20.52, magnitude: 2.56, color: "#ffffff".into() },
        SkyStar { name: "Spica".into(), bayer: Some("Alpha Vir".into()), constellation: "Virgo".into(), ra_deg: 201.30, dec_deg: -11.16, magnitude: 0.98, color: "#93c5fd".into() },
        SkyStar { name: "Vindemiatrix".into(), bayer: Some("Epsilon Vir".into()), constellation: "Virgo".into(), ra_deg: 195.54, dec_deg: 10.96, magnitude: 2.85, color: "#fef08a".into() },
        SkyStar { name: "Porrima".into(), bayer: Some("Gamma Vir".into()), constellation: "Virgo".into(), ra_deg: 190.43, dec_deg: -1.45, magnitude: 2.74, color: "#fef3c7".into() },
        SkyStar { name: "Zubeneschamali".into(), bayer: Some("Beta Lib".into()), constellation: "Libra".into(), ra_deg: 229.25, dec_deg: -9.38, magnitude: 2.61, color: "#86efac".into() },
        SkyStar { name: "Zubenelgenubi".into(), bayer: Some("Alpha Lib".into()), constellation: "Libra".into(), ra_deg: 222.72, dec_deg: -16.04, magnitude: 2.75, color: "#ffffff".into() },
        SkyStar { name: "Pollux".into(), bayer: Some("Beta Gem".into()), constellation: "Gemini".into(), ra_deg: 116.17, dec_deg: 28.03, magnitude: 1.15, color: "#fed7aa".into() },
        SkyStar { name: "Castor".into(), bayer: Some("Alpha Gem".into()), constellation: "Gemini".into(), ra_deg: 113.65, dec_deg: 31.89, magnitude: 1.58, color: "#ffffff".into() },
        SkyStar { name: "Alhena".into(), bayer: Some("Gamma Gem".into()), constellation: "Gemini".into(), ra_deg: 99.43, dec_deg: 16.40, magnitude: 1.93, color: "#ffffff".into() },
        SkyStar { name: "Acubens".into(), bayer: Some("Alpha Cnc".into()), constellation: "Cancer".into(), ra_deg: 134.58, dec_deg: 11.86, magnitude: 4.26, color: "#ffffff".into() },
        SkyStar { name: "Altarf".into(), bayer: Some("Beta Cnc".into()), constellation: "Cancer".into(), ra_deg: 124.13, dec_deg: 9.19, magnitude: 3.53, color: "#fed7aa".into() },

        // CAPRICORN & AQUARIUS & PISCES & ARIES & PEGASUS & ANDROMEDA
        SkyStar { name: "Deneb Algedi".into(), bayer: Some("Delta Cap".into()), constellation: "Capricorn".into(), ra_deg: 326.76, dec_deg: -16.13, magnitude: 2.85, color: "#ffffff".into() },
        SkyStar { name: "Dabih".into(), bayer: Some("Beta Cap".into()), constellation: "Capricorn".into(), ra_deg: 305.25, dec_deg: -14.78, magnitude: 3.05, color: "#fef08a".into() },
        SkyStar { name: "Sadalsuud".into(), bayer: Some("Beta Aqr".into()), constellation: "Aquarius".into(), ra_deg: 322.89, dec_deg: -5.57, magnitude: 2.90, color: "#fef08a".into() },
        SkyStar { name: "Sadalmelik".into(), bayer: Some("Alpha Aqr".into()), constellation: "Aquarius".into(), ra_deg: 331.45, dec_deg: -0.32, magnitude: 2.95, color: "#fef08a".into() },
        SkyStar { name: "Skat".into(), bayer: Some("Delta Aqr".into()), constellation: "Aquarius".into(), ra_deg: 343.66, dec_deg: -15.82, magnitude: 3.27, color: "#ffffff".into() },
        SkyStar { name: "Alrescha".into(), bayer: Some("Alpha Psc".into()), constellation: "Pisces".into(), ra_deg: 30.51, dec_deg: 2.76, magnitude: 3.82, color: "#ffffff".into() },
        SkyStar { name: "Hamal".into(), bayer: Some("Alpha Ari".into()), constellation: "Aries".into(), ra_deg: 31.79, dec_deg: 23.46, magnitude: 2.01, color: "#fed7aa".into() },
        SkyStar { name: "Sheratan".into(), bayer: Some("Beta Ari".into()), constellation: "Aries".into(), ra_deg: 28.66, dec_deg: 20.81, magnitude: 2.64, color: "#ffffff".into() },
        SkyStar { name: "Markab".into(), bayer: Some("Alpha Peg".into()), constellation: "Pegasus".into(), ra_deg: 346.19, dec_deg: 15.21, magnitude: 2.49, color: "#bae6fd".into() },
        SkyStar { name: "Scheat".into(), bayer: Some("Beta Peg".into()), constellation: "Pegasus".into(), ra_deg: 345.94, dec_deg: 28.08, magnitude: 2.44, color: "#fca5a5".into() },
        SkyStar { name: "Alpheratz".into(), bayer: Some("Alpha And".into()), constellation: "Andromeda".into(), ra_deg: 2.10, dec_deg: 29.09, magnitude: 2.07, color: "#bae6fd".into() },
        SkyStar { name: "Algenib".into(), bayer: Some("Gamma Peg".into()), constellation: "Pegasus".into(), ra_deg: 3.31, dec_deg: 15.18, magnitude: 2.84, color: "#bae6fd".into() },
        SkyStar { name: "Enif".into(), bayer: Some("Epsilon Peg".into()), constellation: "Pegasus".into(), ra_deg: 326.05, dec_deg: 9.87, magnitude: 2.38, color: "#fed7aa".into() },
        SkyStar { name: "Mirach".into(), bayer: Some("Beta And".into()), constellation: "Andromeda".into(), ra_deg: 17.43, dec_deg: 35.62, magnitude: 2.07, color: "#f87171".into() },
        SkyStar { name: "Almach".into(), bayer: Some("Gamma And".into()), constellation: "Andromeda".into(), ra_deg: 30.97, dec_deg: 42.33, magnitude: 2.10, color: "#fed7aa".into() },

        // SOUTHERN JEWELS & WAYFINDING
        SkyStar { name: "Canopus".into(), bayer: Some("Alpha Car".into()), constellation: "Carina".into(), ra_deg: 95.99, dec_deg: -52.70, magnitude: -0.74, color: "#ffffff".into() },
        SkyStar { name: "Miaplacidus".into(), bayer: Some("Beta Car".into()), constellation: "Carina".into(), ra_deg: 138.30, dec_deg: -69.72, magnitude: 1.67, color: "#ffffff".into() },
        SkyStar { name: "Avior".into(), bayer: Some("Epsilon Car".into()), constellation: "Carina".into(), ra_deg: 125.63, dec_deg: -59.51, magnitude: 1.86, color: "#fed7aa".into() },
        SkyStar { name: "Aspidiske".into(), bayer: Some("Iota Car".into()), constellation: "Carina".into(), ra_deg: 139.27, dec_deg: -59.28, magnitude: 2.21, color: "#ffffff".into() },
        SkyStar { name: "Achernar".into(), bayer: Some("Alpha Eri".into()), constellation: "Eridanus".into(), ra_deg: 24.43, dec_deg: -57.24, magnitude: 0.45, color: "#93c5fd".into() },
        SkyStar { name: "Fomalhaut".into(), bayer: Some("Alpha PsA".into()), constellation: "Piscis Austrinus".into(), ra_deg: 344.41, dec_deg: -29.62, magnitude: 1.17, color: "#bae6fd".into() },
        SkyStar { name: "Alnair".into(), bayer: Some("Alpha Gru".into()), constellation: "Grus".into(), ra_deg: 332.06, dec_deg: -46.96, magnitude: 1.73, color: "#bae6fd".into() },
        SkyStar { name: "Peacock".into(), bayer: Some("Alpha Pav".into()), constellation: "Pavo".into(), ra_deg: 306.41, dec_deg: -56.74, magnitude: 1.94, color: "#93c5fd".into() },
        SkyStar { name: "Ankaa".into(), bayer: Some("Alpha Phe".into()), constellation: "Phoenix".into(), ra_deg: 6.57, dec_deg: -42.31, magnitude: 2.40, color: "#fed7aa".into() },
    ]
}

/// Stick Figures / Vector Geometries for Constellations across the Sky
pub fn get_constellation_lines() -> Vec<ConstellationLine> {
    vec![
        // Ursa Minor
        ConstellationLine { constellation: "Ursa Minor".into(), star_a: "Polaris".into(), star_b: "Yildun".into() },
        ConstellationLine { constellation: "Ursa Minor".into(), star_a: "Yildun".into(), star_b: "Urodelus".into() },
        ConstellationLine { constellation: "Ursa Minor".into(), star_a: "Urodelus".into(), star_b: "Ahfa al Farkadain".into() },
        ConstellationLine { constellation: "Ursa Minor".into(), star_a: "Ahfa al Farkadain".into(), star_b: "Kochab".into() },
        ConstellationLine { constellation: "Ursa Minor".into(), star_a: "Kochab".into(), star_b: "Pherkad".into() },
        ConstellationLine { constellation: "Ursa Minor".into(), star_a: "Pherkad".into(), star_b: "Anwar al Farkadain".into() },
        ConstellationLine { constellation: "Ursa Minor".into(), star_a: "Anwar al Farkadain".into(), star_b: "Ahfa al Farkadain".into() },

        // Ursa Major
        ConstellationLine { constellation: "Ursa Major".into(), star_a: "Dubhe".into(), star_b: "Merak".into() },
        ConstellationLine { constellation: "Ursa Major".into(), star_a: "Merak".into(), star_b: "Phecda".into() },
        ConstellationLine { constellation: "Ursa Major".into(), star_a: "Phecda".into(), star_b: "Megrez".into() },
        ConstellationLine { constellation: "Ursa Major".into(), star_a: "Megrez".into(), star_b: "Dubhe".into() },
        ConstellationLine { constellation: "Ursa Major".into(), star_a: "Megrez".into(), star_b: "Alioth".into() },
        ConstellationLine { constellation: "Ursa Major".into(), star_a: "Alioth".into(), star_b: "Mizar".into() },
        ConstellationLine { constellation: "Ursa Major".into(), star_a: "Mizar".into(), star_b: "Alkaid".into() },

        // Orion
        ConstellationLine { constellation: "Orion".into(), star_a: "Betelgeuse".into(), star_b: "Bellatrix".into() },
        ConstellationLine { constellation: "Orion".into(), star_a: "Betelgeuse".into(), star_b: "Alnitak".into() },
        ConstellationLine { constellation: "Orion".into(), star_a: "Bellatrix".into(), star_b: "Mintaka".into() },
        ConstellationLine { constellation: "Orion".into(), star_a: "Alnitak".into(), star_b: "Alnilam".into() },
        ConstellationLine { constellation: "Orion".into(), star_a: "Alnilam".into(), star_b: "Mintaka".into() },
        ConstellationLine { constellation: "Orion".into(), star_a: "Alnitak".into(), star_b: "Saiph".into() },
        ConstellationLine { constellation: "Orion".into(), star_a: "Mintaka".into(), star_b: "Rigel".into() },
        ConstellationLine { constellation: "Orion".into(), star_a: "Saiph".into(), star_b: "Rigel".into() },
        ConstellationLine { constellation: "Orion".into(), star_a: "Betelgeuse".into(), star_b: "Meissa".into() },
        ConstellationLine { constellation: "Orion".into(), star_a: "Bellatrix".into(), star_b: "Meissa".into() },

        // Cassiopeia
        ConstellationLine { constellation: "Cassiopeia".into(), star_a: "Caph".into(), star_b: "Schedar".into() },
        ConstellationLine { constellation: "Cassiopeia".into(), star_a: "Schedar".into(), star_b: "Gamma Cas".into() },
        ConstellationLine { constellation: "Cassiopeia".into(), star_a: "Gamma Cas".into(), star_b: "Ruchbah".into() },
        ConstellationLine { constellation: "Cassiopeia".into(), star_a: "Ruchbah".into(), star_b: "Segin".into() },

        // Crux & Centaurus
        ConstellationLine { constellation: "Crux".into(), star_a: "Gacrux".into(), star_b: "Acrux".into() },
        ConstellationLine { constellation: "Crux".into(), star_a: "Mimosa".into(), star_b: "Imai".into() },
        ConstellationLine { constellation: "Centaurus".into(), star_a: "Alpha Centauri".into(), star_b: "Hadar".into() },

        // Canis Major / Minor
        ConstellationLine { constellation: "Canis Major".into(), star_a: "Sirius".into(), star_b: "Mirzam".into() },
        ConstellationLine { constellation: "Canis Major".into(), star_a: "Sirius".into(), star_b: "Wezen".into() },
        ConstellationLine { constellation: "Canis Major".into(), star_a: "Wezen".into(), star_b: "Adhara".into() },
        ConstellationLine { constellation: "Canis Major".into(), star_a: "Wezen".into(), star_b: "Aludra".into() },
        ConstellationLine { constellation: "Canis Minor".into(), star_a: "Procyon".into(), star_b: "Gomeisa".into() },

        // Cygnus & Summer Triangle & Lyra & Aquila
        ConstellationLine { constellation: "Lyra".into(), star_a: "Vega".into(), star_b: "Sheliak".into() },
        ConstellationLine { constellation: "Lyra".into(), star_a: "Sheliak".into(), star_b: "Sulafat".into() },
        ConstellationLine { constellation: "Lyra".into(), star_a: "Sulafat".into(), star_b: "Vega".into() },
        ConstellationLine { constellation: "Cygnus".into(), star_a: "Deneb".into(), star_b: "Sadr".into() },
        ConstellationLine { constellation: "Cygnus".into(), star_a: "Sadr".into(), star_b: "Albireo".into() },
        ConstellationLine { constellation: "Cygnus".into(), star_a: "Delta Cyg".into(), star_b: "Sadr".into() },
        ConstellationLine { constellation: "Cygnus".into(), star_a: "Sadr".into(), star_b: "Gienah Cyg".into() },
        ConstellationLine { constellation: "Aquila".into(), star_a: "Tarazed".into(), star_b: "Altair".into() },
        ConstellationLine { constellation: "Aquila".into(), star_a: "Altair".into(), star_b: "Alshain".into() },

        // Taurus & Auriga & Boötes
        ConstellationLine { constellation: "Taurus".into(), star_a: "Aldebaran".into(), star_b: "Elnath".into() },
        ConstellationLine { constellation: "Taurus".into(), star_a: "Aldebaran".into(), star_b: "Alcyone".into() },
        ConstellationLine { constellation: "Auriga".into(), star_a: "Capella".into(), star_b: "Menkalinan".into() },
        ConstellationLine { constellation: "Auriga".into(), star_a: "Menkalinan".into(), star_b: "Elnath".into() },
        ConstellationLine { constellation: "Bootes".into(), star_a: "Arcturus".into(), star_b: "Muphrid".into() },
        ConstellationLine { constellation: "Bootes".into(), star_a: "Arcturus".into(), star_b: "Izar".into() },

        // Scorpio & Sagittarius
        ConstellationLine { constellation: "Scorpio".into(), star_a: "Graffias".into(), star_b: "Dschubba".into() },
        ConstellationLine { constellation: "Scorpio".into(), star_a: "Dschubba".into(), star_b: "Antares".into() },
        ConstellationLine { constellation: "Scorpio".into(), star_a: "Antares".into(), star_b: "Sargas".into() },
        ConstellationLine { constellation: "Scorpio".into(), star_a: "Sargas".into(), star_b: "Shaula".into() },
        ConstellationLine { constellation: "Scorpio".into(), star_a: "Shaula".into(), star_b: "Lesath".into() },
        ConstellationLine { constellation: "Sagittarius".into(), star_a: "Alnasl".into(), star_b: "Kaus Media".into() },
        ConstellationLine { constellation: "Sagittarius".into(), star_a: "Kaus Media".into(), star_b: "Kaus Australis".into() },
        ConstellationLine { constellation: "Sagittarius".into(), star_a: "Kaus Australis".into(), star_b: "Ascella".into() },
        ConstellationLine { constellation: "Sagittarius".into(), star_a: "Ascella".into(), star_b: "Nunki".into() },
        ConstellationLine { constellation: "Sagittarius".into(), star_a: "Nunki".into(), star_b: "Kaus Borealis".into() },
        ConstellationLine { constellation: "Sagittarius".into(), star_a: "Kaus Borealis".into(), star_b: "Kaus Media".into() },

        // Leo & Virgo & Libra & Gemini
        ConstellationLine { constellation: "Leo".into(), star_a: "Regulus".into(), star_b: "Algieba".into() },
        ConstellationLine { constellation: "Leo".into(), star_a: "Algieba".into(), star_b: "Zosma".into() },
        ConstellationLine { constellation: "Leo".into(), star_a: "Zosma".into(), star_b: "Denebola".into() },
        ConstellationLine { constellation: "Virgo".into(), star_a: "Spica".into(), star_b: "Porrima".into() },
        ConstellationLine { constellation: "Virgo".into(), star_a: "Porrima".into(), star_b: "Vindemiatrix".into() },
        ConstellationLine { constellation: "Libra".into(), star_a: "Zubenelgenubi".into(), star_b: "Zubeneschamali".into() },
        ConstellationLine { constellation: "Gemini".into(), star_a: "Castor".into(), star_b: "Pollux".into() },
        ConstellationLine { constellation: "Gemini".into(), star_a: "Pollux".into(), star_b: "Alhena".into() },

        // Pegasus & Andromeda
        ConstellationLine { constellation: "Pegasus".into(), star_a: "Markab".into(), star_b: "Scheat".into() },
        ConstellationLine { constellation: "Pegasus".into(), star_a: "Scheat".into(), star_b: "Alpheratz".into() },
        ConstellationLine { constellation: "Pegasus".into(), star_a: "Alpheratz".into(), star_b: "Algenib".into() },
        ConstellationLine { constellation: "Pegasus".into(), star_a: "Algenib".into(), star_b: "Markab".into() },
        ConstellationLine { constellation: "Pegasus".into(), star_a: "Markab".into(), star_b: "Enif".into() },
        ConstellationLine { constellation: "Andromeda".into(), star_a: "Alpheratz".into(), star_b: "Mirach".into() },
        ConstellationLine { constellation: "Andromeda".into(), star_a: "Mirach".into(), star_b: "Almach".into() },
        ConstellationLine { constellation: "Aries".into(), star_a: "Hamal".into(), star_b: "Sheratan".into() },
        ConstellationLine { constellation: "Aquarius".into(), star_a: "Sadalsuud".into(), star_b: "Sadalmelik".into() },
        ConstellationLine { constellation: "Aquarius".into(), star_a: "Sadalmelik".into(), star_b: "Skat".into() },
        ConstellationLine { constellation: "Carina".into(), star_a: "Canopus".into(), star_b: "Miaplacidus".into() },
        ConstellationLine { constellation: "Carina".into(), star_a: "Miaplacidus".into(), star_b: "Avior".into() },
        ConstellationLine { constellation: "Carina".into(), star_a: "Avior".into(), star_b: "Aspidiske".into() },
    ]
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

    SkyProjection {
        lat: lat_deg,
        lon: lon_deg,
        utc_time: format!("{:04}-{:02}-{:02} {:02}:{:02} UTC", year, month, day, ut_hour.floor() as u32, ((ut_hour.fract() * 60.0).round() as u32)),
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

