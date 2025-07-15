use std::fs;
use crate::supported_languages;

#[derive(Clone)]
pub struct Language<'a> {
    pub name: &'a str,
} 

pub trait MakeProject {
    fn make_project(&self, project_name: &str) -> Result<(), String>;
    fn create_dirs(&self, lang_assets: supported_languages::Assets, project_name: &str);
    fn copy_files(&self, lang_assets: supported_languages::Assets, project_name: &str);
    fn setup_files(&self, lang_assets: supported_languages::Assets, project_name: &str);
    fn initialize_git(&self);
}

impl MakeProject for Language<'_> {
    fn make_project(&self, project_name: &str) -> Result<(), String> {
        let lang_assets = supported_languages::from_str(self.name)
            .ok_or_else(|| format!("[*] Language '{}' not supported", self.name))?;
        self.create_dirs(lang_assets, project_name);
        self.copy_files(lang_assets, project_name);
        self.setup_files(lang_assets, project_name);
        Ok(())
    }

    fn create_dirs(&self, lang_assets: supported_languages::Assets, project_name: &str) {
        lang_assets.iter().for_each(|file| {
            let file_path = format!("{}/{}", project_name, file);
            let folder_path = file_path.rsplit('/').skip(1).collect::<Vec<&str>>()
                .into_iter().rev().collect::<Vec<&str>>().join("/");
            fs::create_dir_all(folder_path).map_err(|e| format!("[*] Failed to create directory '{}': {}", file, e));
        });
    }

    fn copy_files(&self, lang_assets: supported_languages::Assets, project_name: &str) {
        lang_assets.iter().for_each(|file| {
            if let Some(content) = lang_assets.get(&file) {
                let file_path = format!("{}/{}", project_name, file);
                fs::write(file_path, content.data).map_err(|e| format!("[*] Failed to write file '{}': {}", file, e));
            }
        })
    }

    fn setup_files(&self, lang_assets: supported_languages::Assets, project_name: &str) {
        // go through each file and change project_name to the actual project name
        lang_assets.iter().for_each(|file| {
            let file_path = format!("{}/{}", project_name, file);
            // if the file exists, read it and replace the placeholder
            if fs::metadata(&file_path).is_ok() {
                fs::read_to_string(&file_path)
                    .and_then(|content| {
                        let updated_content = content.replace("project_name", project_name);
                        Ok(fs::write(&file_path, updated_content)
                            .map_err(|e| format!("[*] Failed to update file '{}': {}", file, e)))
                    });
            };
        })
    }

    fn initialize_git(&self) {
        // Implementation for initializing git repository
    }
}