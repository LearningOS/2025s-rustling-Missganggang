fn main() {}

#[cfg(test)]
mod tests {
    // 移除未使用的导入
    // use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}    