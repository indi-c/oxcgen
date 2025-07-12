use std::fs;

#[derive(Clone)]
pub struct Language<'a> {
    pub name: &'a str,
    pub description: &'a str,
} 

pub trait MakeProject {
    fn make_project(&self, project_name: &str) -> Result<(), String>;
    fn create_dirs(&self);
    fn copy_files(&self);
    fn setup_files(&self);
    fn initialize_git(&self);
}

impl MakeProject for Language<'_> {
    fn make_project(&self, project_name: &str) -> Result<(), String> {
        self.create_dirs();
        self.copy_files();
        self.setup_files();
        self.initialize_git();
        Ok(())
    }
    fn create_dirs(&self) {
        // Implementation for creating directories
    }

    fn copy_files(&self) {
        // Implementation for copying files
    }

    fn setup_files(&self) {
        // Implementation for setting up files
    }

    fn initialize_git(&self) {
        // Implementation for initializing git repository
    }
}