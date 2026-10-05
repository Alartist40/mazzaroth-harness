use axum::extract::Query;
use axum::response::Json;
use librarian_core::{project_sky, SkyProjection};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
pub struct SkyQueryParams {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub year: Option<i32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
    pub hour: Option<f64>,
    pub time: Option<String>,
    pub radius: Option<f64>,
}

fn parse_iso_datetime(s: &str) -> Option<(i32, u32, u32, f64)> {
    // Expected format: YYYY-MM-DDTHH:MM or YYYY-MM-DD HH:MM
    let s = s.trim();
    let parts: Vec<&str> = s.split(|c| c == 'T' || c == ' ').collect();
    if parts.is_empty() {
        return None;
    }
    let date_parts: Vec<&str> = parts[0].split('-').collect();
    if date_parts.len() != 3 {
        return None;
    }
    let y = date_parts[0].parse::<i32>().ok()?;
    let m = date_parts[1].parse::<u32>().ok()?;
    let d = date_parts[2].parse::<u32>().ok()?;

    let mut ut_hour = 0.0;
    if parts.len() > 1 {
        let time_str = parts[1].trim_end_matches('Z');
        let time_parts: Vec<&str> = time_str.split(':').collect();
        if !time_parts.is_empty() {
            let h = time_parts[0].parse::<f64>().ok().unwrap_or(0.0);
            let min = if time_parts.len() > 1 {
                time_parts[1].parse::<f64>().ok().unwrap_or(0.0)
            } else {
                0.0
            };
            let sec = if time_parts.len() > 2 {
                time_parts[2].parse::<f64>().ok().unwrap_or(0.0)
            } else {
                0.0
            };
            ut_hour = h + (min / 60.0) + (sec / 3600.0);
        }
    }
    Some((y, m, d, ut_hour))
}

fn current_utc_ymd_h() -> (i32, u32, u32, f64) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    // Convert epoch seconds to (year, month, day, hour)
    let days_since_epoch = (now / 86400) as i64;
    let secs_of_day = now % 86400;
    let ut_hour = secs_of_day as f64 / 3600.0;

    // Civil date from day number (Howard Hinnant algorithm)
    let z = days_since_epoch + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = (yoe as i64 + era * 400) as i32;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let final_y = if m <= 2 { y + 1 } else { y };

    (final_y, m, d, ut_hour)
}

pub async fn handle_sky_projection(
    Query(params): Query<SkyQueryParams>,
) -> Json<SkyProjection> {
    let lat = params
        .lat
        .filter(|v| v.is_finite())
        .unwrap_or(35.6762)
        .clamp(-90.0, 90.0);
    let lon = params
        .lon
        .filter(|v| v.is_finite())
        .map(|v| {
            if (-180.0..=180.0).contains(&v) {
                v
            } else {
                (v + 180.0).rem_euclid(360.0) - 180.0
            }
        })
        .unwrap_or(139.6503);
    let radius = params
        .radius
        .filter(|v| v.is_finite())
        .unwrap_or(300.0)
        .clamp(50.0, 2000.0);

    let (year, month, day, ut_hour) = if let Some(ref t) = params.time {
        parse_iso_datetime(t).unwrap_or_else(current_utc_ymd_h)
    } else if let (Some(y), Some(m), Some(d)) = (params.year, params.month, params.day) {
        let h = params
            .hour
            .filter(|v| v.is_finite())
            .unwrap_or(0.0)
            .rem_euclid(24.0);
        let m_clamped = m.clamp(1, 12);
        let d_clamped = d.clamp(1, 31);
        (y, m_clamped, d_clamped, h)
    } else {
        current_utc_ymd_h()
    };

    let proj = project_sky(
        lat,
        lon,
        year,
        month.clamp(1, 12),
        day.clamp(1, 31),
        ut_hour.clamp(0.0, 24.0),
        radius,
    );
    Json(proj)
}
