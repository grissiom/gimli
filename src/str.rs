use endianity::{Endianity, EndianBuf};
use parser::{parse_null_terminated_string, Error, Result};
use std::ffi;
use std::marker::PhantomData;
use Section;

/// An offset into the `.debug_str` section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DebugStrOffset(pub usize);

/// The `DebugStr` struct represents the DWARF strings
/// found in the `.debug_str` section.
#[derive(Debug, Clone, Copy)]
pub struct DebugStr<'input, Endian>
    where Endian: Endianity
{
    debug_str_section: EndianBuf<'input, Endian>,
}

impl<'input, Endian> DebugStr<'input, Endian>
    where Endian: Endianity
{
    /// Construct a new `DebugStr` instance from the data in the `.debug_str`
    /// section.
    ///
    /// It is the caller's responsibility to read the `.debug_str` section and
    /// present it as a `&[u8]` slice. That means using some ELF loader on
    /// Linux, a Mach-O loader on OSX, etc.
    ///
    /// ```
    /// use gimli::{DebugStr, LittleEndian};
    ///
    /// # let buf = [0x00, 0x01, 0x02, 0x03];
    /// # let read_debug_str_section_somehow = || &buf;
    /// let debug_str = DebugStr::<LittleEndian>::new(read_debug_str_section_somehow());
    /// ```
    pub fn new(debug_str_section: &'input [u8]) -> DebugStr<'input, Endian> {
        DebugStr { debug_str_section: EndianBuf(debug_str_section, PhantomData) }
    }

    /// Lookup a string from the `.debug_str` section by DebugStrOffset.
    ///
    /// ```
    /// use gimli::{DebugStr, DebugStrOffset, LittleEndian};
    ///
    /// # let buf = [0x01, 0x02, 0x00];
    /// # let offset = DebugStrOffset(0);
    /// # let read_debug_str_section_somehow = || &buf;
    /// # let debug_str_offset_somehow = || offset;
    /// let debug_str = DebugStr::<LittleEndian>::new(read_debug_str_section_somehow());
    /// println!("Found string {:?}", debug_str.get_str(debug_str_offset_somehow()));
    /// ```
    pub fn get_str(&self, offset: DebugStrOffset) -> Result<&'input ffi::CStr> {
        if self.debug_str_section.len() < offset.0 {
            return Err(Error::UnexpectedEof);
        }
        let buf = self.debug_str_section.range_from(offset.0..);
        let result = parse_null_terminated_string(buf.0);
        result.map(|(_, cstr)| cstr)
    }
}

impl<'input, Endian> Section<'input> for DebugStr<'input, Endian>
    where Endian: Endianity
{
    fn section_name() -> &'static str {
        ".debug_str"
    }
}

impl<'input, Endian> From<&'input [u8]> for DebugStr<'input, Endian>
    where Endian: Endianity
{
    fn from(v: &'input [u8]) -> Self {
        Self::new(v)
    }
}
