use std::{cell::RefCell, collections::HashMap, hash::Hash};

/// An abstract data structure that can be used to store information with nested properties.
/// The values inputed are stored in `fields`, which work like the scopes of a program, i.e. a field can have fields within itself.
/// Child fields can be created within a field, and the values of outer fields can be accessed from within a child field.
#[derive(Default, Debug, Clone)]
pub struct Stage<K, V>
where
    K: Eq + Hash,
{
    _fields: Vec<Field<K, V>>,
    _forgetful: bool,
    _levels: usize,
    _current_f: usize,
}

impl<K, V> Stage<K, V>
where
    K: Eq + Hash,
{
    /// Create a new stage with a global field.
    pub fn new() -> Self {
        Stage {
            _fields: Vec::from([Field::global()]),
            _forgetful: false,
            _levels: 0,
            _current_f: 0,
        }
    }
    /// By default, when a field is exited, its data values are still present in the stage.
    /// A 'forgetful' stage destroys any field immediately it is exited.
    pub fn forgetful() -> Self {
        Stage {
            _fields: Vec::from([Field::global()]),
            _forgetful: true,
            _current_f: 0,
            _levels: 0,
        }
    }
    /// Sets a value to a key in the current field.
    pub fn set(&mut self, key: K, value: V) {
        self._get_current_field().set(key, value);
    }
    /// Searches for a value using its key.
    /// The main difference between `lookup` and `get` is that `lookup` goes up and searches through parent fields until it finds a match for the key.
    pub fn lookup(&self, key: K) -> Option<&V> {
        match self._get_current_field().get(&key) {
            Some(s) => Some(s),
            None => {
                let mut current = self._get_current_field().id;
                while let Some(id) = self._fields[current].parent {
                    current = id;
                    if let Some(s) = self._fields[current].get(&key) {
                        return Some(s);
                    }
                }
                return None;
            }
        }
    }
    /// Searches for a value using its key in the current field.
    pub fn get(&mut self, key: K) -> Option<&V> {
        self._get_current_field().get(&key)
    }
    /// Creates a field within the current field and sets it as the new current field.
    pub fn create_inner(&mut self) {
        let new_scope = Field::new(self._current_f, self._fields.len());
        self._fields[self._current_f].children.push(new_scope.id);
        self._levels += 1;
        self._current_f = new_scope.id;
        self._fields.push(new_scope);
    }
    /// Leaves the current field and returns to its parent.
    pub fn ascend(&mut self) {
        match self._get_current_field().parent {
            Some(id) => {
                self._current_f = id;
                if self._forgetful {
                    self._fields.pop();
                }
            }
            None => panic!("Cannot leave global field."),
        }
        self._levels -= 1;
    }
    fn _get_current_field(&self) -> &Field<K, V> {
        &self._fields[self._current_f]
    }
    /// Returns the depth of the stage. i.e the level of field nesting.
    pub fn depth(&mut self) -> usize {
        self._levels
    }
}

/// The unit of a stage.
#[derive(Default, Debug, Clone)]
struct Field<K, V>
where
    K: Eq,
{
    id: usize,
    parent: Option<usize>,
    children: Vec<usize>,
    symbols: RefCell<HashMap<K, V>>,
}

impl<K, V> Field<K, V>
where
    K: Eq + Hash,
{
    fn global() -> Self {
        Field {
            id: 0,
            parent: None,
            children: Vec::new(),
            symbols: RefCell::new(HashMap::new()),
        }
    }
    fn new(parent: usize, id: usize) -> Self {
        Field {
            id,
            parent: Some(parent),
            children: Vec::new(),
            symbols: RefCell::new(HashMap::new()),
        }
    }
    fn set(&self, name: K, value: V) {
        self.symbols.borrow_mut().insert(name, value);
    }
    fn get(&self, name: &K) -> Option<&V> {
        unsafe { self.symbols.try_borrow_unguarded().unwrap().get(name) }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic = "Cannot leave global field."]
    fn it_panics_on_leaving_global_scope() {
        let mut stage: Stage<&str, &str> = Stage::new();
        stage.ascend();
    }

    #[test]
    fn it_shadows_outer_field() {
        let mut stage = Stage::new();
        stage.set("name", "Sefunmi");
        stage.create_inner();
        stage.set("name", "Ezra");
        assert_eq!(stage.get("name"), Some(&"Ezra"));
    }

    #[test]
    fn really_really_nested_field() {
        let mut stage = Stage::new();
        stage.create_inner();
        stage.create_inner();
        stage.create_inner();
        stage.set("something", 90);
        stage.create_inner();
        stage.create_inner();
        stage.create_inner();
        assert_eq!(stage.lookup("something"), Some(&90));
    }

    #[test]
    fn it_ascends() {
        let mut stage = Stage::forgetful();
        stage.create_inner();
        stage.set("name", "Ezra");
        stage.create_inner();
        stage.set("name", "Sefunmi");
        stage.ascend();
        assert_eq!(stage.lookup("name"), Some(&"Ezra"));
    }
}
