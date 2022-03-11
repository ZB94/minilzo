use std::error::Error;
use std::ffi::c_void;
use std::os::raw::c_uchar;
use std::ptr::null_mut;

const LZO1X_1_MEM_COMPRESS: usize = 16384 * std::mem::size_of::<usize>();

pub fn compress(input: &[u8]) -> Result<Vec<u8>, LzoError> {
    let mut output = vec![0u8; output_buffer_size(input.len())];
    let mut wrkmem = [0u8; LZO1X_1_MEM_COMPRESS];

    let mut size = output.len();
    let error = unsafe { lzo1x_1_compress(input.as_ptr(), input.len(), output.as_mut_ptr(), &mut size, wrkmem.as_mut_ptr() as *mut c_void) };
    if LzoError::Ok == error {
        output.resize(size, 0);
        Ok(output)
    } else {
        Err(error)
    }
}

pub fn decompress(buffer_len: usize, data: &[u8]) -> Result<Vec<u8>, LzoError> {
    let mut output = vec![0u8; buffer_len];
    let mut output_len = buffer_len;
    let error = unsafe { lzo1x_decompress_safe(data.as_ptr(), data.len(), output.as_mut_ptr(), &mut output_len, null_mut()) };
    if LzoError::Ok == error {
        output.resize(output_len, 0);
        Ok(output)
    } else {
        Err(error)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(i32)]
pub enum LzoError {
    Ok = 0,
    Error = -1,
    /// lzo_alloc_func_t failure
    OutOfMemory = -2,
    /// not used right now
    NotCompressible = -3,
    InputOverrun = -4,
    OutputOverrun = -5,
    LookbehindOverrun = -6,
    EofNotFound = -7,
    InputNotConsumed = -8,
    /// not used right now
    NotYetImplemented = -9,
    InvalidArgument = -10,
    /// pointer argument is not properly aligned
    InvalidAlignment = -11,
    OutputNotConsumed = -12,
    InternalError = -99,
}

impl std::fmt::Display for LzoError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            LzoError::Ok => { "ok" }
            LzoError::Error => { "error" }
            LzoError::OutOfMemory => { "out of memory" }
            LzoError::NotCompressible => { "not compressible" }
            LzoError::InputOverrun => { "input overrun" }
            LzoError::OutputOverrun => { "output overrun" }
            LzoError::LookbehindOverrun => { "lookbehind overrun" }
            LzoError::EofNotFound => { "EOF not found" }
            LzoError::InputNotConsumed => { "input not consumed" }
            LzoError::NotYetImplemented => { "not yet implemented" }
            LzoError::InvalidArgument => { "invalid argument" }
            LzoError::InvalidAlignment => { "invalid alignment" }
            LzoError::OutputNotConsumed => { "output not consumed" }
            LzoError::InternalError => { "internal error" }
        };
        fmt.write_str(s)
    }
}

impl Error for LzoError {}

#[inline]
const fn output_buffer_size(input_size: usize) -> usize {
    input_size + (input_size / 16) + 64 + 3
}

extern "C" {
    fn lzo1x_1_compress(
        src: *const c_uchar, src_len: usize,
        dst: *mut c_uchar, dst_len: *mut usize,
        wrkmem: *mut c_void,
    ) -> LzoError;

    fn lzo1x_decompress_safe(
        src: *const c_uchar, src_len: usize,
        dst: *mut c_uchar, dst_len: *mut usize,
        wrkmem: *mut c_void,
    ) -> LzoError;
}


#[cfg(test)]
mod tests {
    use std::ffi::c_void;
    use std::ptr::null_mut;

    use crate::{lzo1x_1_compress, LZO1X_1_MEM_COMPRESS, lzo1x_decompress_safe, LzoError, output_buffer_size, compress};

    #[test]
    pub fn test() {
        let input = b"test123456789".repeat(100);
        let mut output = vec![0u8; output_buffer_size(input.len())];
        let mut wrkmem = [0u8; LZO1X_1_MEM_COMPRESS];

        let mut size = output.len();
        let result = unsafe { lzo1x_1_compress(input.as_ptr(), input.len(), output.as_mut_ptr(), &mut size, wrkmem.as_mut_ptr() as *mut c_void) };
        assert_eq!(result, LzoError::Ok);

        let mut data = vec![0u8; input.len() + 1];
        let mut data_len = data.len();
        let result = unsafe { lzo1x_decompress_safe(output[0..size].as_ptr(), size, data.as_mut_ptr(), &mut data_len, null_mut()) };
        assert_eq!(result, LzoError::Ok);

        assert_eq!(data[..data_len], input);
    }
}
