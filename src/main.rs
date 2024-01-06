use kakeibo_app::services;
use std::io;

const FILE_PATH: &str = "store/data.json";
fn main() {
    // 標準入力した値を格納する変数の生成
    let mut service_type = String::new();
    println!("実行したい内容を入力してください(0:登録、1:集計)");
    // 標準入力された文字列の読み込み
    io::stdin().read_line(&mut service_type).unwrap();
    // 文字列から改行コード、スペース等の除去、文字列から数値への変換
    let service_type: u8 = service_type.trim().parse().expect("数値で入力してください");
    // 入力値のバリデーション
    services::validate::InputValidator::validate_service_type(service_type);
    if service_type == 0 {
        services::register::run(FILE_PATH)
    } else if service_type == 1 {
        services::summarize::run(FILE_PATH);
    }
}
