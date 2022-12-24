static INT_NULL : i32 = i32::MIN;

pub fn is_int_null(int: i32) -> bool {
    if int == INT_NULL {
        return true;
    }
    return false;
}
