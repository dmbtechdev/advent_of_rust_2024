use std::{
    fs::{File,OpenOptions,remove_file},
    io::{Write, Read}, 
    path::PathBuf, 
    env::temp_dir,
    time::{SystemTime, UNIX_EPOCH}};

pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

// delete itself when out of scope.
impl Drop for TempFile {
    fn drop(&mut self) {
        // Your code here...
        // check if the file exists
        if self.file_path.exists() {
                remove_file(&self.file_path).unwrap();
            }
        
        // let _ = remove_file(&self.file_path);
    }
}

impl TempFile {
        
    pub fn new() -> Result<Self, std::io::Error> {
        // Your code here...
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        let rand_text = format!("{}", nanos as u32);
        let file_path = temp_dir().join(rand_text);
        Ok(Self {
            file_path: file_path.clone(),
            file: File::create(&file_path)?,
        })
    }

    pub fn write(&self, data: &[u8]) -> Result<(), std::io::Error> {
        // Your code here...
        OpenOptions::new().write(true).open(&self.file_path)?.write_all(data)?;
        Ok(())
    }

    pub fn read_to_string(&self) -> Result<String, std::io::Error> {
        // Your code here...
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }

}

