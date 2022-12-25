static INT_NULL : i32 = i32::MIN;
static STRING_NULL : &str = "Cift : String : None";

pub fn is_int_null(int: i32) -> bool {
    if &int == &INT_NULL {
        return true;
    }
    return false;
}

pub fn is_string_null(string: String) -> bool {
    if &string == &STRING_NULL {
        return true;
    }
    return false;
}
