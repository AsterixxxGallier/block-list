#![allow(unused, reason =
"This module is designed to be complete in its functionality, even though parts are not used.")]

use std::iter::repeat;
use std::num::NonZeroUsize;
use itertools::iterate;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Height(pub usize);

impl Height {
    #[inline(always)]
    pub fn get(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn size(self) -> Size {
        Size(1 << self.get())
    }

    #[inline(always)]
    pub fn height_at_zero(self) -> HeightAtZero {
        HeightAtZero(self)
    }

    #[inline(always)]
    pub fn size_at_zero(self) -> SizeAtZero {
        SizeAtZero(self.size())
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=Height> {
        (0..self.get()).rev().map(Height)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Size(pub usize);

impl Size {
    #[inline(always)]
    pub fn get(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn height(self) -> Height {
        Height(self.get().trailing_zeros() as usize)
    }

    #[inline(always)]
    pub fn height_at_zero(self) -> HeightAtZero {
        HeightAtZero(self.height())
    }

    #[inline(always)]
    pub fn size_at_zero(self) -> SizeAtZero {
        SizeAtZero(self)
    }

    #[inline(always)]
    pub fn parent(self) -> Size {
        Size(self.get() << 1)
    }

    #[inline(always)]
    pub fn parent_bounded(self, len: usize) -> Option<Size> {
        Some(self.parent()).filter(|it| it.get() <= len)
    }

    #[inline(always)]
    pub fn parents(self) -> impl Iterator<Item=Size> {
        iterate(self, |block| block.parent()).skip(1)
    }

    #[inline(always)]
    pub fn parents_bounded(self, len: usize) -> impl Iterator<Item=Size> {
        self.parents().take_while(move |block| block.get() <= len)
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=Size> {
        iterate(self.get(), |size| size >> 1).skip(1)
            .take_while(|size| *size > 0).map(Size)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Start(pub usize);

impl Start {
    #[inline(always)]
    pub fn get(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=Start> {
        repeat(self)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct End(pub usize);

impl End {
    #[inline(always)]
    pub fn get(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn height(self) -> Height {
        Height(self.get().trailing_zeros() as usize)
    }

    #[inline(always)]
    pub fn size(self) -> Size {
        self.height().size()
    }

    #[inline(always)]
    pub fn start(self) -> Start {
        Start(self.get() - self.size().get())
    }

    #[inline(always)]
    pub fn storage(self) -> Storage {
        Storage(self.get() - 1)
    }

    #[inline(always)]
    pub fn start_and_height(self) -> StartAndHeight {
        StartAndHeight(self.start(), self.height())
    }

    #[inline(always)]
    pub fn start_and_size(self) -> StartAndSize {
        StartAndSize(self.start(), self.size())
    }

    #[inline(always)]
    pub fn parent(self) -> End {
        End(self.get() + self.size().get())
    }

    #[inline(always)]
    pub fn parent_bounded(self, len: usize) -> Option<End> {
        Some(self.parent()).filter(|it| it.get() <= len)
    }

    #[inline(always)]
    pub fn parents(self) -> impl Iterator<Item=End> {
        iterate(self, |block| block.parent()).skip(1)
    }

    #[inline(always)]
    pub fn parents_bounded(self, len: usize) -> impl Iterator<Item=End> {
        self.parents().take_while(move |block| block.get() <= len)
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=End> {
        self.size().children().map(move |size| End(self.get() - size.get()))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Storage(pub usize);

impl Storage {
    #[inline(always)]
    pub fn get(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn height(self) -> Height {
        Height(self.get().trailing_ones() as usize)
    }

    #[inline(always)]
    pub fn size(self) -> Size {
        self.height().size()
    }

    #[inline(always)]
    pub fn start(self) -> Start {
        self.end().start()
    }

    #[inline(always)]
    pub fn end(self) -> End {
        End(self.get() + 1)
    }

    #[inline(always)]
    pub fn start_and_height(self) -> StartAndHeight {
        StartAndHeight(self.start(), self.height())
    }

    #[inline(always)]
    pub fn start_and_size(self) -> StartAndSize {
        StartAndSize(self.start(), self.size())
    }

    #[inline(always)]
    pub fn parent(self) -> Storage {
        Storage(self.get() + self.size().get())
    }

    #[inline(always)]
    pub fn parent_bounded(self, len: usize) -> Option<Storage> {
        Some(self.parent()).filter(|it| it.get() < len)
    }

    #[inline(always)]
    pub fn parents(self) -> impl Iterator<Item=Storage> {
        iterate(self, |block| block.parent()).skip(1)
    }

    #[inline(always)]
    pub fn parents_bounded(self, len: usize) -> impl Iterator<Item=Storage> {
        self.parents().take_while(move |block| block.get() < len)
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=Storage> {
        self.size().children().map(move |size| Storage(self.get() - size.get()))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct StartAndHeight(pub Start, pub Height);

impl StartAndHeight {
    #[inline(always)]
    pub fn height(self) -> Height {
        self.1
    }

    #[inline(always)]
    pub fn size(self) -> Size {
        self.height().size()
    }

    #[inline(always)]
    pub fn start(self) -> Start {
        self.0
    }

    #[inline(always)]
    pub fn end(self) -> End {
        End(self.start().get() + self.size().get())
    }

    #[inline(always)]
    pub fn storage(self) -> Storage {
        self.end().storage()
    }

    #[inline(always)]
    pub fn start_and_size(self) -> StartAndSize {
        StartAndSize(self.start(), self.size())
    }

    #[inline(always)]
    pub fn height_at_zero(self) -> HeightAtZero {
        HeightAtZero(self.height())
    }

    #[inline(always)]
    pub fn size_at_zero(self) -> SizeAtZero {
        SizeAtZero(self.size())
    }

    #[inline(always)]
    pub fn parent(self) -> StartAndHeight {
        let height = self.height().get() + self.start().get().trailing_ones() as usize;
        StartAndHeight(Start(self.start().get() >> height << height), Height(height + 1))
    }

    #[inline(always)]
    pub fn parent_bounded(self, len: usize) -> Option<StartAndHeight> {
        Some(self.parent()).filter(|it| it.end().get() <= len)
    }

    #[inline(always)]
    pub fn parents(self) -> impl Iterator<Item=StartAndHeight> {
        iterate(self, |block| block.parent()).skip(1)
    }

    #[inline(always)]
    pub fn parents_bounded(self, len: usize) -> impl Iterator<Item=StartAndHeight> {
        self.parents().take_while(move |block| block.end().get() <= len)
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=StartAndHeight> {
        self.height().children().map(move |height| StartAndHeight(self.start(), height))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct StartAndSize(pub Start, pub Size);

impl StartAndSize {
    #[inline(always)]
    pub fn height(self) -> Height {
        self.size().height()
    }

    #[inline(always)]
    pub fn size(self) -> Size {
        self.1
    }

    #[inline(always)]
    pub fn start(self) -> Start {
        self.0
    }

    #[inline(always)]
    pub fn end(self) -> End {
        End(self.start().get() + self.size().get())
    }

    #[inline(always)]
    pub fn storage(self) -> Storage {
        Storage(self.end().get() - 1)
    }

    #[inline(always)]
    pub fn start_and_height(self) -> StartAndHeight {
        StartAndHeight(self.start(), self.height())
    }

    #[inline(always)]
    pub fn height_at_zero(self) -> HeightAtZero {
        HeightAtZero(self.height())
    }

    #[inline(always)]
    pub fn size_at_zero(self) -> SizeAtZero {
        SizeAtZero(self.size())
    }

    #[inline(always)]
    pub fn parent(self) -> StartAndSize {
        End(self.end().get() + self.size().get()).start_and_size()
    }

    #[inline(always)]
    pub fn parent_bounded(self, len: usize) -> Option<StartAndSize> {
        Some(self.parent()).filter(|it| it.end().get() <= len)
    }

    #[inline(always)]
    pub fn parents(self) -> impl Iterator<Item=StartAndSize> {
        iterate(self, |block| block.parent()).skip(1)
    }

    #[inline(always)]
    pub fn parents_bounded(self, len: usize) -> impl Iterator<Item=StartAndSize> {
        self.parents().take_while(move |block| block.end().get() <= len)
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=StartAndSize> {
        self.size().children().map(move |size| StartAndSize(self.0, size))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct HeightAtZero(pub Height);

impl HeightAtZero {
    #[inline(always)]
    pub fn height(self) -> Height {
        self.0
    }

    #[inline(always)]
    pub fn size(self) -> Size {
        self.height().size()
    }

    #[inline(always)]
    pub fn start(self) -> Start {
        Start(0)
    }

    #[inline(always)]
    pub fn end(self) -> End {
        End(self.size().get())
    }

    #[inline(always)]
    pub fn storage(self) -> Storage {
        self.end().storage()
    }

    #[inline(always)]
    pub fn start_and_height(self) -> StartAndHeight {
        StartAndHeight(self.start(), self.height())
    }

    #[inline(always)]
    pub fn start_and_size(self) -> StartAndSize {
        StartAndSize(self.start(), self.size())
    }

    #[inline(always)]
    pub fn size_at_zero(self) -> SizeAtZero {
        SizeAtZero(self.size())
    }

    #[inline(always)]
    pub fn parent(self) -> HeightAtZero {
        self.height().parent().height_at_zero()
    }

    #[inline(always)]
    pub fn parent_bounded(self, len: usize) -> Option<HeightAtZero> {
        Some(self.parent()).filter(|it| it.end().get() <= len)
    }

    #[inline(always)]
    pub fn parents(self) -> impl Iterator<Item=HeightAtZero> {
        iterate(self, |block| block.parent()).skip(1)
    }

    #[inline(always)]
    pub fn parents_bounded(self, len: usize) -> impl Iterator<Item=HeightAtZero> {
        self.parents().take_while(move |block| block.end().get() <= len)
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=HeightAtZero> {
        self.height().children().map(move |height| HeightAtZero(self.start(), height))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct SizeAtZero(pub Size);

impl SizeAtZero {
    #[inline(always)]
    pub fn height(self) -> Height {
        self.size().height()
    }

    #[inline(always)]
    pub fn size(self) -> Size {
        self.0
    }

    #[inline(always)]
    pub fn start(self) -> Start {
        Start(0)
    }

    #[inline(always)]
    pub fn end(self) -> End {
        End(self.size().get())
    }

    #[inline(always)]
    pub fn storage(self) -> Storage {
        Storage(self.end().get() - 1)
    }

    #[inline(always)]
    pub fn start_and_height(self) -> StartAndHeight {
        StartAndHeight(self.start(), self.height())
    }

    #[inline(always)]
    pub fn start_and_size(self) -> StartAndSize {
        StartAndSize(self.start(), self.size())
    }

    #[inline(always)]
    pub fn height_at_zero(self) -> HeightAtZero {
        HeightAtZero(self.height())
    }

    #[inline(always)]
    pub fn parent(self) -> SizeAtZero {
        SizeAtZero(self.size().parent())
    }

    #[inline(always)]
    pub fn parent_bounded(self, len: usize) -> Option<SizeAtZero> {
        Some(self.parent()).filter(|it| it.end().get() <= len)
    }

    #[inline(always)]
    pub fn parents(self) -> impl Iterator<Item=SizeAtZero> {
        iterate(self, |block| block.parent()).skip(1)
    }

    #[inline(always)]
    pub fn parents_bounded(self, len: usize) -> impl Iterator<Item=SizeAtZero> {
        self.parents().take_while(move |block| block.end().get() <= len)
    }

    #[inline(always)]
    pub fn children(self) -> impl Iterator<Item=SizeAtZero> {
        self.size().children().map(move |size| SizeAtZero(self.0, size))
    }
}
