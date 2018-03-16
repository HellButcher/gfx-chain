mod access;
mod buffer;
mod image;
mod layout;
mod usage;

use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Range;
use hal::buffer::{Access as BufferAccess, Usage as BufferUsage};
use hal::image::{Access as ImageAccess, ImageLayout, SubresourceRange, Usage as ImageUsage};
use hal::pso::PipelineStage;

pub use self::access::Access;
pub use self::buffer::BufferLayout;
pub use self::layout::Layout;
pub use self::usage::Usage;

/// Defines resource type.
/// Should be implemented for buffers and images.
pub trait Resource: Copy + Debug + Eq + Ord + Hash {
    /// Access type of the resource.
    type Access: Access;

    /// Layout type of the resource.
    type Layout: Layout;

    /// Usage type of the resource.
    type Usage: Usage;

    /// Sub-resource range.
    type Range: Clone;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Buffer {}
impl Resource for Buffer {
    type Access = BufferAccess;
    type Layout = buffer::BufferLayout;
    type Usage = BufferUsage;
    type Range = Range<u64>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Image {}
impl Resource for Image {
    type Access = ImageAccess;
    type Layout = ImageLayout;
    type Usage = ImageUsage;
    type Range = SubresourceRange;
}

/// Resource id
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id<R>(usize, PhantomData<R>);

#[derive(Clone, Copy, Debug)]
pub struct State<R: Resource> {
    pub access: R::Access,
    pub layout: R::Layout,
    pub stages: PipelineStage,
}

impl<R> State<R>
where
    R: Resource,
{
    /// Merge states.
    /// Panic if layouts are incompatible.
    pub fn merge(&self, rhs: Self) -> Self {
        State {
            access: self.access | rhs.access,
            layout: self.layout.merge(rhs.layout).unwrap(),
            stages: self.stages | rhs.stages,
        }
    }

    /// Check if access is exclusive.
    pub fn exclusive(&self) -> bool {
        self.access.is_write()
    }

    /// Check if states are compatible.
    /// This requires layouts to be compatible and non-exclusive access.
    pub fn compatible(&self, rhs: Self) -> bool {
        !self.exclusive() && !rhs.exclusive() && self.layout.merge(rhs.layout).is_some()
    }
}
