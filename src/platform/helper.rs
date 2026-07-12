#[derive(Debug, Clone)]
pub struct OptionItem {
    pub label: String,
    pub value: String,
}

/// 用于 TUI 展示的启动项信息，内嵌用于删除的 OptionItem
#[derive(Debug, Clone)]
pub struct DisplayItem {
    pub icon: String,
    pub type_label: String,
    pub label: String,
    pub path: Option<String>,
    pub option: OptionItem,
}
