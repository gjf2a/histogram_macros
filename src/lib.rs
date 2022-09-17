macro_rules! bump {
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
        bump!($d, $k, $k.to_owned(), $v)
    }
}

#[macro_export]
macro_rules! bump_copy {
    ($d:expr, $k:expr) => {
        bump_copy_by!($d, $k, 1)
    }
}

#[macro_export]
macro_rules! bump_copy_by {
    ($d:expr, $k:expr, $v:expr) => {
        bump!($d, &$k, $k, $v)
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
        *($d.get($k).unwrap_or(&0))
    }
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
        println!("{:?}", hist);
    }

    #[test]
    fn test_int() {
        let mut hist = HashMap::new();
        bump_copy!(hist, 3);
        bump_copy!(hist, 5);
        bump_copy!(hist, 3);
        assert_eq!(count!(hist, 3), 2);
        println!("{:?}", hist);
    }
}
