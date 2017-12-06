mod factory;
mod tree_dict;

#[cfg(not(test))]
fn main() {
    println!("Run \"cargo test\" instead of \"cargo run\".");
}

#[cfg(test)]
mod test {
    use super::factory::*;

    #[test]
    fn insertion() {
        let mut map = make_map();
        map.insert("A", 5);
        assert_eq!(map.get(&"A"), Some(5));
    }

    #[test]
    fn replacement() {
        let mut map = make_map();
        map.insert("Hola", 42);
        map.insert("Nuevo", 12);
        map.insert("Hola", 83);
        assert_eq!(map.get(&"Hola"), Some(83));
    }

    #[test]
    fn nil_value() {
        let map: Box<Map<&'static str, i32>> = make_map();
        assert_eq!(map.get(&"Nice key!"), None);
    }
}
