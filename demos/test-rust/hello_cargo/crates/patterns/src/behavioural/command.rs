//!https://rust-unofficial.github.io/patterns/patterns/behavioural/command.html

// 分离actions为对象，把对象当作参数传入

// 方式一： 使用 trait Object
pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}

pub struct CreateTable;

impl Migration for CreateTable {
    fn execute(&self) -> &str {
        "create table"
    }

    fn rollback(&self) -> &str {
        "drop table"
    }
}

pub struct AddField;

impl Migration for AddField {
    fn execute(&self) -> &str {
        "add field"
    }

    fn rollback(&self) -> &str {
        "remove field"
    }
}

struct Schema {
    commands: Vec<Box<dyn Migration>>,
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }
    fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }
    fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }
    fn rollback(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| cmd.rollback())
            .collect()
    }
}

#[test]
fn t_schema() {
    let mut schema = Schema::new();

    let cmd = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}

// ======================================================

// 方式二  使用函数指针
type FnPtr = fn() -> String;

struct Command {
    execute: FnPtr,
    rollback: FnPtr,
}

struct SchemaFnPtr {
    commands: Vec<Command>,
}

impl SchemaFnPtr {
    fn new() -> Self {
        Self { commands: vec![] }
    }
    fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
        self.commands.push(Command { execute, rollback });
    }
    fn execute(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }
    fn rollback(&self) -> Vec<String> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| (cmd.rollback)())
            .collect()
    }
}

fn add_field_2() -> String {
    "add field".to_string()
}

fn remove_field_2() -> String {
    "remove field".to_string()
}

#[test]
fn t_schema_fn_ptr() {
    let mut schema = SchemaFnPtr::new();
    schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
    schema.add_migration(add_field_2, remove_field_2);
    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}

// ======================================================

// 方式三  使用函数特征对象
type MigrationFnTrait<'a> = Box<dyn Fn() -> &'a str>;

struct SchemaFnTrait<'a> {
    executes: Vec<MigrationFnTrait<'a>>,
    rollbacks: Vec<MigrationFnTrait<'a>>,
}

impl<'a> SchemaFnTrait<'a> {
    fn new() -> Self {
        Self {
            executes: vec![],
            rollbacks: vec![],
        }
    }
    // 必须使用两个。不然编译失败
    fn add_migration<E, R>(&mut self, execute: E, rollback: R)
    where
        E: Fn() -> &'a str + 'static,
        R: Fn() -> &'a str + 'static,
    {
        self.executes.push(Box::new(execute));
        self.rollbacks.push(Box::new(rollback));
    }

    fn execute(&self) -> Vec<&str> {
        self.executes.iter().map(|cmd| cmd()).collect()
    }
    fn rollback(&self) -> Vec<&str> {
        self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
    }
}

fn add_field() -> &'static str {
    "add field"
}

fn remove_field() -> &'static str {
    "remove field"
}

#[test]
fn t_migration_fn() {
    let mut schema = SchemaFnTrait::new();
    schema.add_migration(|| "create table", || "drop table");
    schema.add_migration(add_field, remove_field);
    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}
