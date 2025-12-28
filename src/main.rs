mod audio;
mod player;

use anyhow::Result;

fn main() -> Result<()> {
    player::play_airhorn()?;
    Ok(())
}
