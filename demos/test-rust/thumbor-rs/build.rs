fn main() {
    // 需要创建 src/pb 目录; cargo build;  ---> pb 目录下有生成的数据结构
    // abi.rs
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
