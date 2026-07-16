/// 删除令牌的统一解析：返回 `(kind, payload)`。
///
/// 令牌格式为 `<kind>|<payload>`，其中 payload 可能自身包含 `|`
/// （如文件路径、含 `|` 的名称），因此仅按第一个 `|` 拆分。
/// 若令牌不含 `|`，则 kind 为空字符串、payload 为原值。
pub fn parse_token(value: &str) -> (&str, &str) {
    value.split_once('|').unwrap_or(("", value))
}

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

#[cfg(test)]
mod tests {
    use super::parse_token;

    #[test]
    fn parse_token_kind_and_payload() {
        let (kind, payload) = parse_token("plist|/Users/x/foo.plist");
        assert_eq!(kind, "plist");
        assert_eq!(payload, "/Users/x/foo.plist");
    }

    #[test]
    fn parse_token_payload_may_contain_pipe() {
        // 值名中含 `|` 时仅按首个 `|` 拆分，payload 保留剩余部分
        let (kind, payload) = parse_token("reg|HKCU\\...\\Run|weird|name");
        assert_eq!(kind, "reg");
        assert_eq!(payload, "HKCU\\...\\Run|weird|name");
    }

    #[test]
    fn parse_token_loginitem() {
        let (kind, payload) = parse_token("loginitem|Dropbox");
        assert_eq!(kind, "loginitem");
        assert_eq!(payload, "Dropbox");
    }

    #[test]
    fn parse_token_no_pipe_falls_back() {
        let (kind, payload) = parse_token("no-token-here");
        assert_eq!(kind, "");
        assert_eq!(payload, "no-token-here");
    }

    #[test]
    fn parse_token_empty() {
        let (kind, payload) = parse_token("");
        assert_eq!(kind, "");
        assert_eq!(payload, "");
    }
}
