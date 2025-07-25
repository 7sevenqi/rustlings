mod sausage_factory {
    // 保持私有，仅模块内部可见
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 标记为 pub，允许外部调用
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage(); // 现在可以访问公开函数
}
