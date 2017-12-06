use factory::Map;

// Hint: Create the TreeDict<K, V> type and implement everything
// that is necessary to make the code work.

impl<K, V> Map<K, V> for TreeDict<K, V>
    where K: PartialOrd,
          V: Clone
{
    fn insert(&mut self, k: K, v: V) {
        unimplemented!();
    }

    fn get(&self, k: &K) -> Option<V> {
        unimplemented!();
    }
}
