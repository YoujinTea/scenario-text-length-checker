use std::env::current_dir;
use std::fs::{self, canonicalize, File};
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use serde_json;
use regex::Regex;

#[derive(Serialize, Deserialize)]
struct Settings {
    linefeed_tag: Vec<String>,
    page_break_tag: Vec<String>,
    line_count: usize,
    max_row_length: usize,
}

fn get_settings() -> Settings {
    let settings_path = Path::new("settings.json");

    // settings.jsonが存在しない場合は作成する
    if !settings_path.exists() {
        let settings = Settings {
            linefeed_tag: vec!["[r]".to_string()],
            page_break_tag: vec!["[p]".to_string()],
            line_count: 2,
            max_row_length: 30,
        };
        let file = File::create(settings_path).unwrap();
        serde_json::to_writer_pretty(file, &settings).unwrap();
        return settings;
    }

    // settings.jsonが存在する場合は読み込む
    let file = File::open(settings_path).unwrap();
    serde_json::from_reader(file).unwrap()
}

fn search_ks_files(directory: &Path) -> Vec<PathBuf> {
    let mut ks_files = Vec::new();

    if directory.is_dir() {
        for entry in fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            // ディレクトリの場合は再帰的に探索
            if path.is_dir() {
                ks_files.extend(search_ks_files(&path));
            }

            // .ksファイルの場合はリストに追加
            if path.extension().map(|e| e == "ks").unwrap_or(false) {
                ks_files.push(path);
            }
        }
    }

    ks_files
}

fn check_statement_length(statement: &Vec<String>, settings: &Settings) -> bool {
    let mut current_line = 0;

    for s in statement {
        let mut length: i32 = s.chars().count() as i32;
        while length > 0 {
            length -= settings.max_row_length as i32;
            current_line += 1;
        }
    }

    current_line > settings.line_count
}

fn check_text_length(ks_file: &Path, display_path: &Path, settings: &Settings) {
    let file = File::open(ks_file).unwrap();
    let lines: Vec<String> = io::BufReader::new(file).lines().filter_map(Result::ok).collect();

    let mut before_statement = vec!["".to_string()];

    let mut is_script = false;

    for (i, line) in lines.iter().enumerate() {
        let mut line = line.trim().to_string();

        // コメント行はスキップ
        if line.starts_with(';') {
            continue;
        }

        // ラベル行はスキップ
        if line.starts_with('*') {
            continue;
        }

        // # で始まる行はスキップ
        if line.starts_with('#') {
            continue;
        }

        // iscriptタグがあればスキップ
        if line.contains("[iscript]") {
            is_script = true;
            continue;
        }

        // iscriptタグが閉じられるまでスキップ
        if is_script {
            if line.contains("[endscript]") {
                is_script = false;
            }
            continue;
        }

        let mut statements = vec![line.clone()];

        // 改ページタグで区切る
        for tag in &settings.page_break_tag {
            statements = statements.iter().flat_map(|s| s.split(tag)).map(|s| s.to_string()).collect();
        }

        // split_line_callback処理
        let split_line_callback = |statement: &str| -> Vec<String> {
            let mut statement_tmp = vec![statement.trim().to_string()];

            // 改行タグで区切る
            for tag in &settings.linefeed_tag {
                statement_tmp = statement_tmp.iter().flat_map(|s| s.split(tag)).map(|s| s.to_string()).collect();
            }

            // 文章の中のタグを削除
            let re = Regex::new(r"\[.*?\]").unwrap();
            statement_tmp.iter_mut().for_each(|s| *s = re.replace_all(s, "").to_string());

            statement_tmp
        };

        let mut statements: Vec<Vec<String>> = statements.into_iter().map(|s| split_line_callback(&s)).collect();

        // 空行はスキップ
        if statements.is_empty() {
            continue;
        }

        // 前行の文章を結合
        let last_index = statements[0].len() - 1;
        let last_statement = statements[0][last_index].clone();
        statements[0][last_index] = before_statement[before_statement.len() - 1].clone() + &last_statement;

        // 行数が設定値を超えているかチェック
        for statement in &statements[0..statements.len() - 1] {
            if check_statement_length(statement, settings) {
                println!("{} {}行: 文章が長すぎます。", display_path.display(), i + 1);
            }
        }

        // 最後の文章を次の行に引き継ぐ
        before_statement = statements[statements.len() - 1].clone();
    }
}

fn main() {
    let path = current_dir().unwrap();

    if path.parent().unwrap().file_name().unwrap() != "others" {
        println!("Error: このアプリケーションはothersディレクトリに配置してください。");
        println!("終了するにはEnterキーを押してください。");
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }

    let scenario_dir = path.parent().unwrap().parent().unwrap().join("scenario");

    if !scenario_dir.exists() {
        println!("Error: scenarioディレクトリが存在しません。");
        println!("終了するにはEnterキーを押してください。");
        io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }

    let settings = get_settings();
    let ks_files = search_ks_files(&path.parent().unwrap().parent().unwrap().join("scenario"));

    for ks_file in ks_files {
        check_text_length(&ks_file, &ks_file.strip_prefix(&scenario_dir).unwrap(), &settings);
    }

    println!("終了するにはEnterキーを押してください。");
    io::stdin().read_line(&mut String::new()).unwrap();
}
