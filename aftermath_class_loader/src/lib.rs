use {
    glob::glob,
    std::{
        collections::HashMap,
        fs::File,
        io::{BufReader, Error as IOError},
    },
    thiserror::Error,
    zip::{result::ZipError, ZipArchive},
};

#[derive(Error, Debug)]
enum Error
{
    #[error("Pattern error -> ")]
    PatternError(#[from] glob::PatternError),
    #[error("Glob error -> ")]
    GlobError(#[from] glob::GlobError),
    #[error("I/O Error -> ")]
    IOError(#[from] IOError),
    #[error("Zip Error [Help -> If you're seeing this, either the bootstrap java modules for your JDK installation are malformed, or Aftermath is broken.] -> ")]
    ZipError(#[from] ZipError),
}

#[derive(Default)]
pub struct ClassLoader
{
    pub classes: HashMap<String, usize>,
    pub archives: Vec<String>,
}

impl ClassLoader
{
    fn new() -> ClassLoader
    {
        Self::default()
    }

    fn add_dir(&mut self, path: &str) -> Result<(), Error>
    {
        for i in glob(&format!("{}/*.jmod", path))?.flatten() {
            self.archives.push(i.to_string_lossy().to_string());
            let current_archive = self.archives.len();

            for z in ZipArchive::new(BufReader::new(File::open(i)?))?.file_names() {
                if z.ends_with(".class") {
                    self.classes
                        .insert(z[8..z.len() - 6].to_string(), current_archive);
                }
            }
        }

        Ok(())
    }

    fn find_class(&self, id: &str) -> Option<&str>
    {
        Some(&self.archives[*self.classes.get(id)?])
    }
}
