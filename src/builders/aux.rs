use std::io;

use anyhow::Result;

/// This trait defines methods which every builder representation
/// must implement.
pub trait Builder {
    fn create_tmp(&self) -> Result<(), io::Error>;
    fn generate_aux_data(&self) -> Result<()>;
    fn render_templates(&self) -> Result<()>;
    fn copy_data(&self) -> Result<()>;
    fn clean_up(&self);
}
