use std::{cell::RefCell, collections::HashMap, hash::Hash};

mod structs;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct CsvS {
    alfabet: String,
    sex: String,
    age: String,
    count: String,
}

impl CsvS {
    fn new(row: &str) -> Self {
        let mut cell = row.split(",");
        CsvS {
            alfabet: cell.next().unwrap().to_string(),
            sex: cell.next().unwrap().to_string(),
            age: cell.next().unwrap().to_string(),
            count: cell.next().unwrap().to_string(),
        }
    }
    fn gen_vec(source: &str) -> Vec<Self> {
        let rows = source.split("\n");
        let mut vec = Vec::new();
        for row in rows {
            if row.len() == 0 {
                continue;
            }
            vec.push(CsvS::new(row));
        }
        vec
    }
}

impl ToString for CsvS {
    fn to_string_without_count(&self) -> String {
        format!("{},{},{}", self.alfabet, self.sex, self.age)
    }
}

impl CountUp for CsvS {
    fn count(&self, other: &CsvS) -> u32 {
        let count = other.count.parse::<u32>().unwrap();
        count
    }
}

impl Equal for CsvS {
    fn is_equal(&self, other: &Self) -> bool {
        if self.age != other.age {
            return false;
        }
        if self.sex != other.sex {
            return false;
        }
        if self.alfabet == "a" || self.alfabet == "z" {
            if other.alfabet == "a" || other.alfabet == "z" {
                return true;
            }
        }
        if self.alfabet != other.alfabet {
            return false;
        }
        true
    }
}
fn main() {
    let source = r#"a,male,10,2
b,female,20,3
b,female,30,3
z,male,10,3
x,female,30,3
b,female,20,3
b,female,30,3
z,male,10,3
x,female,30,3
"#;
    let source = CsvS::gen_vec(source);
    let conter = Conter::new(source);
    println!("{}", conter.to_string(","));
}

#[derive(Debug)]
struct Conter<T: Equal + ToString + Eq + Hash + Clone + CountUp> {
    table: RefCell<HashMap<T, u32>>,
}

impl<T: Equal + ToString + Clone + Eq + Hash + CountUp> Conter<T> {
    pub fn new(source: Vec<T>) -> Self {
        let counter = Conter {
            table: RefCell::new(HashMap::new()),
        };
        for s in source {
            if let Some(key) = counter.get_count_up_key(&s) {
                *counter.table.borrow_mut().get_mut(&key).unwrap() += key.count(&s);
            } else {
                counter.table.borrow_mut().insert(s.clone(), s.count(&s));
            }
        }
        counter
    }
    pub fn to_string(&self, separate: &str) -> String {
        let table = self.table.borrow();
        let keys = table.keys();
        keys.fold(String::new(), |acc, cur| {
            format!(
                "{}{}{}{}\n",
                acc,
                cur.to_string_without_count(),
                separate,
                self.table.borrow().get(cur).unwrap(),
            )
        })
    }
    fn get_count_up_key(&self, comp: &T) -> Option<T> {
        let table = self.table.borrow();
        let keys = table.keys();
        for key in keys {
            if comp.is_equal(key) {
                return Some(key.clone());
            }
        }
        None
    }
}
trait ToString {
    fn to_string_without_count(&self) -> String;
}
trait CountUp {
    fn count(&self, _: &Self) -> u32 {
        1
    }
}
trait Equal {
    fn is_equal(&self, other: &Self) -> bool;
}

#[test]
fn to_string_test() {
    let stubs = vec![
        Stub {
            value: "a".to_string(),
        },
        Stub {
            value: "a".to_string(),
        },
        Stub {
            value: "b".to_string(),
        },
    ];
    let conter = Conter::new(stubs);
    assert_eq!(
        conter.to_string(","),
        r#"a,2
b,1
"#
        .to_string()
    )
}
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Stub {
    value: String,
}
impl Equal for Stub {
    fn is_equal(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl ToString for Stub {
    fn to_string_without_count(&self) -> String {
        self.value.clone()
    }
}
impl CountUp for Stub {}

#[test]
fn new_test() {
    assert!(CsvS::new("a,male,10").is_equal(&CsvS::new("z,male,10")))
}
#[test]
fn test() {
    let source = r#"a,male,10
b,female,20
b,female,30
"#;
    assert_eq!(
        CsvS::gen_vec(source),
        vec![
            CsvS::new("a,male,10"),
            CsvS::new("b,female,20"),
            CsvS::new("b,female,30"),
        ]
    )
}
