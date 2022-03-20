use room_order::{clear, pause};
use text_io::read;

fn main() {
    loop {
        println!("================ Welcome room order system =================");
        println!("请输入您的身份");
        println!("\t\t---------------------------------");
        println!("\t\t|                               |");
        println!("\t\t|           1.学生               |");
        println!("\t\t|                               |");
        println!("\t\t|           2.老师               |");
        println!("\t\t|                               |");
        println!("\t\t|           3.admin             |");
        println!("\t\t|                               |");
        println!("\t\t|           0.exit              |");
        println!("\t\t|                               |");
        println!("\t\t---------------------------------");
        println!("请输入您的选择： ");
        let choice: u32 = read!();
        match choice {
            1 => {
                // Student
            }
            2 => {
                // Teacher
            }
            3 => {
                // Admin
            }
            4 => {
                // exit
                println!("欢迎下次使用");
                pause();
                std::process::exit(0);
            }
            _ => {
                println!("无效输入，请重新输入!");
                pause();
                clear();
            }
        }
    }
}
