use std::process::ExitCode;

pub mod args;

#[derive(Copy, Clone)]
pub enum ExitStatus {
    Success,

    Failure,

    Error,
}

impl From<ExitStatus> for ExitCode {
    fn from(status: ExitStatus) -> Self {
        match status {
            // 成功
            ExitStatus::Success => ExitCode::from(0),
            // 一般的なエラー全般
            ExitStatus::Failure => ExitCode::from(1),
            // コマンドの引数やオプションが不正であるといったケース
            ExitStatus::Error => ExitCode::from(2),
        }
    }
}
