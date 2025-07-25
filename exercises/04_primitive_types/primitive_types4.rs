fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // 创建一个包含索引1到3（不包含4）的切片
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
