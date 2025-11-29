use std::io::Write;

use std::process::ExitCode;

use anyhow::Context;
use clap::Parser;
use colored::Colorize;
use sqltool::{ExitStatus, args::Args};

pub fn main() -> ExitCode {
    // std::env の args()やargs_os()を使うと、「シェル/OSから渡された文字列」をそのまま受け取るだけになる
    // Unix系シェルではコマンドラインで`*.rs`のようなglob(ワイルドカード)があれば、シェル側がファイル名
    // 一覧に展開してかプログラムに渡す。一方でWindowsの`cmd.exe`はデフォルトではglob展開しない
    // wildクレートは、Windowsを含めどのプラットフォームではglob展開した形で引数を取得できるようにするモノである。
    // 将来的なWindows対応も見越して、wildを導入する
    let args = wild::args_os();
    let args = match argfile::expand_args_from(
        args,
        argfile::parse_fromfile,
        argfile::PREFIX,
    )
    .context("Failed to read CLI arguments from files")
    {
        Ok(args) => args,
        Err(error) => return report_error(&error),
    };

    let args = Args::parse_from(args);

    dbg!(args);

    ExitStatus::Success.into()
}

fn report_error(error: &anyhow::Error) -> ExitCode {
    {
        // FYI: Broken Pipe Error の場合は正常終了として扱う
        //
        // Broken Pipe は、パイプの読み手が先に終了した時に書き手が発生する
        // I/Oエラーである. Unix/LinuxではSIGPIPEとしても知られる
        //
        // See:
        // - https://qiita.com/imishinist/items/282430dbbdf9548246e9
        // - https://github.com/BurntSushi/ripgrep/blob/bf63fe8f258afc09bae6caa48f0ae35eaf115005/crates/core/main.rs#L47C1-L61C14
        for cause in error.chain() {
            if let Some(io_error) = cause.downcast_ref::<std::io::Error>()
            {
                if io_error.kind() == std::io::ErrorKind::BrokenPipe {
                    return ExitStatus::Success.into();
                }
            }
        }

        // std::io::stderr().lock()は、標準エラー出力への排他的なアクセスを取得する。
        // ロックが取得されている間は、他のスレッドからの書き込みは待機され、出力が混ざらない。
        // 複数のスレッドから同時にstderrに書き込んでも、出力が混ざらないようにします。
        let mut stderr = std::io::stderr().lock();

        // writeln!マクロは、eprintln!と異なり、書き込みエラーをResultとして返す。
        // .ok()でエラーを無視することで、stderrが閉じられている場合でもパニックせずに処理を続行できる。
        writeln!(stderr, "{}", "sqltool failed".red().bold()).ok();

        for cause in error.chain() {
            writeln!(stderr, "  {} {cause}", "Cause:".bold()).ok();
        }
    }

    ExitStatus::Error.into()
}
