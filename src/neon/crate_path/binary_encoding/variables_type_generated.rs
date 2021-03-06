// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum variables_typeOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct variables_type<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for variables_type<'a> {
    type Inner = variables_type<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> variables_type<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        variables_type { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args variables_typeArgs<'args>) -> flatbuffers::WIPOffset<variables_type<'bldr>> {
      let mut builder = variables_typeBuilder::new(_fbb);
      if let Some(x) = args.indexes { builder.add_indexes(x); }
      builder.finish()
    }

    pub const VT_INDEXES: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn indexes(&self) -> Option<flatbuffers::Vector<'a, u64>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u64>>>(variables_type::VT_INDEXES, None)
  }
}

impl flatbuffers::Verifiable for variables_type<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u64>>>("indexes", Self::VT_INDEXES, false)?
     .finish();
    Ok(())
  }
}
pub struct variables_typeArgs<'a> {
    pub indexes: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u64>>>,
}
impl<'a> Default for variables_typeArgs<'a> {
    #[inline]
    fn default() -> Self {
        variables_typeArgs {
            indexes: None,
        }
    }
}
pub struct variables_typeBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> variables_typeBuilder<'a, 'b> {
  #[inline]
  pub fn add_indexes(&mut self, indexes: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u64>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(variables_type::VT_INDEXES, indexes);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> variables_typeBuilder<'a, 'b> {
    let start = _fbb.start_table();
    variables_typeBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<variables_type<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for variables_type<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("variables_type");
      ds.field("indexes", &self.indexes());
      ds.finish()
  }
}
