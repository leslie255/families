pub trait Family {
    type This<T>;
}

pub use typefamilies_derive::Family;

pub struct OptionFamily;
impl Family for OptionFamily {
    type This<T> = Option<T>;
}

pub struct ResultFamily<T>(std::marker::PhantomData<T>);
impl<E> Family for ResultFamily<E> {
    type This<T> = Result<T, E>;
}
pub struct ResultFamilyFamily;
impl Family for ResultFamilyFamily {
    type This<E> = ResultFamily<E>;
}

pub struct VecFamily;
impl Family for VecFamily {
    type This<T> = Vec<T>;
}

pub struct VecDequeFamily;
impl Family for VecDequeFamily {
    type This<T> = std::collections::VecDeque<T>;
}

pub struct LinkedListFamily;
impl Family for LinkedListFamily {
    type This<T> = std::collections::LinkedList<T>;
}

pub struct HashMapFamily<K>(std::marker::PhantomData<K>);
impl<K> Family for HashMapFamily<K> {
    type This<V> = std::collections::HashMap<K, V>;
}
pub struct HashMapFamilyFamily;
impl Family for HashMapFamilyFamily {
    type This<K> = HashMapFamily<K>;
}

pub struct BTreeMapFamily<K>(std::marker::PhantomData<K>);
impl<K> Family for BTreeMapFamily<K> {
    type This<V> = std::collections::BTreeMap<K, V>;
}
pub struct BTreeMapFamilyFamily;
impl Family for BTreeMapFamilyFamily {
    type This<K> = BTreeMapFamily<K>;
}

pub struct HashSetFamily;
impl Family for HashSetFamily {
    type This<T> = std::collections::HashSet<T>;
}

pub struct BTreeSetFamily;
impl Family for BTreeSetFamily {
    type This<T> = std::collections::BTreeSet<T>;
}

pub struct BinaryHeapFamily;
impl Family for BinaryHeapFamily {
    type This<T> = std::collections::binary_heap::BinaryHeap<T>;
}
