use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    #[cfg(target_os = "windows")]
    {
        // 获取当前项目根目录
        let current_dir = env::current_dir().expect("Failed to get current directory");

        // 相对于项目根目录的自定义库路径
        let custom_lib_relative_path = PathBuf::from("libduckdb");

        // 构建绝对路径
        let custom_lib_path = current_dir.join(&custom_lib_relative_path);

        // 将新的路径添加到 LINKER_SEARCH_PATH
        println!("cargo:rustc-link-search=native={}", custom_lib_path.display());
        // 设置要链接的库名称
        println!("cargo:rustc-link-lib=static=duckdb");

        // 获取目标输出目录并转到上级目录
        let out_dir = env::var("OUT_DIR").unwrap();
        let target_dir = PathBuf::from(&out_dir)
            .ancestors()
            .nth(3)
            .expect("Failed to find target directory")
            .to_path_buf();

        // 定义一个递归复制文件的函数
        fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
            if !dst.exists() {
                fs::create_dir_all(dst)?;
            }
            for entry in fs::read_dir(src)? {
                let entry = entry?;
                let file_type = entry.file_type()?;
                if file_type.is_dir() {
                    copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
                } else {
                    fs::copy(entry.path(), dst.join(entry.file_name()))?;
                }
            }
            Ok(())
        }

        // 复制 lib 文件夹中的所有文件到 target 目录
        copy_dir_all(&custom_lib_path, &target_dir).expect("Failed to copy files");
    }
}
