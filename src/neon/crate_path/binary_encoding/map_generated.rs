// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum mapOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct map<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for map<'a> {
    type Inner = map<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> map<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        map { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args mapArgs<'args>) -> flatbuffers::WIPOffset<map<'bldr>> {
      let mut builder = mapBuilder::new(_fbb);
      if let Some(x) = args.data_shape { builder.add_data_shape(x); }
      if let Some(x) = args.data { builder.add_data(x); }
      if let Some(x) = args.frames { builder.add_frames(x); }
      if let Some(x) = args.functions { builder.add_functions(x); }
      builder.finish()
    }

    pub const VT_FUNCTIONS: flatbuffers::VOffsetT = 4;
    pub const VT_FRAMES: flatbuffers::VOffsetT = 6;
    pub const VT_DATA: flatbuffers::VOffsetT = 8;
    pub const VT_DATA_SHAPE: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn functions(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<function_map<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<function_map>>>>(map::VT_FUNCTIONS, None)
  }
  #[inline]
  pub fn frames(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<frame_map<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<frame_map>>>>(map::VT_FRAMES, None)
  }
  #[inline]
  pub fn data(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(map::VT_DATA, None).map(|v| v.safe_slice())
  }
  #[inline]
  pub fn data_shape(&self) -> Option<variables_type<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<variables_type>>(map::VT_DATA_SHAPE, None)
  }
}

impl flatbuffers::Verifiable for map<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<function_map>>>>("functions", Self::VT_FUNCTIONS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<frame_map>>>>("frames", Self::VT_FRAMES, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("data", Self::VT_DATA, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<variables_type>>("data_shape", Self::VT_DATA_SHAPE, false)?
     .finish();
    Ok(())
  }
}
pub struct mapArgs<'a> {
    pub functions: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<function_map<'a>>>>>,
    pub frames: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<frame_map<'a>>>>>,
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub data_shape: Option<flatbuffers::WIPOffset<variables_type<'a>>>,
}
impl<'a> Default for mapArgs<'a> {
    #[inline]
    fn default() -> Self {
        mapArgs {
            functions: None,
            frames: None,
            data: None,
            data_shape: None,
        }
    }
}
pub struct mapBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> mapBuilder<'a, 'b> {
  #[inline]
  pub fn add_functions(&mut self, functions: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<function_map<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(map::VT_FUNCTIONS, functions);
  }
  #[inline]
  pub fn add_frames(&mut self, frames: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<frame_map<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(map::VT_FRAMES, frames);
  }
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(map::VT_DATA, data);
  }
  #[inline]
  pub fn add_data_shape(&mut self, data_shape: flatbuffers::WIPOffset<variables_type<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<variables_type>>(map::VT_DATA_SHAPE, data_shape);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> mapBuilder<'a, 'b> {
    let start = _fbb.start_table();
    mapBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<map<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for map<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("map");
      ds.field("functions", &self.functions());
      ds.field("frames", &self.frames());
      ds.field("data", &self.data());
      ds.field("data_shape", &self.data_shape());
      ds.finish()
  }
}
#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_map<'a>(buf: &'a [u8]) -> map<'a> {
  unsafe { flatbuffers::root_unchecked::<map<'a>>(buf) }
}

#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_map<'a>(buf: &'a [u8]) -> map<'a> {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<map<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `map`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_map_unchecked`.
pub fn root_as_map(buf: &[u8]) -> Result<map, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<map>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `map` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_map_unchecked`.
pub fn size_prefixed_root_as_map(buf: &[u8]) -> Result<map, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<map>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `map` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_map_unchecked`.
pub fn root_as_map_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<map<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<map<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `map` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_map_unchecked`.
pub fn size_prefixed_root_as_map_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<map<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<map<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a map and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `map`.
pub unsafe fn root_as_map_unchecked(buf: &[u8]) -> map {
  flatbuffers::root_unchecked::<map>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed map and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `map`.
pub unsafe fn size_prefixed_root_as_map_unchecked(buf: &[u8]) -> map {
  flatbuffers::size_prefixed_root_unchecked::<map>(buf)
}
#[inline]
pub fn finish_map_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<map<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_map_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<map<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
