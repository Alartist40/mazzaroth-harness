use anyhow::{bail, Result};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PmTilesHeader {
    pub magic: [u8; 7],
    pub version: u8,
    pub root_offset: u64,
    pub root_length: u64,
    pub json_metadata_offset: u64,
    pub json_metadata_length: u64,
    pub leaf_directory_offset: u64,
    pub leaf_directory_length: u64,
    pub tile_data_offset: u64,
    pub tile_data_length: u64,
    pub num_addressed_tiles: u64,
    pub num_tile_entries: u64,
    pub num_tile_contents: u64,
    pub min_zoom: u8,
    pub max_zoom: u8,
    pub min_lon_e7: i32,
    pub min_lat_e7: i32,
    pub max_lon_e7: i32,
    pub max_lat_e7: i32,
    pub center_zoom: u8,
    pub center_lon_e7: i32,
    pub center_lat_e7: i32,
}

impl PmTilesHeader {
    pub fn read_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let mut f = File::open(path)?;
        let mut buf = [0u8; 127];
        f.read_exact(&mut buf)?;

        if &buf[0..7] != b"PMTiles" {
            bail!("Invalid PMTiles header magic");
        }

        let version = buf[7];
        if version != 3 {
            bail!("Unsupported PMTiles version {}, only v3 is supported", version);
        }

        let root_offset = u64::from_le_bytes(buf[8..16].try_into()?);
        let root_length = u64::from_le_bytes(buf[16..24].try_into()?);
        let json_metadata_offset = u64::from_le_bytes(buf[24..32].try_into()?);
        let json_metadata_length = u64::from_le_bytes(buf[32..40].try_into()?);
        let leaf_directory_offset = u64::from_le_bytes(buf[40..48].try_into()?);
        let leaf_directory_length = u64::from_le_bytes(buf[48..56].try_into()?);
        let tile_data_offset = u64::from_le_bytes(buf[56..64].try_into()?);
        let tile_data_length = u64::from_le_bytes(buf[64..72].try_into()?);
        let num_addressed_tiles = u64::from_le_bytes(buf[72..80].try_into()?);
        let num_tile_entries = u64::from_le_bytes(buf[80..88].try_into()?);
        let num_tile_contents = u64::from_le_bytes(buf[88..96].try_into()?);
        let min_zoom = buf[100];
        let max_zoom = buf[101];
        let min_lon_e7 = i32::from_le_bytes(buf[102..106].try_into()?);
        let min_lat_e7 = i32::from_le_bytes(buf[106..110].try_into()?);
        let max_lon_e7 = i32::from_le_bytes(buf[110..114].try_into()?);
        let max_lat_e7 = i32::from_le_bytes(buf[114..118].try_into()?);
        let center_zoom = buf[118];
        let center_lon_e7 = i32::from_le_bytes(buf[119..123].try_into()?);
        let center_lat_e7 = i32::from_le_bytes(buf[123..127].try_into()?);

        Ok(Self {
            magic: b"PMTiles".to_owned(),
            version,
            root_offset,
            root_length,
            json_metadata_offset,
            json_metadata_length,
            leaf_directory_offset,
            leaf_directory_length,
            tile_data_offset,
            tile_data_length,
            num_addressed_tiles,
            num_tile_entries,
            num_tile_contents,
            min_zoom,
            max_zoom,
            min_lon_e7,
            min_lat_e7,
            max_lon_e7,
            max_lat_e7,
            center_zoom,
            center_lon_e7,
            center_lat_e7,
        })
    }

    pub fn read_range(path: impl AsRef<Path>, offset: u64, length: usize) -> Result<Vec<u8>> {
        let mut f = File::open(path)?;
        f.seek(SeekFrom::Start(offset))?;
        let mut buf = vec![0u8; length];
        f.read_exact(&mut buf)?;
        Ok(buf)
    }
}
