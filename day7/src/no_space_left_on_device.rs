use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub enum FileSystemError {
    PathNotFound,
    ImpossibleAction,
    FileAlreadyExists,
}

#[derive(Debug)]
pub struct FileSystem {
    root: Rc<RefCell<FileType>>,
    cwd: Rc<RefCell<FileType>>,
}

impl FileSystem {
    pub fn cd(&mut self, path: &str) -> Result<(), FileSystemError> {
        if path == "/" {
            self.cwd = self.root.clone();
        } else if path == ".." {
            let new_cwd = self
                .cwd
                .borrow()
                .get_parent()
                .ok_or(FileSystemError::PathNotFound)?;
            self.cwd = new_cwd.upgrade().ok_or(FileSystemError::PathNotFound)?;
        } else {
            let new_cwd = self.cwd.borrow().get_contents(path)?.clone();
            self.cwd = new_cwd;
        }

        Ok(())
    }

    pub fn mkdir(&mut self, name: &str) -> Result<(), FileSystemError> {
        let new_dir = FileType::Dir {
            parent: Some(Rc::downgrade(&self.cwd)),
            contents: HashMap::new(),
        };

        self.cwd.borrow_mut().add_file(name, new_dir)
    }

    pub fn touch(&mut self, name: &str, size: usize) -> Result<(), FileSystemError> {
        let new_file = FileType::File { size };

        self.cwd.borrow_mut().add_file(name, new_file)
    }

    pub fn get_total_size(&self) -> usize {
        self.root.borrow().get_total_size()
    }

    pub fn get_dir_size(&self) -> usize {
        self.cwd.borrow().get_total_size()
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        let root = Rc::new(RefCell::new(FileType::default()));

        Self {
            cwd: root.clone(),
            root,
        }
    }
}

#[derive(Debug)]
enum FileType {
    Dir {
        parent: Option<Weak<RefCell<FileType>>>,
        contents: HashMap<String, Rc<RefCell<FileType>>>,
    },
    File {
        size: usize,
    },
}

impl FileType {
    pub fn get_total_size(&self) -> usize {
        match self {
            Self::Dir { contents, .. } => contents
                .values()
                .fold(0, |acc, content| acc + content.borrow().get_total_size()),
            Self::File { size } => *size,
        }
    }

    pub fn get_parent(&self) -> Option<Weak<RefCell<FileType>>> {
        match self {
            Self::Dir { parent, .. } => parent.clone(),
            Self::File { .. } => None,
        }
    }

    pub fn get_contents(&self, path: &str) -> Result<&Rc<RefCell<FileType>>, FileSystemError> {
        if let FileType::Dir { contents, .. } = self {
            contents.get(path).ok_or(FileSystemError::PathNotFound)
        } else {
            Err(FileSystemError::ImpossibleAction)
        }
    }

    pub fn add_file(&mut self, name: &str, file: FileType) -> Result<(), FileSystemError> {
        if let FileType::Dir { contents, .. } = self {
            if contents.contains_key(name) {
                Err(FileSystemError::FileAlreadyExists)
            } else {
                contents.insert(String::from(name), Rc::new(RefCell::new(file)));

                Ok(())
            }
        } else {
            Err(FileSystemError::ImpossibleAction)
        }
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self::Dir {
            parent: None,
            contents: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _empty_file_system_should_have_size_0() {
        let empty_fs = FileSystem::default();
        assert_eq!(0, empty_fs.get_dir_size());
        assert_eq!(0, empty_fs.get_total_size());
    }

    #[test]
    fn _should_return_correct_size_with_only_root() -> Result<(), FileSystemError> {
        let mut fs = FileSystem::default();
        fs.touch("b.txt", 14_848_514)?;
        assert_eq!(fs.get_dir_size(), 14_848_514);

        fs.touch("c.dat", 8_504_156)?;
        assert_eq!(fs.get_dir_size(), 23_352_670);

        assert_eq!(fs.get_total_size(), 23_352_670);

        Ok(())
    }

    #[test]
    fn _should_return_correct_size_with_nested_dirs() -> Result<(), FileSystemError> {
        let mut fs = FileSystem::default();
        fs.mkdir("a")?;
        fs.touch("b.txt", 14_848_514)?;
        fs.touch("c.dat", 8_504_156)?;
        fs.mkdir("d")?;

        fs.cd("a")?;
        fs.mkdir("e")?;
        fs.touch("f", 29_116)?;
        fs.touch("g", 2_557)?;
        fs.touch("h.lst", 62_596)?;

        fs.cd("e")?;
        fs.touch("i", 584)?;
        assert_eq!(fs.get_dir_size(), 584);

        fs.cd("..")?;
        assert_eq!(fs.get_dir_size(), 94_853);

        fs.cd("..")?;
        fs.cd("d")?;
        fs.touch("j", 4_060_174)?;
        fs.touch("d.log", 8_033_020)?;
        fs.touch("d.ext", 5_626_152)?;
        fs.touch("k", 7_214_296)?;
        assert_eq!(fs.get_dir_size(), 24_933_642);

        assert_eq!(fs.get_total_size(), 48_381_165);

        Ok(())
    }
}
