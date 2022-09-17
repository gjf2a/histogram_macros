macro_rules! bump_skeleton {
    ($d:expr, $kg:expr, $ki:expr, $v:expr) => {
        match $d.get_mut($kg) {
            None => {$d.insert($ki, $v);}
            Some(count) => {*count += $v}
        }
    }
}

#[macro_export]
macro_rules! bump_ref {
    ($d:expr, $k:expr) => {
        bump_ref_by!($d, $k, 1)
    }
}

#[macro_export]
macro_rules! bump_ref_by {
    ($d:expr, $k:expr, $v:expr) => {
        bump_skeleton!($d, $k, $k.to_owned(), $v)
    }
}

#[macro_export]
macro_rules! bump {
    ($d:expr, $k:expr) => {
        bump_by!($d, $k, 1)
    }
}

#[macro_export]
macro_rules! bump_by {
    ($d:expr, $k:expr, $v:expr) => {
        bump_skeleton!($d, &$k, $k, $v)
    }
}

#[macro_export]
macro_rules! count {
    ($d:expr, $k:expr) => {
        count_ref!($d, &$k)
    }
}

#[macro_export]
macro_rules! count_ref {
    ($d:expr, $k:expr) => {
        get_skeleton!($d, $k, 0)
    }
}

#[macro_export]
macro_rules! weight {
    ($d:expr, $k:expr) => {
        weight_ref!($d, &$k)
    }
}

#[macro_export]
macro_rules! weight_ref {
    ($d:expr, $k:expr) => {
        get_skeleton!($d, $k, 0.0)
    }
}

macro_rules! get_skeleton {
    ($d:expr, $k:expr, $z:expr) => {
        *($d.get($k).unwrap_or(&$z))
    }
}

macro_rules! total_skeleton {
    ($d:expr, $z:expr) => {
        $d.iter().map(|(_,value)| value).copied().reduce(|acc, n| acc + n).unwrap_or($z)
    }
}

#[macro_export]
macro_rules! total {
    ($d:expr) => {total_skeleton!($d, 0)}
}

#[macro_export]
macro_rules! total_weight {
    ($d:expr) => {total_skeleton!($d, 0.0)}
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_str() {
        let mut hist = HashMap::new();
        bump_ref!(hist, "walk");
        bump_ref!(hist, "talk");
        bump_ref!(hist, "walk");
        bump_ref!(hist, "balk");
        assert_eq!(count_ref!(hist, "walk"), 2);
        assert_eq!(count_ref!(hist, "balk"), 1);
        assert_eq!(count_ref!(hist, "talk"), 1);
        assert_eq!(count_ref!(hist, "sulk"), 0);
        assert_eq!(total!(hist), 4);
    }

    #[test]
    fn test_string() {
        let mut hist = HashMap::new();
        bump!(hist, "walk".to_owned());
        bump!(hist, "talk".to_owned());
        bump!(hist, "walk".to_owned());
        bump!(hist, "balk".to_owned());
        assert_eq!(count!(hist, "walk".to_owned()), 2);
        assert_eq!(count!(hist, "balk".to_owned()), 1);
        assert_eq!(count!(hist, "talk".to_owned()), 1);
        assert_eq!(count!(hist, "sulk".to_owned()), 0);
        assert_eq!(total!(hist), 4);
    }

    #[test]
    fn test_int() {
        let mut hist = HashMap::new();
        bump!(hist, 3);
        bump!(hist, 5);
        bump!(hist, 3);
        assert_eq!(count!(hist, 3), 2);
        assert_eq!(count!(hist, 4), 0);
        assert_eq!(count!(hist, 5), 1);
        assert_eq!(total!(hist), 3);
    }

    #[test]
    fn test_float() {
        let mut hist = HashMap::new();
        bump_ref_by!(hist, "hi", 1.5);
        bump_ref_by!(hist, "bye", 2.6);
        bump_ref_by!(hist, "hi", 0.3);
        assert_eq!(weight_ref!(hist, "hi"), 1.8);
        assert_eq!(weight_ref!(hist, "bye"), 2.6);
        assert_eq!(total_weight!(hist), 4.4);
    }
}
