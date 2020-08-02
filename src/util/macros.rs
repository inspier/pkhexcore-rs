#![allow(unused_macros)]

macro_rules! array_ref {
    ($arr:expr, $offset:expr, $len:expr) => {{
        {
            let offset = $offset;
            let len = $len;
            $arr[offset..offset + len]
                .try_into()
                .expect("Failed to create array from buffer.")
        }
    }};
}

macro_rules! array_two {
    ($arr:expr, $offset:expr) => {{
        array_ref![$arr, $offset, 2]
    }};
}

macro_rules! array_four {
    ($arr:expr, $offset:expr) => {{
        array_ref![$arr, $offset, 4]
    }};
}

macro_rules! array_eight {
    ($arr:expr, $offset:expr) => {{
        array_ref![$arr, $offset, 8]
    }};
}
