use speech_manager::{clear, Speech};
use text_io::read;

fn main() {
    // c++ 例子需要随机的 seed??
    let mut speech = Speech::new();
    let mut choice: u32 = 0;
    loop {
        speech.show_menu();
        println!("请输入您的选择： ");
        choice = read!();
        match choice {
            1 => {
                // 开始
                speech.start_speech();
            }
            2 => {
                // 往届记录
                speech.show_record();
            }
            3 => {
                // 清空记录
                speech.clear_record();
            }
            0 => {
                // 退出
                speech.exit_system();
            }
            _ => {
                clear();
            }
        }
    }
    // 读取输入
    /*
    let mut numbers = String::new();
    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");
    let mut iter = numbers.split_whitespace();
     */
}
