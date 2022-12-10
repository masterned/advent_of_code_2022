use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
    str::FromStr,
};

#[derive(Debug)]
pub enum FileSystemError {
    PathNotFound,
    ImpossibleAction,
    FileAlreadyExists,
}

#[derive(Debug)]
pub enum FileSystemMessageError {
    ParseError,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileSystemMessage {
    Cd(String),
    Mkdir(String),
    Touch(String, usize),
}

impl FromStr for FileSystemMessage {
    type Err = FileSystemMessageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        match tokens.next() {
            Some("$") => match tokens.next() {
                Some("cd") => {
                    let path = tokens.next().ok_or(Self::Err::ParseError)?.to_string();
                    Ok(Self::Cd(path))
                }
                _ => Err(Self::Err::ParseError),
            },
            Some("dir") => {
                let path = tokens.next().ok_or(Self::Err::ParseError)?;
                Ok(Self::Mkdir(String::from(path)))
            }
            Some(size_str) => {
                let size = size_str.parse().map_err(|_| Self::Err::ParseError)?;
                let name = tokens.next().ok_or(Self::Err::ParseError)?.to_string();
                Ok(Self::Touch(name, size))
            }
            None => Err(Self::Err::ParseError),
        }
    }
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
            let new_cwd = self.cwd.borrow().get_file(path)?.clone();
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

    pub fn exec(&mut self, msg: FileSystemMessage) -> Result<(), FileSystemError> {
        match msg {
            FileSystemMessage::Cd(path) => self.cd(&path),
            FileSystemMessage::Mkdir(name) => self.mkdir(&name),
            FileSystemMessage::Touch(name, size) => self.touch(&name, size),
        }
    }

    pub fn total_dir_size_less_than_or_equal_to_100_000(&self) -> usize {
        self.root
            .borrow()
            .total_dir_size_less_than_or_equal_to_100_000()
    }

    pub fn get_minimum_size_with_at_least(&self, size: usize) -> usize {
        self.root.borrow().get_minimum_size_with_at_least(size)
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

    pub fn get_file(&self, path: &str) -> Result<&Rc<RefCell<FileType>>, FileSystemError> {
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

    pub fn get_dirs(&self) -> Vec<&Rc<RefCell<FileType>>> {
        if let FileType::Dir { contents, .. } = self {
            contents
                .values()
                .filter(|file| file.borrow().is_dir())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Dir { .. })
    }

    pub fn total_dir_size_less_than_or_equal_to_100_000(&self) -> usize {
        let mut count = 0;

        let dirs = self.get_dirs();

        for dir in dirs {
            if dir.borrow().get_total_size() <= 100_000 {
                count += dir.borrow().get_total_size();
            }

            count += dir.borrow().total_dir_size_less_than_or_equal_to_100_000();
        }

        count
    }

    pub fn get_minimum_size_with_at_least(&self, size: usize) -> usize {
        let mut minimum_size = usize::MAX;

        let dirs = self.get_dirs();

        for dir in dirs {
            let dir_size = dir.borrow().get_total_size();
            if dir_size >= size && dir_size < minimum_size {
                minimum_size = dir_size;
            }

            let minimum_contained_size = dir.borrow().get_minimum_size_with_at_least(size);
            if minimum_contained_size < minimum_size {
                minimum_size = minimum_contained_size;
            }
        }

        minimum_size
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

    mod file_system_message_tests {
        use super::*;

        #[test]
        fn _test_file_system_message_parse() -> Result<(), FileSystemMessageError> {
            let msg: FileSystemMessage = "$ cd /".parse()?;
            assert_eq!(msg, FileSystemMessage::Cd(String::from("/")));

            let msg: FileSystemMessage = "dir dcvzbqf".parse()?;
            assert_eq!(msg, FileSystemMessage::Mkdir(String::from("dcvzbqf")));

            let msg: FileSystemMessage = "23804 gsdpmrq.bsz".parse()?;
            assert_eq!(
                msg,
                FileSystemMessage::Touch(String::from("gsdpmrq.bsz"), 23804)
            );

            Ok(())
        }
    }

    mod file_system_tests {
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
}
