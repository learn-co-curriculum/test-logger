use std::{env, fs::OpenOptions};

use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Whether test passed or failed (either 'p' or 'f')
    #[arg(short, default_value_t = 'p')]
    result_type: char,
    /// Message to log
    #[arg(short, default_value = "")]
    message: String,
    /// Optional explanation
    #[arg(short, default_value = "")]
    explanation: String,
    /// Whether to output the log to console or not
    #[arg(short)]
    output: bool,
}

fn main() {
    let args = Args::parse();

    if let Ok(workspace) = env::var("GITHUB_WORKSPACE") {
        let dir = workspace.to_owned() + "/test/results.csv";
        let file_exists = std::path::Path::new(&dir).exists();
        let results_file = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .open(dir)
            .unwrap();

        if args.output {
            let total_records = csv::Reader::from_reader(results_file).records().count();

            // File consumed counting, so re-open
            let results_file = OpenOptions::new()
                .write(true)
                .read(true)
                .append(true)
                .open(workspace.to_owned() + "/test/results.csv")
                .unwrap();
            let mut reader = csv::Reader::from_reader(&results_file);
            for (idx, record) in reader.records().enumerate() {
                let record = record.unwrap();

                let mut fields = record.iter();
                let result_type = fields.next().unwrap();
                let message = fields.next().unwrap();
                let explanation = fields.next().unwrap();

                let mut message = format!("[{idx}/{total_records}] {result_type} | {message}");
                if !explanation.is_empty() {
                    message.push_str("\n    ");
                    message += explanation;
                }

                if result_type == "PASS" {
                    println!("{}", message.green());
                } else {
                    println!("{}", message.red());
                }
            }
        } else {
            let mut writer = csv::Writer::from_writer(results_file);

            if !file_exists {
                writer
                    .write_record(&["Result", "Message", "Explanation"])
                    .unwrap();
            }

            let result = if args.result_type == 'p' {
                "PASS"
            } else {
                "FAIL"
            };

            writer
                .write_record(&[result, &args.message, &args.explanation])
                .unwrap();
        }
    }
}