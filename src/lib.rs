use std::fs::File;
use std::ops::Range;

#[cfg(windows)]
mod windows;

#[derive(Debug)]
pub enum FileViewAccessError {
    MemCopyError,
}

pub struct ReadOnlyFileView {
    pub len: u64,
    #[cfg(windows)]
    file_view: windows::FileView,
}

impl ReadOnlyFileView {
    #[inline]
    #[cfg(windows)]
    pub fn try_read_into(
        &self,
        offset: Range<usize>,
        dst: &mut [u8],
    ) -> Result<(), FileViewAccessError> {
        self.file_view.try_read_into(offset, dst)
    }
}

#[inline]
#[cfg(windows)]
pub fn open_view_readonly(file: &File) -> Result<ReadOnlyFileView, std::io::Error> {
    let file_mapping = windows::create_read_only_file_mapping(file)?;
    let len = file.metadata()?.len();
    let file_view = file_mapping.map_view_of_file(0, len)?;
    Ok(ReadOnlyFileView { len, file_view })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_build_rs() {
        let mut file_view_content = vec![0; 50];
        let mut direct_read_content = vec![0; 50];
        let mut file = File::open("build.rs").unwrap();

        // Read directly
        file.read_exact(&mut direct_read_content).unwrap();

        // Read with file view
        let view = open_view_readonly(&file).unwrap();
        view.try_read_into(0..50, &mut file_view_content[0..50])
            .unwrap();

        assert_eq!(direct_read_content, file_view_content)
    }
}
