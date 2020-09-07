mod try_catch_memcpy;

use crate::windows::try_catch_memcpy::safe_try_catch_memcpy;
use crate::FileViewAccessError;
use std::convert::TryInto;
use std::fs::File;
use std::ops::Range;
use std::os::windows::io::AsRawHandle;
use std::{io, mem, ptr};
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{DWORD, LPCVOID};
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{CreateFileMappingW, MapViewOfFile, UnmapViewOfFile, FILE_MAP_READ};
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
use winapi::um::sysinfoapi::GetSystemInfo;
use winapi::um::winnt::{HANDLE, PAGE_READONLY};

pub struct FileMapping {
    file_mapping_handle: HANDLE,
}

pub struct FileView {
    ptr: *const u8,
}

pub fn create_read_only_file_mapping(file: &File) -> io::Result<FileMapping> {
    let handle = unsafe {
        CreateFileMappingW(
            file.as_raw_handle() as HANDLE,
            ptr::null_mut() as LPSECURITY_ATTRIBUTES,
            PAGE_READONLY,
            0, // "If this parameter and dwMaximumSizeHigh are 0 (zero), the maximum size of the file mapping object is equal to the current size of the file that hFile identifies."
            0,
            ptr::null(),
        )
    };

    if handle.is_null() {
        return Err(io::Error::last_os_error());
    }
    Ok(FileMapping {
        file_mapping_handle: handle,
    })
}

fn allocation_granularity() -> usize {
    unsafe {
        let mut info = mem::zeroed();
        GetSystemInfo(&mut info);
        info.dwAllocationGranularity as usize
    }
}

impl FileMapping {
    pub fn map_view_of_file(&self, offset: u64, len: u64) -> io::Result<FileView> {
        let alignment = offset % allocation_granularity() as u64;
        let aligned_offset = offset - alignment as u64;
        let aligned_len = len + alignment;

        let ptr = unsafe {
            MapViewOfFile(
                self.file_mapping_handle,
                FILE_MAP_READ,
                (aligned_offset >> 16 >> 16) as DWORD,
                (aligned_offset & 0xffff_ffff) as DWORD,
                aligned_len as SIZE_T,
            )
        };

        if ptr.is_null() {
            return Err(io::Error::last_os_error());
        }

        Ok(FileView {
            ptr: ptr as *const u8,
        })
    }
}

impl Drop for FileMapping {
    fn drop(&mut self) {
        let success = unsafe { CloseHandle(self.file_mapping_handle) };
        if success == 0 {
            panic!("CloseHandle failed: {}", io::Error::last_os_error())
        }
    }
}

impl FileView {
    pub fn try_read_into(
        &self,
        offset: Range<usize>,
        dst: &mut [u8],
    ) -> Result<(), FileViewAccessError> {
        match safe_try_catch_memcpy(dst, unsafe {
            std::slice::from_raw_parts(
                self.ptr.offset(offset.start.try_into().unwrap()),
                offset.end - offset.start,
            )
        }) {
            Ok(()) => Ok(()),
            Err(_) => Err(FileViewAccessError::MemCopyError),
        }
    }
}

impl Drop for FileView {
    fn drop(&mut self) {
        let alignment = self.ptr as usize % allocation_granularity();
        unsafe {
            let ptr = self.ptr.offset(-(alignment as isize));
            assert!(
                UnmapViewOfFile(ptr as LPCVOID) != 0,
                "UnmapViewOfFile failed: {}",
                io::Error::last_os_error()
            );
        }
    }
}
