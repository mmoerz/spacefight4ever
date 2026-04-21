use thiserror::Error;

/// unified asset loading errors
#[derive(Debug, Error)]
pub enum AssetLoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("RON deserialization error: {0}")]
    Ron(#[from] ron::error::SpannedError),

    #[error("Invalid atlas size in {origin}: rows={rows}, cols={cols}")]
    InvalidAtlasSize { origin: String, rows: u32, cols: u32 },

    #[error("Tile size invalid in {origin}: width={width}, height={height}")]
    InvalidTileSize { origin: String, width: u32, height: u32 },

    /// Mapping index out of bounds
    #[error("Invalid mapping index {index} at {position} in {origin}, max allowed is {max}")]
    InvalidMapping {
        origin: String,
        position: usize,
        index: usize,
        max: usize,
    },
}