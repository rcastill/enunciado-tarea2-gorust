use tree_dict::TreeDict;

pub trait Map<K, V>
    where K: PartialOrd,
          V: Clone
{
    fn insert(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<V>;

    fn has_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}

pub fn make_map<K, V>() -> Box<Map<K, V>>
    where K: 'static + PartialOrd,
          V: 'static + Clone
{
    Box::new(TreeDict::new())
}
