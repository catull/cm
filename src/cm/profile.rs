use super::*;
use pcre2::bytes::{Regex, RegexBuilder};
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::string::ToString;

pub struct Profile {
    pub regex_list: StringList,
    pub cmd_list: StringList,
    // TODO(#231): there is no UI for customizing Profile::shell similar to how we customize key bindings
    pub shell: PathBuf,
    pub key_map: KeyMap,
}

impl Profile {
    pub fn new() -> Self {
        Self {
            regex_list: StringList::new(),
            cmd_list: StringList::new(),
            shell: PathBuf::new(),
            key_map: KeyMap::new(),
        }
    }

    pub fn from_file(input: Vec<String>, file_path: &Path) -> Self {
        let mut result = Profile::new();
        let mut regex_count = 0;
        let mut cmd_count = 0;
        for (i, line) in input.iter().map(|x| x.trim_start()).enumerate() {
            // TODO(#128): profile parsing errors should be application error messages instead of Rust panics
            let fail = |message| panic!("{}:{}: {}", file_path.display(), i + 1, message);

            if !line.is_empty() {
                let (key, value) = config::split_key_value(line)
                    .unwrap_or_else(|| fail("Invalid configuration line"));

                if key.is_empty() {
                    fail("Key is not provided");
                }

                match key {
                    "regexs" => {
                        regex_count += 1;
                        result.regex_list.list.items.push(value.to_string());
                    }
                    "cmds" => {
                        cmd_count += 1;
                        result.cmd_list.list.items.push(value.to_string());
                    }
                    "shell" => {
                        result.shell.push(value);
                    }
                    "current_regex" => {
                        if value.is_empty() {
                            fail("Value is not provided");
                        }
                        result.regex_list.list.cursor_y =
                            value.parse::<usize>().unwrap_or_else(|_| {
                                fail("Not a number");
                                0
                            })
                    }
                    "current_cmd" => {
                        if value.is_empty() {
                            fail("Value is not provided");
                        }
                        result.cmd_list.list.cursor_y =
                            value.parse::<usize>().unwrap_or_else(|_| {
                                fail("Not a number");
                                0
                            })
                    }
                    key => {
                        if value.is_empty() {
                            fail("Value is not provided");
                        }
                        let key_stroke = KeyStroke::from_str(key).unwrap();
                        let action = action::from_str(value).unwrap();
                        result.key_map.bind(key_stroke, action);
                    }
                }
            }
        }

        // NOTE: regex_count-1 converts value from count to 0-based index
        if result.regex_list.list.cursor_y > regex_count - 1 {
            result.regex_list.list.cursor_y = regex_count - 1;
        }

        // NOTE: cmd_count-1 converts value from count to 0-based index
        if result.cmd_list.list.cursor_y > cmd_count - 1 {
            result.cmd_list.list.cursor_y = cmd_count - 1;
        }

        result
    }

    pub fn to_file<F: io::Write>(&self, stream: &mut F) -> io::Result<()> {
        writeln!(stream, "version = {}", migration::CURRENT_VERSION)?;

        for regex in self.regex_list.list.items.iter() {
            writeln!(stream, "regexs = {}", regex)?;
        }

        for cmd in self.cmd_list.list.items.iter() {
            writeln!(stream, "cmds = {}", cmd)?;
        }

        writeln!(stream, "shell = {}", self.shell.display())?;

        writeln!(stream, "current_regex = {}", self.regex_list.list.cursor_y)?;
        writeln!(stream, "current_cmd = {}", self.cmd_list.list.cursor_y)?;

        self.key_map.to_file(stream)?;

        Ok(())
    }

    pub fn current_regex(&self) -> Option<Result<Regex, pcre2::Error>> {
        match self.regex_list.state {
            StringListState::Navigate => self
                .regex_list
                .current_item()
                .map(|s| RegexBuilder::new().utf(true).ucp(true).build(&s)),
            StringListState::Editing { .. } => Some(
                RegexBuilder::new()
                    .utf(true)
                    .ucp(true)
                    .build(&self.regex_list.edit_field.buffer),
            ),
        }
    }

    pub fn current_cmd(&self) -> Option<String> {
        match self.cmd_list.state {
            StringListState::Navigate => self.cmd_list.current_item().map(String::from),
            StringListState::Editing { .. } => Some(self.cmd_list.edit_field.buffer.clone()),
        }
    }

    pub fn initial() -> Self {
        let mut result = Self::new();
        result
            .regex_list
            .list
            .items
            .push(r"(\/?\b.*?):(\d+):".to_string());
        result.cmd_list.list.items.push("vim +\\2 \\1".to_string());
        result
            .cmd_list
            .list
            .items
            .push("emacs -nw +\\2 \\1".to_string());
        result.shell.push("/bin/sh");
        result.key_map = KeyMap::initial();
        result
    }
}
