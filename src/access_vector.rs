use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::FromIterator;

pub trait AccessPermission: Clone + Debug + Eq + Hash {
    fn allowed_class() -> &'static AccessVector<Self>;
}

#[derive(Clone, Debug)]
pub struct AccessVector<T>
    where T: AccessPermission
{
    av: HashSet<T>,
}

impl<T> AccessVector<T>
    where T: AccessPermission
{
    pub fn new(av: Vec<T>) -> AccessVector<T> {
        AccessVector {
            av: HashSet::from_iter(av),
        }
    }

    pub fn has(&self, perms: &AccessVector<T>) -> bool {
        self.av.is_superset(&perms.av)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Eq, Hash, PartialEq, Debug)]
    pub enum Sample {
        Read,
        Write,
        Execute,
    }

    impl AccessPermission for Sample {
        fn allowed_class() -> &'static AccessVector<Sample> {
            lazy_static! {
                static ref ALLOWED: AccessVector<Sample> = AccessVector::new(vec![Sample::Read, Sample::Execute]);
            }

            &ALLOWED
        }
    }


    #[test]
    fn has_permission() {
        let permissions = AccessVector::new(vec![Sample::Read, Sample::Write, Sample::Execute]);
        let check = AccessVector::new(vec![Sample::Read]);

        assert_eq!(permissions.has(&check), true);
    }

    #[test]
    fn has_no_permission() {
        let permissions = AccessVector::new(vec![Sample::Write, Sample::Execute]);
        let check = AccessVector::new(vec![Sample::Read]);

        assert_eq!(permissions.has(&check), false);
    }

    #[test]
    fn in_allowed_class() {
        let check_true = AccessVector::new(vec![Sample::Read]);
        let check_false = AccessVector::new(vec![Sample::Write]);

        assert_eq!(Sample::allowed_class().has(&check_true), true);
        assert_eq!(Sample::allowed_class().has(&check_false), false);
    }
}
