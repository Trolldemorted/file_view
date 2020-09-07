#[derive(Debug)]
pub enum WindowsMemCopyError {
    DestinationTooSmall,
    ExceptionInPage,
}

pub fn safe_try_catch_memcpy(dst: &mut [u8], src: &[u8]) -> Result<(), WindowsMemCopyError> {
    if src.len() > dst.len() {
        return Err(WindowsMemCopyError::DestinationTooSmall);
    }
    if unsafe { try_catch_memcpy(dst.as_mut_ptr(), src.as_ptr(), src.len()) } {
        Ok(())
    } else {
        Err(WindowsMemCopyError::ExceptionInPage)
    }
}

extern "C" {
    pub fn try_catch_memcpy(dst: *mut u8, src: *const u8, len: usize) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_memcopy() {
        let a = [0; 12];
        let mut b = [1; 12];
        safe_try_catch_memcpy(&mut b[3..9], &a[0..6]).unwrap();
        assert_eq!(b, [1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1])
    }
}
