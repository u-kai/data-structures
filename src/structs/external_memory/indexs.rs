use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) struct BIndex(usize);
impl Deref for BIndex {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<usize> for BIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}
impl From<BIndex> for usize {
    fn from(index: BIndex) -> Self {
        *index
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) struct KeyIndex(usize);
impl Deref for KeyIndex {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<KeyIndex> for usize {
    fn from(key: KeyIndex) -> Self {
        *key
    }
}
impl From<usize> for KeyIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) struct ChildIndex(usize);
impl Deref for ChildIndex {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<ChildIndex> for usize {
    fn from(key: ChildIndex) -> Self {
        *key
    }
}

impl From<usize> for ChildIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}
