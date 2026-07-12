use crate::platform::{self, helper::DisplayItem};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};
use ratatui::Terminal;
use std::io;
use std::time::Duration;

/// 启动 TUI 应用，负责终端的初始化与还原
pub fn run() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = app.main_loop(&mut terminal);

    // 无论运行是否出错，都还原终端状态
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

struct App {
    items: Vec<DisplayItem>,
    state: ListState,
    status: String,
    should_quit: bool,
    confirm: bool,
}

impl App {
    fn new() -> Self {
        let items = platform::get_display_items();
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        Self {
            status: format!("📦 共发现 {} 个开机启动项", items.len()),
            items,
            state,
            should_quit: false,
            confirm: false,
        }
    }

    fn main_loop<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<()> {
        while !self.should_quit {
            terminal.draw(|f| self.draw(f))?;
            if !event::poll(Duration::from_millis(250))? {
                continue;
            }
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    self.handle_key(key.code);
                }
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, code: KeyCode) {
        // 确认删除模式下优先处理
        if self.confirm {
            match code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                    self.delete_selected();
                }
                _ => self.status = "已取消删除".to_string(),
            }
            self.confirm = false;
            return;
        }

        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Down | KeyCode::Char('j') => self.next(),
            KeyCode::Up | KeyCode::Char('k') => self.prev(),
            KeyCode::Char('d') | KeyCode::Delete => {
                if self.state.selected().is_some() {
                    self.confirm = true;
                }
            }
            KeyCode::Char('r') => self.refresh(),
            _ => {}
        }
    }

    fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = self.state.selected().unwrap_or(0);
        let i = (i + 1) % self.items.len();
        self.state.select(Some(i));
    }

    fn prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = self.state.selected().unwrap_or(0);
        let i = if i == 0 {
            self.items.len() - 1
        } else {
            i - 1
        };
        self.state.select(Some(i));
    }

    fn refresh(&mut self) {
        self.items = platform::get_display_items();
        if self.items.is_empty() {
            self.state.select(None);
        } else {
            let i = self.state.selected().unwrap_or(0).min(self.items.len() - 1);
            self.state.select(Some(i));
        }
        self.status = format!("🔄 已刷新，共 {} 个启动项", self.items.len());
    }

    fn delete_selected(&mut self) {
        let i = match self.state.selected() {
            Some(i) => i,
            None => return,
        };
        if i >= self.items.len() {
            return;
        }
        let label = self.items[i].label.clone();
        let result = platform::delete_item(&self.items[i].option);
        match result {
            Ok(()) => {
                self.status = format!("✅ 已删除: {}", label);
                self.items.remove(i);
                if self.items.is_empty() {
                    self.state.select(None);
                } else {
                    let i = i.min(self.items.len() - 1);
                    self.state.select(Some(i));
                }
            }
            Err(e) => self.status = format!("❌ 删除失败: {}", e),
        }
    }

    fn draw(&mut self, f: &mut ratatui::Frame) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(f.size());

        // 标题栏
        let header = Paragraph::new("BootWatch 🔍  开机启动项管理")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // 启动项列表
        let list_items: Vec<ListItem> = self
            .items
            .iter()
            .map(|it| {
                let main = Line::from(vec![
                    Span::styled(format!("{} ", it.icon), Style::default()),
                    Span::styled(
                        format!("[{}] ", it.type_label),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(it.label.clone(), Style::default().fg(Color::White)),
                ]);
                let path = it.path.clone().unwrap_or_else(|| "-".to_string());
                let sub = Line::from(Span::styled(
                    format!("    {}", path),
                    Style::default().fg(Color::DarkGray),
                ));
                ListItem::new(vec![main, sub])
            })
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("启动项 ({})", self.items.len())),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");
        f.render_stateful_widget(list, chunks[1], &mut self.state);

        // 状态栏
        let status_color = if self.status.starts_with("❌") {
            Color::Red
        } else if self.status.starts_with("✅") {
            Color::Green
        } else {
            Color::Gray
        };
        let status = Paragraph::new(self.status.as_str()).style(Style::default().fg(status_color));
        f.render_widget(status, chunks[2]);

        // 帮助栏
        let help = " ↑/↓ 或 j/k 移动 · d 删除 · r 刷新 · q 退出 ";
        let help = Paragraph::new(help).style(Style::default().fg(Color::DarkGray));
        f.render_widget(help, chunks[3]);

        if self.confirm {
            self.draw_confirm(f);
        }
    }

    fn draw_confirm(&self, f: &mut ratatui::Frame) {
        let area = centered_rect(50, 7, f.size());
        f.render_widget(Clear, area);
        let i = self.state.selected().unwrap_or(0);
        let label = self.items.get(i).map(|it| it.label.as_str()).unwrap_or("");
        let lines = vec![
            Line::from(Span::styled(
                "⚠  确认删除该启动项？",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                label.to_string(),
                Style::default().fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "y 确认  /  其它键取消",
                Style::default().fg(Color::Gray),
            )),
        ];
        let popup = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("确认"))
            .alignment(Alignment::Center);
        f.render_widget(popup, area);
    }
}

/// 计算居中矩形区域，用于弹窗
fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let v = Layout::vertical([
        Constraint::Length(r.height.saturating_sub(height) / 2),
        Constraint::Length(height),
        Constraint::Min(0),
    ])
    .split(r);
    let h = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(v[1]);
    h[1]
}
