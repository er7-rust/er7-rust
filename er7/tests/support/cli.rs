//! Shared black-box checks for the two workspace binaries.
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

fn with_stdin(command: &mut Command, input: &[u8]) -> Output {
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the command builds");
    let mut pipe = child.stdin.take().expect("stdin is piped");
    // A run that fails while reading its arguments exits before it reads
    // standard input at all, which closes this pipe under us. That is the
    // command behaving correctly, not the test failing, so a broken pipe
    // here is expected rather than fatal.
    match pipe.write_all(input) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {}
        Err(e) => panic!("writes to stdin: {e}"),
    }
    drop(pipe); // the child needs the EOF before it will finish
    child.wait_with_output().expect("the command finishes")
}

pub fn cli(binary: &str, args: &[&str], stdin: &str) -> (bool, String, String) {
    let output = with_stdin(Command::new(binary).args(args), stdin.as_bytes());
    (
        output.status.success(),
        String::from_utf8(output.stdout).expect("stdout is UTF-8"),
        String::from_utf8(output.stderr).expect("stderr is UTF-8"),
    )
}

/// Each test owns its directory, including when several toolchains run concurrently.
pub struct TestDirectory(PathBuf);

impl TestDirectory {
    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn new() -> Self {
        let base = std::env::temp_dir();
        let mut suffix = 0;
        loop {
            let path = base.join(format!("er7-cli-{}-{suffix}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => suffix += 1,
                Err(error) => panic!("creating test directory {}: {error}", path.display()),
            }
        }
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

pub fn check_utf8_io(binary: &str, args: &[&str], expected_segment: &str) {
    let directory = TestDirectory::new();
    let source = directory.path().join("読み込み α.er7");
    let destination = directory.path().join("écriture β.er7");
    let header = r"MSH|^~\&|LAB";
    let original = format!("{header}\nZTX|示例^例文\n");
    fs::write(&source, &original).unwrap();
    for (option, separator) in [("cr", "\r"), ("lf", "\n"), ("crlf", "\r\n")] {
        let expected = format!("{header}{separator}{expected_segment}{separator}");
        let command = || {
            let mut cmd = Command::new(binary);
            cmd.args(args).args(["--terminator", option]);
            cmd
        };
        let from_file = command().arg(&source).output().unwrap();
        assert!(from_file.status.success(), "{from_file:?}");
        assert_eq!(from_file.stdout, expected.as_bytes());

        let from_pipe = with_stdin(
            command().arg("--output").arg(&destination),
            original.as_bytes(),
        );
        assert!(from_pipe.status.success(), "{from_pipe:?}");
        assert_eq!(from_pipe.stdout, b"");
        assert_eq!(fs::read(&destination).unwrap(), expected.as_bytes());
    }

    let invalid = [original.as_bytes(), &[0xff]].concat();
    fs::write(&source, &invalid).unwrap();
    fs::write(&destination, b"existing output").unwrap();
    for piped in [false, true] {
        let mut command = Command::new(binary);
        command.args(args).arg("--output").arg(&destination);
        let failure = if piped {
            with_stdin(&mut command, &invalid)
        } else {
            command.arg(&source).output().unwrap()
        };
        assert_eq!(failure.status.code(), Some(1), "{failure:?}");
        assert!(
            String::from_utf8_lossy(&failure.stderr).contains("reading"),
            "{failure:?}"
        );
        assert_eq!(failure.stdout, b"");
        assert_eq!(fs::read(&destination).unwrap(), b"existing output");
    }
}
