use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use csv::{Reader, Writer};
use std::fs::OpenOptions;

#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 新しい口座を作る
    New(NewArgs),
    /// 口座に入金する
    Deposit(DepositArgs),
    /// 口座から出金する
    Withdraw(WithdrawArgs),
    /// CSV からインポートする
    Import(ImportArgs),
    /// レポートを出力する
    Report,
}

#[derive(Args)]
struct NewArgs {
    account_name: String,
}
impl NewArgs {
    fn run(&self) {
        let file_name = format!("{}.csv", self.account_name);
        let mut writer = Writer::from_path(file_name).unwrap();
        writer.write_record(["日付", "用途", "金額"]).unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Args)]
struct DepositArgs {
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}
impl DepositArgs {
    fn run(&self) {
        // 追記モードでファイルを開く
        let open_option = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}.csv", self.account_name))
            .unwrap();

        // ファイルに書き込む
        let mut writer = Writer::from_writer(open_option);
        writer
            .write_record([
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                self.amount.to_string(),
            ])
            .unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Args)]
struct WithdrawArgs {
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}
impl WithdrawArgs {
    fn run(&self) {
        // 追記モードでファイルを開く
        let open_option = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}.csv", self.account_name))
            .unwrap();

        // ファイルに書き込む
        let mut writer = Writer::from_writer(open_option);
        writer
            .write_record([
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                // MEMO: depositとの差分はここだけ
                format!("-{}", self.amount),
            ])
            .unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Args)]
struct ImportArgs {
    src_file_name: String,
    dst_account_name: String,
}

impl ImportArgs {
    fn run(&self) {
        let open_option = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}.csv", self.dst_account_name))
            .unwrap();
        let mut writer = Writer::from_writer(open_option);
        let mut reader = Reader::from_path(&self.src_file_name).unwrap();
        for result in reader.records() {
            // Readerは先頭行をヘッダーとして読み込むので、2行目以降について実行される
            let record = result.unwrap();
            writer.write_record(&record).unwrap();
        }
        writer.flush().unwrap();
    }
}

fn main() {
    let args = App::parse();
    match args.command {
        Commands::New(args) => args.run(),
        Commands::Deposit(args) => args.run(),
        Commands::Withdraw(args) => args.run(),
        Commands::Import(args) => args.run(),
        Commands::Report => unimplemented!(),
    }
}
