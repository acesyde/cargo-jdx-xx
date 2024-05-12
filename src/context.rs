use std::path::{Path, PathBuf};
use std::sync::Mutex;

static LOAD_ROOT: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn get_load_root() -> PathBuf {
    LOAD_ROOT.lock().unwrap().clone().unwrap_or_default()
}

pub fn set_load_root<P: Into<PathBuf>>(root: P) {
    *LOAD_ROOT.lock().unwrap() = Some(root.into());
}

pub fn prepend_load_root<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    match path.is_relative() {
        true => get_load_root().join(path),
        false => path.to_path_buf(),
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use test_log::test;

    use super::*;

    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_set_load_root() {
        let _t = TEST_MUTEX.lock().unwrap();
        set_load_root(PathBuf::from("/foo/bar"));
        assert_eq!(get_load_root(), PathBuf::from("/foo/bar"));
    }

    #[test]
    fn test_prepend_load_root() {
        let _t = TEST_MUTEX.lock().unwrap();
        set_load_root(PathBuf::from("/foo/bar"));
        assert_eq!(
            prepend_load_root(Path::new("baz")),
            PathBuf::from("/foo/bar/baz")
        );
        assert_eq!(prepend_load_root(Path::new("/baz")), PathBuf::from("/baz"));
    }
}
