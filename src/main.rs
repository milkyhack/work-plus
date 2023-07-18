use std::io;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, Duration};
use chrono::{Local, DateTime};

fn create_log_file(executor: &str, identifier: &str, department: &str, description: &str, start_time: &str, end_time: &str) {
    let filename = format!("log_{}.txt", identifier);
    let mut file = File::create(filename).expect("Не удалось создать файл");

    writeln!(file, "Исполнитель: {}", executor).expect("Не удалось записать в файл");
    writeln!(file, "Идентификатор: {}", identifier).expect("Не удалось записать в файл");
    writeln!(file, "Отдел: {}", department).expect("Не удалось записать в файл");
    writeln!(file, "Описание действий: {}", description).expect("Не удалось записать в файл");
    writeln!(file, "Начало работ: {}", start_time).expect("Не удалось записать в файл");
    writeln!(file, "Конец работ: {}", end_time).expect("Не удалось записать в файл");
}

fn main() {
    let mut executor = String::new();
    let mut identifier = String::new();
    let mut department = String::new();
    let mut description = String::new();

    println!("Введите исполнителя: ");
    io::stdin().read_line(&mut executor).expect("Ошибка при чтении ввода");
    let executor = executor.trim();

    println!("Введите идентификатор: ");
    io::stdin().read_line(&mut identifier).expect("Ошибка при чтении ввода");
    let identifier = identifier.trim();

    println!("Введите отдел: ");
    io::stdin().read_line(&mut department).expect("Ошибка при чтении ввода");
    let department = department.trim();

    println!("Введите описание действий: ");
    io::stdin().read_line(&mut description).expect("Ошибка при чтении ввода");
    let description = description.trim();

    let start_time = Local::now();
    println!("Работа начата: {}", start_time.format("%Y-%m-%d %H:%M:%S"));

    println!("Нажмите Enter для остановки работы...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка при чтении ввода");

    let end_time = Local::now();
    println!("Работа завершена: {}", end_time.format("%Y-%m-%d %H:%M:%S"));

    let duration = end_time.signed_duration_since(start_time);
    let minutes = duration.num_minutes();

    create_log_file(
        executor,
        identifier,
        department,
        description,
        &start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        &end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
    );
    println!("Лог-файл успешно создан.");

    println!("Продолжительность работы: {} минут", minutes);
}

