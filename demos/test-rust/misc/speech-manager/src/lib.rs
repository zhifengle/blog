use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::process::exit;
use std::process::Command;
use text_io::read;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub fn pause() {
    // use std::io;
    // use std::io::prelude::*;
    // let mut stdin = io::stdin();
    // let mut stdout = io::stdout();

    // // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    // write!(stdout, "Press any key to continue...\n").unwrap();
    // stdout.flush().unwrap();

    // // Read a single byte and discard
    // let _ = stdin.read(&mut [0u8]).unwrap();

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
pub fn clear() {
    // Command::new("cls");
    // @TODO clear
    // 这种方式失败了
    // if Command::new("cls").status().unwrap().success() {
    //     println!("screen successfully cleared");
    //     self.show_menu()
    // }
    // https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed

    // print!("{}[2J", 27 as char);
    // print!("{esc}c", esc = 27 as char);
    print!("\x1B[2J\x1B[1;1H");
}

#[derive(Default)]
pub struct Speech {
    stage: u32,
    v1: Vec<u32>,
    v2: Vec<u32>,
    // 决赛
    v3: Vec<u32>,
    speakers: HashMap<u32, Speaker>,
    // 往届记录
    record: HashMap<u32, Vec<String>>,
}

impl Speech {
    pub fn new() -> Self {
        let mut speech = Speech::default();
        speech.init_speech();
        speech.create_speaker();
        // @TODO 这里直接初始化了 v1
        speech.speakers.iter().for_each(|(i, _)| {
            speech.v1.push(*i);
        });
        speech.load_record();

        speech
    }
    pub fn show_menu(&self) {
        println!("**************************");
        println!("****欢迎参加演讲比赛****");
        println!("**** 1.开始演讲比赛 ****");
        println!("**** 2.查看往届记录 ****");
        println!("**** 3.清空比赛记录 ****");
        println!("**** 0.退出比赛程序 ****");
    }
    pub fn exit_system(&self) {
        println!("欢迎下次使用");
        // Windows 下面是这样的
        // Linux 可以看这里
        // https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494
        pause();
        exit(0);
    }
    pub fn init_speech(&mut self) {
        self.v1.clear();
        self.v2.clear();
        self.v3.clear();
        self.speakers.clear();
        // 后面加的忘了初始化
        self.stage = 1;
        self.record.clear();

        self.stage = 1;
    }
    fn create_speaker(&mut self) {
        let name_seed = "ABCDEFGHIJKL";
        name_seed.chars().enumerate().for_each(|(i, c)| {
            let mut name = "选手".to_string();
            name.push(c);
            let score = vec![0 as f32; 2];
            let sp = Speaker { name, score };
            let num = (10001 + i) as u32;
            self.speakers.insert(num, sp);
        });

        // test print
        // for (num, sp) in self.speakers.iter() {
        //     println!("id: {}. name: {}, score: {}", num, sp.name, sp.score[0])
        // }
    }
    pub fn start_speech(&mut self) {
        self.draw_speech();
        self.speech_contest();
        self.show_score();

        // 第二轮
        self.stage += 1;
        self.draw_speech();
        self.speech_contest();
        self.show_score();
        // 保存分数
        self.save_record().unwrap_or_else(|e| {
            eprintln!("error: {}", e);
        });
        // 重置
        self.init_speech();
        self.create_speaker();
        self.load_record();

        println!("本届比赛完毕！");
        pause();
        clear();
    }
    pub fn draw_speech(&mut self) {
        println!("第 {} 轮比赛选手正在抽签", self.stage);
        println!("---------------------");
        println!("抽签后的演讲顺序如下：");
        // 随机 vector
        let mut rng = thread_rng();
        match self.stage {
            1 => {
                self.v1.shuffle(&mut rng);
                self.v1.iter().for_each(|num| {
                    print!("{} ", num);
                });
                println!();
            }
            2 => {
                self.v2.shuffle(&mut rng);
                self.v2.iter().for_each(|num| {
                    print!("{} ", num);
                });
                println!();
            }
            _ => {
                // TODO
            }
        }
        println!("---------------------");
        pause();
        println!();
    }
    fn speech_contest(&mut self) {
        println!(
            "-------------第 {} 轮正式比赛开始---------------",
            self.stage
        );
        let mut participants = self.v1.clone();
        if self.stage == 2 {
            participants = self.v2.clone();
        }
        // 小组
        let mut num = 0;
        let mut group_scores: Vec<(f32, u32)> = vec![];
        for p_id in participants.iter() {
            // 使用 Deque 方便去除头尾
            num += 1;
            let avg = get_avg_score();
            // 需要使用 get_mut
            let sp = self.speakers.get_mut(p_id).unwrap();
            // println!(
            //     "选手编号: {}, 选手姓名: {}, 平均分: {:.1}",
            //     p_id, sp.name, avg
            // );
            // 写入分数
            let idx = (self.stage - 1) as usize;
            sp.score[idx] = avg;
            group_scores.push((avg, *p_id));
            if num % 6 == 0 {
                println!("第 {} 组名次如下: ", num / 6);
                // i32 可以直接用 b.1.cmp(a.1)
                group_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
                for (_, p_id) in group_scores.iter() {
                    self.print_contestant(*p_id);
                }
                // 一组取三个
                for (_, p_id) in group_scores.iter().take(3) {
                    if self.stage == 1 {
                        self.v2.push(*p_id);
                    } else if self.stage == 2 {
                        self.v3.push(*p_id);
                    }
                }
                group_scores.clear();
                println!();
            }
        }
        println!("-------------第 {} 轮比赛完毕----------", self.stage);
        pause();
    }
    fn print_contestant(&self, id: u32) {
        let sp = self.speakers.get(&id).unwrap();
        let idx = (self.stage - 1) as usize;
        println!(
            "选手编号: {}, 选手姓名: {}, 得分: {:.1}",
            id, sp.name, sp.score[idx]
        );
    }
    fn show_score(&self) {
        println!(
            "------------- 第 {} 轮晋级选手信息如下: -------------",
            self.stage
        );
        let v = {
            if self.stage == 1 {
                &self.v2
            } else {
                &self.v3
            }
        };
        for p_id in v.iter() {
            self.print_contestant(*p_id);
        }
        println!();
        pause();
        clear();
        // 这个打印是为了美观需要
        self.show_menu();
    }
    fn save_record(&self) -> Result<(), Error> {
        let path = "speech.csv";

        let mut output = File::create(path)?;
        for id in self.v3.iter() {
            let sp = self.speakers.get(&id).unwrap();
            write!(output, "{},{},", id, sp.score[1])?;
        }
        println!("记录已经保存");

        Ok(())
    }
    fn load_record(&mut self) {
        let path = "speech.csv";
        match File::open(path) {
            Ok(input) => {
                let buffered = BufReader::new(input);
                for (i, line) in buffered.lines().into_iter().enumerate() {
                    let line = line.unwrap();
                    let mut v = vec![];
                    for item in line.split(",") {
                        v.push(item.to_string());
                    }
                    self.record.insert(i as u32, v);
                }
            }
            Err(_) => {
                println!("文件不存在!");
            }
        }
    }
    pub fn show_record(&self) {
        if self.record.is_empty() {
            println!("文件不存在或者记录为空");
            return;
        }
        for (i, arr) in self.record.iter() {
            print!("第{}界 ", i + 1);
            print!("冠军编号: {} 得分: {} ", arr[0], arr[1]);
            print!("亚军编号: {} 得分: {} ", arr[2], arr[3]);
            println!("季军编号: {} 得分: {}", arr[4], arr[5]);
        }
        pause();
        clear();
    }
    pub fn clear_record(&mut self) {
        println!("是否要清空数据?");
        println!("1.是");
        println!("2.否");
        let select: u32 = read!();
        if select == 1 {
            let path = "speech.csv";
            let mut output = File::create(path).unwrap();
            write!(output, "").unwrap();
            self.init_speech();
            self.create_speaker();
            self.load_record();
            println!("清空成功");
        }
        pause();
        clear();
    }
}

fn get_avg_score() -> f32 {
    let mut rng = thread_rng();

    let mut scores = VecDeque::new();
    for _ in 0..10 {
        let num: i32 = rng.gen_range(600..=1000);
        let score: f32 = num as f32 / 10.0;
        // 打印1位小数点
        // print!("{:.1} ", score);
        scores.push_back(score);
    }
    // println!();
    scores
        .make_contiguous()
        .sort_by(|a, b| a.partial_cmp(b).unwrap());
    // 去除最高和最低
    scores.pop_back();
    scores.pop_front();
    let sum: f32 = scores.iter().sum();
    let avg = sum / scores.len() as f32;
    avg
}

pub struct Speaker {
    pub name: String,
    pub score: Vec<f32>,
}
