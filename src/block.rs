#![allow(unused, reason =
"This module is designed to be complete in its functionality, even though parts are not used.")]

use std::iter::repeat;
use itertools::iterate;



#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Height(pub usize);

impl Height {
    #[inline(always)]
    pub fn height(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn size(self) -> usize {
        1 << self.height()
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=Height> {
        (0..self.height()).rev().map(Height)
    }
}

// region From implementations
impl From<Size> for Height {
    #[inline(always)]
    fn from(size: Size) -> Self {
        Height(size.height())
    }
}

impl From<End> for Height {
    #[inline(always)]
    fn from(end: End) -> Self {
        Height(end.height())
    }
}

impl From<Storage> for Height {
    #[inline(always)]
    fn from(storage: Storage) -> Self {
        Height(storage.height())
    }
}

impl From<StartAndHeight> for Height {
    #[inline(always)]
    fn from(start_and_height: StartAndHeight) -> Self {
        Height(start_and_height.height())
    }
}

impl From<StartAndSize> for Height {
    #[inline(always)]
    fn from(start_and_size: StartAndSize) -> Self {
        Height(start_and_size.height())
    }
}
// endregion

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Size(pub usize);

impl Size {
    #[inline(always)]
    pub fn height(self) -> usize {
        self.size().trailing_zeros() as usize
    }

    #[inline(always)]
    pub fn size(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=Size> {
        iterate(self.size(), |size| size >> 1).skip(1)
            .take_while(|size| *size > 0).map(Size)
    }
}

// region From implementations
impl From<Height> for Size {
    #[inline(always)]
    fn from(height: Height) -> Self {
        Size(height.size())
    }
}

impl From<End> for Size {
    #[inline(always)]
    fn from(end: End) -> Self {
        Size(end.size())
    }
}

impl From<Storage> for Size {
    #[inline(always)]
    fn from(storage: Storage) -> Self {
        Size(storage.size())
    }
}

impl From<StartAndHeight> for Size {
    #[inline(always)]
    fn from(start_and_height: StartAndHeight) -> Self {
        Size(start_and_height.size())
    }
}

impl From<StartAndSize> for Size {
    #[inline(always)]
    fn from(start_and_size: StartAndSize) -> Self {
        Size(start_and_size.size())
    }
}
// endregion

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Start(pub usize);

impl Start {
    #[inline(always)]
    pub fn start(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=Start> {
        repeat(self)
    }
}

// region From implementations
impl From<End> for Start {
    #[inline(always)]
    fn from(end: End) -> Self {
        Start(end.start())
    }
}

impl From<Storage> for Start {
    #[inline(always)]
    fn from(storage: Storage) -> Self {
        Start(storage.start())
    }
}

impl From<StartAndHeight> for Start {
    #[inline(always)]
    fn from(start_and_height: StartAndHeight) -> Self {
        Start(start_and_height.start())
    }
}

impl From<StartAndSize> for Start {
    #[inline(always)]
    fn from(start_and_size: StartAndSize) -> Self {
        Start(start_and_size.start())
    }
}
// endregion

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct End(pub usize);

impl End {
    #[inline(always)]
    pub fn height(self) -> usize {
        self.end().trailing_zeros() as usize
    }

    #[inline(always)]
    pub fn size(self) -> usize {
        1 << self.height()
    }

    #[inline(always)]
    pub fn start(self) -> usize {
        self.end() - self.size()
    }

    #[inline(always)]
    pub fn end(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn storage(self) -> usize {
        self.end() - 1
    }

    #[inline(always)]
    pub fn parent(self) -> End {
        End(self.end() + self.size())
    }

    #[inline(always)]
    pub fn parent_bounded(self, len: usize) -> Option<End> {
        Some(self.parent()).filter(|it| it.end() <= len)
    }

    #[inline(always)]
    pub fn parents(self) -> impl Iterator<Item=End> {
        iterate(self, |block| block.parent()).skip(1)
    }

    #[inline(always)]
    pub fn parents_bounded(self, len: usize) -> impl Iterator<Item=End> {
        self.parents().take_while(move |block| block.end() <= len)
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=End> {
        Size::from(self).children().map(move |size| End(self.end() - size.size()))
    }
}

// region From implementations
impl From<Storage> for End {
    #[inline(always)]
    fn from(storage: Storage) -> Self {
        End(storage.end())
    }
}

impl From<StartAndHeight> for End {
    #[inline(always)]
    fn from(start_and_height: StartAndHeight) -> Self {
        End(start_and_height.end())
    }
}

impl From<StartAndSize> for End {
    #[inline(always)]
    fn from(start_and_size: StartAndSize) -> Self {
        End(start_and_size.end())
    }
}
// endregion

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Storage(pub usize);

impl Storage {
    #[inline(always)]
    pub fn height(self) -> usize {
        self.storage().trailing_ones() as usize
    }

    #[inline(always)]
    pub fn size(self) -> usize {
        1 << self.height()
    }

    #[inline(always)]
    pub fn start(self) -> usize {
        self.end() - self.size()
    }

    #[inline(always)]
    pub fn end(self) -> usize {
        self.storage() + 1
    }

    #[inline(always)]
    pub fn storage(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn parent(self) -> Storage {
        Storage(self.storage() + self.size())
    }

    #[inline(always)]
    pub fn parent_bounded(self, len: usize) -> Option<Storage> {
        Some(self.parent()).filter(|it| it.storage() < len)
    }

    #[inline(always)]
    pub fn parents(self) -> impl Iterator<Item=Storage> {
        iterate(self, |block| block.parent()).skip(1)
    }

    #[inline(always)]
    pub fn parents_bounded(self, len: usize) -> impl Iterator<Item=Storage> {
        self.parents().take_while(move |block| block.storage() < len)
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=Storage> {
        Size::from(self).children().map(move |size| Storage(self.storage() - size.size()))
    }
}

// region From implementations
impl From<End> for Storage {
    #[inline(always)]
    fn from(end: End) -> Self {
        Storage(end.storage())
    }
}

impl From<StartAndHeight> for Storage {
    #[inline(always)]
    fn from(start_and_height: StartAndHeight) -> Self {
        Storage(start_and_height.storage())
    }
}

impl From<StartAndSize> for Storage {
    #[inline(always)]
    fn from(start_and_size: StartAndSize) -> Self {
        Storage(start_and_size.storage())
    }
}
// endregion

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct StartAndHeight(pub Start, pub Height);

impl StartAndHeight {
    #[inline(always)]
    pub fn height(self) -> usize {
        self.1.height()
    }

    #[inline(always)]
    pub fn size(self) -> usize {
        self.1.size()
    }

    #[inline(always)]
    pub fn start(self) -> usize {
        self.0.start()
    }

    #[inline(always)]
    pub fn end(self) -> usize {
        self.start() + self.size()
    }

    #[inline(always)]
    pub fn storage(self) -> usize {
        self.end() - 1
    }

    #[inline(always)]
    pub fn parent(self) -> StartAndHeight {
        let height = self.height() + self.start().trailing_ones() as usize;
        StartAndHeight(Start(self.start() >> height << height), Height(height + 1))
    }

    #[inline(always)]
    pub fn parent_bounded(self, len: usize) -> Option<StartAndHeight> {
        Some(self.parent()).filter(|it| it.end() <= len)
    }

    #[inline(always)]
    pub fn parents(self) -> impl Iterator<Item=StartAndHeight> {
        iterate(self, |block| block.parent()).skip(1)
    }

    #[inline(always)]
    pub fn parents_bounded(self, len: usize) -> impl Iterator<Item=StartAndHeight> {
        self.parents().take_while(move |block| block.end() <= len)
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=StartAndHeight> {
        Height::from(self).children().map(move |height| StartAndHeight(self.0, height))
    }
}

// region From implementations
impl From<End> for StartAndHeight {
    #[inline(always)]
    fn from(end: End) -> Self {
        StartAndHeight(Start(end.start()), Height(end.height()))
    }
}

impl From<Storage> for StartAndHeight {
    #[inline(always)]
    fn from(storage: Storage) -> Self {
        StartAndHeight(Start(storage.start()), Height(storage.height()))
    }
}

impl From<StartAndSize> for StartAndHeight {
    #[inline(always)]
    fn from(start_and_size: StartAndSize) -> Self {
        StartAndHeight(Start(start_and_size.start()), Height(start_and_size.height()))
    }
}
// endregion

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct StartAndSize(pub Start, pub Size);

impl StartAndSize {
    #[inline(always)]
    pub fn height(self) -> usize {
        self.1.height()
    }

    #[inline(always)]
    pub fn size(self) -> usize {
        self.1.size()
    }

    #[inline(always)]
    pub fn start(self) -> usize {
        self.0.start()
    }

    #[inline(always)]
    pub fn end(self) -> usize {
        self.start() + self.size()
    }

    #[inline(always)]
    pub fn storage(self) -> usize {
        self.end() - 1
    }

    #[inline(always)]
    pub fn parent(self) -> StartAndSize {
        End(self.end() + self.size()).into()
    }

    #[inline(always)]
    pub fn parent_bounded(self, len: usize) -> Option<StartAndSize> {
        Some(self.parent()).filter(|it| it.end() <= len)
    }

    #[inline(always)]
    pub fn parents(self) -> impl Iterator<Item=StartAndSize> {
        iterate(self, |block| block.parent()).skip(1)
    }

    #[inline(always)]
    pub fn parents_bounded(self, len: usize) -> impl Iterator<Item=StartAndSize> {
        self.parents().take_while(move |block| block.end() <= len)
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=StartAndSize> {
        Size::from(self).children().map(move |size| StartAndSize(self.0, size))
    }
}

// region From implementations
impl From<End> for StartAndSize {
    #[inline(always)]
    fn from(end: End) -> Self {
        StartAndSize(Start(end.start()), Size(end.size()))
    }
}

impl From<Storage> for StartAndSize {
    #[inline(always)]
    fn from(storage: Storage) -> Self {
        StartAndSize(Start(storage.start()), Size(storage.size()))
    }
}

impl From<StartAndHeight> for StartAndSize {
    #[inline(always)]
    fn from(start_and_height: StartAndHeight) -> Self {
        StartAndSize(Start(start_and_height.start()), Size(start_and_height.size()))
    }
}
// endregion
