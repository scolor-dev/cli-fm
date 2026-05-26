use serde::Deserialize;

#[derive(Deserialize)]
pub struct KeymapConfig {
    pub normal: NormalKeymap,
    #[serde(default)]
    pub input: InputKeymap,
    #[serde(default)]
    pub confirm: ConfirmKeymap,
}

#[derive(Deserialize)]
pub struct NormalKeymap {
    pub up: Vec<String>,
    pub down: Vec<String>,
    pub enter: Vec<String>,
    pub back: Vec<String>,
    pub quit: Vec<String>,
    pub command_mode: Vec<String>,
    pub search_mode: Vec<String>,
    pub path_mode: Vec<String>,
    #[serde(default = "default_copy")]
    pub copy: Vec<String>,
    #[serde(default = "default_cut")]
    pub cut: Vec<String>,
    #[serde(default = "default_paste")]
    pub paste: Vec<String>,
    #[serde(default = "default_delete")]
    pub delete: Vec<String>,
    #[serde(default = "default_rename")]
    pub rename: Vec<String>,
    #[serde(default = "default_new_entry")]
    pub new_entry: Vec<String>,
    #[serde(default = "default_undo")]
    pub undo: Vec<String>,
    #[serde(default = "default_redo")]
    pub redo: Vec<String>,
}

#[derive(Deserialize)]
pub struct InputKeymap {
    #[serde(default = "default_submit")]
    pub submit: Vec<String>,
    #[serde(default = "default_cancel")]
    pub cancel: Vec<String>,
    #[serde(default = "default_backspace")]
    pub backspace: Vec<String>,
    #[serde(default = "default_tab_complete")]
    pub tab_complete: Vec<String>,
}

#[derive(Deserialize)]
pub struct ConfirmKeymap {
    #[serde(default = "default_yes")]
    pub yes: Vec<String>,
    #[serde(default = "default_no")]
    pub no: Vec<String>,
}

impl Default for InputKeymap {
    fn default() -> Self {
        Self {
            submit: default_submit(),
            cancel: default_cancel(),
            backspace: default_backspace(),
            tab_complete: default_tab_complete(),
        }
    }
}

impl Default for ConfirmKeymap {
    fn default() -> Self {
        Self { yes: default_yes(), no: default_no() }
    }
}

fn default_copy() -> Vec<String> { vec!["c".to_string()] }
fn default_cut() -> Vec<String> { vec!["m".to_string()] }
fn default_paste() -> Vec<String> { vec!["p".to_string()] }
fn default_delete() -> Vec<String> { vec!["d".to_string()] }
fn default_rename() -> Vec<String> { vec!["r".to_string()] }
fn default_new_entry() -> Vec<String> { vec!["a".to_string()] }
fn default_undo() -> Vec<String> { vec!["Z".to_string()] }
fn default_redo() -> Vec<String> { vec!["Y".to_string()] }
fn default_submit() -> Vec<String> { vec!["enter".to_string()] }
fn default_cancel() -> Vec<String> { vec!["escape".to_string()] }
fn default_backspace() -> Vec<String> { vec!["backspace".to_string()] }
fn default_tab_complete() -> Vec<String> { vec!["tab".to_string()] }
fn default_yes() -> Vec<String> { vec!["y".to_string()] }
fn default_no() -> Vec<String> { vec!["n".to_string(), "escape".to_string()] }
