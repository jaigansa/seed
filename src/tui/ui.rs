use super::{App, MarkdownEditorModal, MarkdownModalMode, ProjectCategory};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Wrap,
    },
    Frame,
};
use std::path::Path;

pub fn render(f: &mut Frame, app: &mut App) {
    let area = f.area();
    if area.width < 75 || area.height < 20 {
        render_size_warning(f, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header & Quick Action Bar
            Constraint::Min(10),   // Unified Workspace Dashboard
            Constraint::Length(2), // Bottom Status & Keybindings
        ])
        .split(area);

    render_header(f, app, chunks[0]);

    // Primary View: Always render the Workspace Dashboard as the base view
    render_view_projects_tab(f, app, chunks[1]);

    render_footer(f, app, chunks[2]);

    // Centered Modal Overlays for creation and settings
    match app.current_tab {
        1 => render_create_modal(f, app, area),
        2 | 3 => render_config_modal(f, app, area),
        _ => {}
    }

    // Modals render over everything else in priority order
    if let Some(ref modal) = app.md_editor_modal {
        render_markdown_editor_modal(f, modal);
    }

    if app.confirm_discard_md {
        render_confirm_discard_modal(f);
    }

    if let Some((ref name, ref path)) = app.confirm_archive {
        render_confirm_archive_modal(f, name, path);
    }

    if let Some((ref name, ref report)) = app.doctor_popup {
        render_doctor_modal(f, name, report);
    }

    if let Some((ref name, ref lines, scroll)) = app.tree_popup {
        render_tree_modal(f, name, lines, scroll);
    }

    if let Some(ref form) = app.manual_template_modal {
        render_manual_template_modal(f, form);
    }

    if app.show_help_modal {
        render_help_modal(f);
    }

    if let Some(ref popup) = app.popup_message {
        render_popup(f, popup);
    }
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(18), Constraint::Min(20)])
        .split(area);

    let logo = Paragraph::new(Line::from(vec![
        Span::styled(
            " ◉ SEED ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" v0.2.0", Style::default().fg(Color::DarkGray)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Color::Green)),
    );
    f.render_widget(logo, header_chunks[0]);

    let top_bar = Paragraph::new(Line::from(vec![
        Span::styled(" 🗁  Workspace: ", Style::default().fg(Color::DarkGray)),
        Span::styled(&app.config.workspace_root, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" ◈ Workspace Dashboard "),
    );

    f.render_widget(top_bar, header_chunks[1]);
}

fn render_create_modal(f: &mut Frame, app: &mut App, screen_area: Rect) {
    let area = centered_rect(90, 88, screen_area);
    f.render_widget(Clear, area);

    let modal_block = Block::default()
        .title(" ◈ New Project ")
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::Green));
    let inner_area = modal_block.inner(area);
    f.render_widget(modal_block, area);

    render_create_tab(f, app, inner_area);
}

fn render_config_modal(f: &mut Frame, app: &mut App, screen_area: Rect) {
    let area = centered_rect(78, 82, screen_area);
    f.render_widget(Clear, area);

    let modal_block = Block::default()
        .title(" ◈ Workspace Configuration ")
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::Yellow));
    let inner_area = modal_block.inner(area);
    f.render_widget(modal_block, area);

    render_config_tab(f, app, inner_area);
}

// ==========================================
// TAB 0: CREATE NEW PROJECT
// ==========================================
fn render_create_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(34), Constraint::Percentage(66)])
        .split(area);

    let use_nf = app.config.use_nerd_font();
    let preset_count = app.templates.iter().filter(|t| t.is_preset()).count();
    let manual_count = app.templates.iter().filter(|t| t.is_manual()).count();
    let filter_label = match app.create_template_filter_idx {
        1 => format!("Preset ({})", preset_count),
        2 => format!("Manual ({})", manual_count),
        _ => format!("All ({})", app.templates.len()),
    };

    let filtered = app.filtered_templates_for_create();
    let template_items: Vec<ListItem> = if filtered.is_empty() {
        vec![
            ListItem::new(Line::from(vec![
                Span::styled(" No templates match filter.", Style::default().fg(Color::DarkGray)),
            ])),
            ListItem::new(Line::from(vec![
                Span::styled(" Press [←/→] to switch.", Style::default().fg(Color::DarkGray)),
            ])),
        ]
    } else {
        filtered
            .iter()
            .map(|&(i, t)| {
                let is_sel = i == app.create_template_idx;
                let is_fav = app.config.is_favorite(&t.name);
                let mut spans = Vec::new();
                let badge = if t.is_manual() { "[M] " } else { "[P] " };
                let badge_color = if t.is_manual() { Color::Magenta } else { Color::Cyan };
                spans.push(Span::styled(badge, Style::default().fg(badge_color)));
                spans.push(Span::styled(
                    format!("{:<13} ", t.name),
                    if is_sel {
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                    } else if is_fav {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default().fg(Color::White)
                    },
                ));
                if is_fav {
                    spans.push(Span::styled(
                        crate::icons::fav_badge(true, use_nf).trim(),
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ));
                }
                ListItem::new(Line::from(spans))
            })
            .collect()
    };

    let mut state = ListState::default();
    if !filtered.is_empty() {
        let sel_pos = filtered.iter().position(|&(i, _)| i == app.create_template_idx).unwrap_or(0);
        state.select(Some(sel_pos));
    }

    let is_tpl_focused = app.create_pane == 0;
    let is_form_focused = app.create_pane == 1;

    let template_block = Block::default()
        .title(if is_tpl_focused {
            format!(" ▶ 1. Templates [{}] ", filter_label)
        } else {
            format!("   1. Templates [{}] ", filter_label)
        })
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(if is_tpl_focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        });

    let template_list = List::new(template_items)
        .block(template_block)
        .highlight_style(
            Style::default()
                .bg(if is_tpl_focused { Color::Rgb(25, 55, 40) } else { Color::Rgb(20, 30, 25) })
                .fg(if is_tpl_focused { Color::Green } else { Color::White })
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(if is_tpl_focused { "▶ " } else { "> " });
    f.render_stateful_widget(template_list, chunks[0], &mut state);

    let is_personal = app.create_category_idx == 0;
    let is_client = app.create_category_idx == 1;
    let is_resource = app.create_category_idx == 2;

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(if is_client { 11 } else { 10 }), // Project Details Form
            Constraint::Min(6),                                  // Expansive Structure Preview
        ])
        .split(chunks[1]);

    let licenses = ["MIT", "Apache-2.0", "BSD-3-Clause", "Unlicense", "None"];

    let author_idx = if is_client { 3 } else { 2 };
    let lic_idx = if is_client { 4 } else { 3 };
    let git_idx = if is_client { 5 } else { 4 };
    let btn_idx = if is_client { 6 } else { 5 };

    let is_editing_title = app.create_field_idx == 1 && app.is_input_editing;
    let is_editing_client = app.create_field_idx == 2 && app.is_input_editing && is_client;
    let is_editing_author = app.create_field_idx == author_idx && app.is_input_editing;

    let mut form_lines = vec![
        Line::from(vec![
            Span::styled(
                if app.create_field_idx == 0 { "▶ Target:   " } else { "  Target:   " },
                if app.create_field_idx == 0 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            if is_personal {
                Span::styled("[Personal]", Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled(" Personal ", Style::default().fg(Color::DarkGray))
            },
            Span::raw(" "),
            if is_client {
                Span::styled("[Client]", Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD))
            } else {
                Span::styled(" Client ", Style::default().fg(Color::DarkGray))
            },
            Span::raw(" "),
            if is_resource {
                Span::styled("[Resource]", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD))
            } else {
                Span::styled(" Resource ", Style::default().fg(Color::DarkGray))
            },
        ]),
        Line::from(vec![
            Span::styled(
                if app.create_field_idx == 1 { "▶ Title:    " } else { "  Title:    " },
                if app.create_field_idx == 1 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            Span::styled(
                if is_editing_title { format!("{}_", app.create_project_name) } else { app.create_project_name.clone() },
                if is_editing_title {
                    Style::default().fg(Color::Yellow).bg(Color::Rgb(40, 50, 70)).add_modifier(Modifier::BOLD)
                } else if app.create_field_idx == 1 {
                    Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::UNDERLINED)
                } else {
                    Style::default().fg(Color::White)
                },
            ),
            if is_editing_title {
                Span::styled(" ● [EDITING]", Style::default().fg(Color::Yellow))
            } else {
                Span::raw("")
            },
        ]),
    ];

    if is_client {
        form_lines.push(Line::from(vec![
            Span::styled(
                if app.create_field_idx == 2 { "▶ Client:   " } else { "  Client:   " },
                if app.create_field_idx == 2 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            Span::styled(
                if is_editing_client { format!("{}_", app.create_client_name) } else { app.create_client_name.clone() },
                if is_editing_client {
                    Style::default().fg(Color::Yellow).bg(Color::Rgb(40, 50, 70)).add_modifier(Modifier::BOLD)
                } else if app.create_field_idx == 2 {
                    Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::UNDERLINED)
                } else {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                },
            ),
            if is_editing_client {
                Span::styled(" ● [EDITING]", Style::default().fg(Color::Yellow))
            } else {
                Span::raw("")
            },
        ]));
    }

    form_lines.push(Line::from(vec![
        Span::styled(
            if app.create_field_idx == author_idx { "▶ Author:   " } else { "  Author:   " },
            if app.create_field_idx == author_idx { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
        ),
        Span::styled(
            if is_editing_author { format!("{}_", app.create_author) } else { app.create_author.clone() },
            if is_editing_author {
                Style::default().fg(Color::Yellow).bg(Color::Rgb(40, 50, 70)).add_modifier(Modifier::BOLD)
            } else if app.create_field_idx == author_idx {
                Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::UNDERLINED)
            } else {
                Style::default().fg(Color::White)
            },
        ),
        if is_editing_author {
            Span::styled(" ● [EDITING]", Style::default().fg(Color::Yellow))
        } else {
            Span::raw("")
        },
    ]));

    form_lines.push(Line::from(vec![
        Span::styled(
            if app.create_field_idx == lic_idx { "▶ License:  " } else { "  License:  " },
            if app.create_field_idx == lic_idx { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
        ),
        Span::styled(
            format!("< {} >", licenses[app.create_license_idx % licenses.len()]),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ),
    ]));

    form_lines.push(Line::from(vec![
        Span::styled(
            if app.create_field_idx == git_idx { "▶ Git Init: " } else { "  Git Init: " },
            if app.create_field_idx == git_idx { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
        ),
        Span::styled(
            if app.create_init_git { "[X] Initialize Git Repo" } else { "[ ] No Git" },
            Style::default().fg(if app.create_init_git { Color::Green } else { Color::DarkGray }).add_modifier(Modifier::BOLD),
        ),
    ]));

    form_lines.push(Line::from(""));
    form_lines.push(Line::from(vec![
        Span::styled(
            if app.create_field_idx == btn_idx { "▶ [ CREATE & SEED PROJECT NOW ]" } else { "  [ CREATE & SEED PROJECT NOW ]" },
            if app.create_field_idx == btn_idx {
                Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            },
        ),
    ]));

    let form_block = Block::default()
        .title(if is_form_focused {
            " ▶ 2. Project Details "
        } else {
            "   2. Project Details "
        })
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(if is_form_focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        });

    let form_box = Paragraph::new(form_lines).block(form_block);
    f.render_widget(form_box, right_chunks[0]);

    let active_tpl = if filtered.is_empty() {
        None
    } else if let Some(&(_, t)) = filtered.iter().find(|&&(idx, _)| idx == app.create_template_idx) {
        Some(t)
    } else {
        filtered.first().map(|&(_, t)| t)
    };

    if let Some(tpl) = active_tpl {
        let is_fav = app.config.is_favorite(&tpl.name);
        let mut tpl_spans = vec![
            Span::styled("Template: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&tpl.name, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            if tpl.is_manual() {
                Span::styled("[Manual]", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("[Preset]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            },
        ];
        if is_fav {
            tpl_spans.push(Span::raw(" "));
            tpl_spans.push(Span::styled(
                crate::icons::fav_badge(true, use_nf).trim(),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ));
        }
        tpl_spans.push(Span::styled(format!(" - {}", tpl.description), Style::default().fg(Color::White)));

        let path_display = if let Ok(home) = std::env::var("HOME") {
            if app.create_custom_path.starts_with(&home) {
                format!("~{}", &app.create_custom_path[home.len()..])
            } else {
                app.create_custom_path.clone()
            }
        } else {
            app.create_custom_path.clone()
        };

        let mut preview_lines = vec![
            Line::from(vec![
                Span::styled("Project Path: ", Style::default().fg(Color::DarkGray)),
                Span::styled(path_display, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(tpl_spans),
        ];

        if !tpl.outputs.is_empty() {
            preview_lines.push(Line::from(vec![
                Span::styled("Deliverables: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(tpl.outputs.join(" • "), Style::default().fg(Color::Cyan)),
            ]));
        }

        preview_lines.push(Line::from(""));
        preview_lines.push(Line::from(Span::styled("Structure to be Created:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));

        for d in &tpl.directories {
            preview_lines.push(Line::from(vec![
                Span::styled("  + ", Style::default().fg(Color::Blue)),
                Span::raw(format!("{}/", d)),
            ]));
        }

        for file in &tpl.files {
            preview_lines.push(Line::from(vec![
                Span::styled("  - ", Style::default().fg(Color::Green)),
                Span::raw(&file.path),
                if file.required {
                    Span::styled(" [req]", Style::default().fg(Color::Yellow))
                } else {
                    Span::styled(" [opt]", Style::default().fg(Color::DarkGray))
                },
            ]));
        }

        let preview_box = Paragraph::new(preview_lines)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title(" 3. Directory & File Structure Preview ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain),
            );
        f.render_widget(preview_box, right_chunks[1]);
    } else {
        let empty_preview = Paragraph::new("No templates match the active category filter.\nPress [←/→] to browse All or Preset templates.")
            .block(
                Block::default()
                    .title(" 3. Directory & File Structure Preview ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );
        f.render_widget(empty_preview, right_chunks[1]);
    }
}

// ==========================================
// TAB 1: VIEW PROJECTS (REPLACES INIT STRUCTURE)
// ==========================================
fn render_view_projects_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(area);

    let show_search = app.is_searching || !app.search_query.is_empty();
    let left_chunks = if show_search {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Filter bar
                Constraint::Length(3), // Search bar
                Constraint::Min(6),    // List of projects
            ])
            .split(chunks[0])
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Filter bar
                Constraint::Min(8),    // List of projects
            ])
            .split(chunks[0])
    };

    let total_all = app.projects_list.len();
    let total_personal = app.projects_list.iter().filter(|p| matches!(p.category, ProjectCategory::Personal)).count();
    let total_clients = app.projects_list.iter().filter(|p| matches!(p.category, ProjectCategory::Client(_))).count();
    let total_resources = app.projects_list.iter().filter(|p| matches!(p.category, ProjectCategory::Resource)).count();
    let total_archive = app.projects_list.iter().filter(|p| matches!(p.category, ProjectCategory::Archive(_))).count();

    let filters = [
        format!("All ({})", total_all),
        format!("Personal ({})", total_personal),
        format!("Clients ({})", total_clients),
        format!("Resources ({})", total_resources),
        format!("Archive ({})", total_archive),
    ];
    let filter_lines = Line::from(
        filters
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let is_sel = i == app.projects_filter_idx;
                if is_sel {
                    Span::styled(
                        format!("▸[{}]◂", name),
                        Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::styled(format!("  {}  ", name), Style::default().fg(Color::DarkGray))
                }
            })
            .collect::<Vec<_>>(),
    );

    let filter_box = Paragraph::new(filter_lines).block(
        Block::default()
            .title(" ◉ Category Filter ")
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(filter_box, left_chunks[0]);

    let filtered = app.filtered_projects();

    if show_search {
        let search_box = Paragraph::new(Line::from(vec![
            Span::styled(" [Search] ", Style::default().fg(Color::Yellow)),
            Span::styled(&app.search_query, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(if app.is_searching { "█" } else { "" }, Style::default().fg(Color::Cyan)),
            Span::styled(format!(" ({} matching) ", filtered.len()), Style::default().fg(Color::DarkGray)),
            Span::styled(" [Esc to exit]", Style::default().fg(Color::DarkGray)),
        ])).block(
            Block::default()
                .title(if app.is_searching { " Search Active (typing...) " } else { " Active Search Filter " })
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Cyan)),
        );
        f.render_widget(search_box, left_chunks[1]);
    }

    let project_items: Vec<ListItem> = if filtered.is_empty() {
        vec![ListItem::new(Line::from(if show_search { "  No matching projects found. Press Esc to clear search." } else { "  No projects found in this category." }))
            .style(Style::default().fg(Color::DarkGray))]
    } else {
        let use_nf = app.config.use_nerd_font();
        filtered
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let is_sel = i == app.selected_project_idx;
                let (cat_label, color) = match &p.category {
                    ProjectCategory::Personal => ("Personal", Color::Green),
                    ProjectCategory::Client(c) => (c.as_str(), Color::Yellow),
                    ProjectCategory::Resource => ("Resource", Color::Cyan),
                    ProjectCategory::Archive(y) => (y.as_str(), Color::DarkGray),
                };

                let mut spans = Vec::new();
                let is_pinned = app.config.is_pinned_project(&p.name);
                if is_pinned {
                    spans.push(Span::styled("★ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
                }

                if use_nf {
                    let p_icon = crate::icons::project_icon(&p.detected_type, true);
                    if !p_icon.is_empty() {
                        spans.push(Span::styled(p_icon, Style::default().fg(Color::Cyan)));
                    }
                }

                // Git status indicator
                match p.git_status {
                    crate::utils::GitRepoStatus::Clean => {
                        spans.push(Span::styled("✓ ", Style::default().fg(Color::Green)));
                    }
                    crate::utils::GitRepoStatus::Modified => {
                        spans.push(Span::styled("● ", Style::default().fg(Color::Yellow)));
                    }
                    crate::utils::GitRepoStatus::NotRepo => {}
                }

                spans.push(Span::styled(
                    format!("{:<15} ", p.name),
                    if is_sel {
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    },
                ));
                spans.push(Span::styled(format!("[{}] ", cat_label), Style::default().fg(color)));
                spans.push(Span::styled(
                    format!("({} files)", p.file_count),
                    Style::default().fg(Color::DarkGray),
                ));
                ListItem::new(Line::from(spans))
            })
            .collect()
    };

    let mut state = ListState::default();
    if !filtered.is_empty() {
        state.select(Some(app.selected_project_idx));
    }

    let projects_list_widget = List::new(project_items)
        .block(
            Block::default()
                .title(format!(
                    " ▸ Workspace Projects ({}) [Sort: {}] ",
                    filtered.len(),
                    app.sort_mode.label()
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(40, 60, 80))
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▸ ");
    let list_target = if show_search { left_chunks[2] } else { left_chunks[1] };
    f.render_stateful_widget(projects_list_widget, list_target, &mut state);

    // Right: Selected Project Card & Actions
    if let Some(proj) = filtered.get(app.selected_project_idx) {
        let (cat_str, cat_col) = match &proj.category {
            ProjectCategory::Personal => ("Personal Project", Color::Green),
            ProjectCategory::Client(c) => (c.as_str(), Color::Yellow),
            ProjectCategory::Resource => ("Resource Library", Color::Cyan),
            ProjectCategory::Archive(y) => (y.as_str(), Color::DarkGray),
        };

        let path_str = if let Ok(home) = std::env::var("HOME") {
            let p_str = proj.path.display().to_string();
            if p_str.starts_with(&home) {
                format!("~{}", &p_str[home.len()..])
            } else {
                p_str
            }
        } else {
            proj.path.display().to_string()
        };

        let (git_badge, git_col) = match proj.git_status {
            crate::utils::GitRepoStatus::Clean => ("✓ Clean (Up to date)", Color::Green),
            crate::utils::GitRepoStatus::Modified => ("● Uncommitted Changes", Color::Yellow),
            crate::utils::GitRepoStatus::NotRepo => ("No Git Repo", Color::DarkGray),
        };

        let health_color = if proj.health_score >= 90 {
            Color::Green
        } else if proj.health_score >= 70 {
            Color::Yellow
        } else {
            Color::Red
        };

        let last_mod_str = crate::utils::format_relative_time(proj.last_modified);
        let created_str = match proj.created_at {
            Some(time) => crate::utils::format_date_time(time),
            None => "Unknown".to_string(),
        };
        let is_proj_pinned = app.config.is_pinned_project(&proj.name);

        let mut detail_lines = vec![
            Line::from(vec![
                Span::styled("Title:   ", Style::default().fg(Color::DarkGray)),
                Span::styled(&proj.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::raw("   "),
                Span::styled(format!("[{}]", cat_str), Style::default().fg(cat_col).add_modifier(Modifier::BOLD)),
                Span::raw("   "),
                Span::styled("Stack: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&proj.detected_type, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("   "),
                Span::styled(if is_proj_pinned { "★ Pinned to Top" } else { "" }, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Path:    ", Style::default().fg(Color::DarkGray)),
                Span::styled(path_str, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("Content: ", Style::default().fg(Color::DarkGray)),
                Span::raw(format!("{} files, {} directories ({})", proj.file_count, proj.dir_count, crate::tree::format_size(proj.total_bytes))),
            ]),
            Line::from(vec![
                Span::styled("Created: ", Style::default().fg(Color::DarkGray)),
                Span::styled(created_str, Style::default().fg(Color::White)),
                Span::raw("   "),
                Span::styled("Modified: ", Style::default().fg(Color::DarkGray)),
                Span::styled(last_mod_str, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Git:     ", Style::default().fg(Color::DarkGray)),
                Span::styled(git_badge, Style::default().fg(git_col).add_modifier(Modifier::BOLD)),
                Span::raw("   "),
                Span::styled("Health: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{}%", proj.health_score), Style::default().fg(health_color).add_modifier(Modifier::BOLD)),
            ]),
        ];

        let use_nf = app.config.use_nerd_font();

        if !proj.preview_files.is_empty() {
            detail_lines.push(Line::from(""));
            detail_lines.push(Line::from(Span::styled("Recent Project Files:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));
            for f in proj.preview_files.iter().take(5) {
                let f_icon = crate::icons::file_icon(Path::new(f), use_nf);
                detail_lines.push(Line::from(vec![
                    Span::styled(format!("  {} ", f_icon.trim()), Style::default().fg(Color::Green)),
                    Span::styled(f, Style::default().fg(Color::White)),
                ]));
            }
        }

        detail_lines.push(Line::from(""));
        detail_lines.push(Line::from(Span::styled("README Preview:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));

        let mut readme_found: Option<String> = None;
        for f in &proj.preview_files {
            let n = f.to_lowercase();
            if n == "readme.md" || n == "readme.txt" || n.starts_with("readme") {
                readme_found = Some(f.clone());
                break;
            }
        }
        if readme_found.is_none() {
            if proj.path.join("README.md").exists() {
                readme_found = Some("README.md".to_string());
            } else if proj.path.join("readme.md").exists() {
                readme_found = Some("readme.md".to_string());
            }
        }
        if let Some(rname) = readme_found.clone() {
            let r_icon = crate::icons::file_icon(Path::new(&rname), use_nf);
            detail_lines.push(Line::from(vec![
                Span::styled(format!("  {} ", r_icon.trim()), Style::default().fg(Color::Yellow)),
                Span::styled(rname.clone(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(" [Inline Preview]", Style::default().fg(Color::DarkGray)),
            ]));
            let readme_full_path = proj.path.join(&rname);
            if let Ok(content) = std::fs::read_to_string(&readme_full_path) {
                let snippet_lines: Vec<&str> = content
                    .lines()
                    .map(|l| l.trim())
                    .filter(|l| !l.is_empty())
                    .take(4)
                    .collect();
                for s in snippet_lines {
                    let truncated = if s.len() > 60 { format!("{}...", &s[..57]) } else { s.to_string() };
                    detail_lines.push(Line::from(vec![
                        Span::styled("    │ ", Style::default().fg(Color::Cyan)),
                        Span::styled(truncated, Style::default().fg(Color::White)),
                    ]));
                }
            }
        } else {
            detail_lines.push(Line::from(vec![
                Span::styled("  (No README found in this project)", Style::default().fg(Color::DarkGray)),
            ]));
        }

        let card_box = Paragraph::new(detail_lines)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title(" ◈ Selected Project Details ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(Color::Cyan)),
            );
        f.render_widget(card_box, chunks[1]);
    } else {
        let empty_box = Paragraph::new(if show_search { "No projects match your active search query.\nPress Esc to reset search." } else { "No projects found in this category." }).block(
            Block::default()
                .title(" Project Overview ")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        );
        f.render_widget(empty_box, chunks[1]);
    }
}

// ==========================================
// WORKSPACE CONFIGURATION & PROJECT PATHS
// ==========================================
fn render_config_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(18), // Expansive Config Form with Template Folder Path
            Constraint::Min(8),     // Workspace architecture guidelines
        ])
        .split(area);

    let is_editing_ws = app.config_field_idx == 0 && app.is_input_editing;
    let is_editing_tpl_dir = app.config_field_idx == 1 && app.is_input_editing;
    let is_editing_author = app.config_field_idx == 2 && app.is_input_editing;
    let is_editing_tpl = app.config_field_idx == 4 && app.is_input_editing;

    let form_lines = vec![
        Line::from(vec![
            Span::styled(
                if app.config_field_idx == 0 { "▶ Workspace Root Directory:       " } else { "  Workspace Root Directory:       " },
                if app.config_field_idx == 0 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            Span::styled(
                if is_editing_ws { format!("{}_", app.config.workspace_root) } else { app.config.workspace_root.clone() },
                if is_editing_ws {
                    Style::default().fg(Color::Yellow).bg(Color::Rgb(40, 50, 70)).add_modifier(Modifier::BOLD)
                } else if app.config_field_idx == 0 {
                    Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::UNDERLINED)
                } else {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                },
            ),
            if is_editing_ws {
                Span::styled(" ● [EDITING]", Style::default().fg(Color::Yellow))
            } else {
                Span::raw("")
            },
        ]),
        Line::from(vec![
            Span::styled(
                if app.config_field_idx == 1 { "▶ Custom Templates Folder Path:   " } else { "  Custom Templates Folder Path:   " },
                if app.config_field_idx == 1 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            Span::styled(
                if is_editing_tpl_dir { format!("{}_", app.config.custom_templates_dir) } else { app.config.custom_templates_dir.clone() },
                if is_editing_tpl_dir {
                    Style::default().fg(Color::Yellow).bg(Color::Rgb(40, 50, 70)).add_modifier(Modifier::BOLD)
                } else if app.config_field_idx == 1 {
                    Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::UNDERLINED)
                } else {
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                },
            ),
            if is_editing_tpl_dir {
                Span::styled(" ● [EDITING]", Style::default().fg(Color::Yellow))
            } else {
                Span::raw("")
            },
        ]),
        Line::from(vec![
            Span::styled("   Standard Subfolders:            ", Style::default().fg(Color::DarkGray)),
            Span::styled("personal/   clients/   resources/   archive/", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(" (standard convention)", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                if app.config_field_idx == 2 { "▶ Default Author Name:            " } else { "  Default Author Name:            " },
                if app.config_field_idx == 2 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            Span::styled(
                if is_editing_author { format!("{}_", app.config.default_author) } else { app.config.default_author.clone() },
                if is_editing_author {
                    Style::default().fg(Color::Yellow).bg(Color::Rgb(40, 50, 70)).add_modifier(Modifier::BOLD)
                } else if app.config_field_idx == 2 {
                    Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::UNDERLINED)
                } else {
                    Style::default().fg(Color::White)
                },
            ),
            if is_editing_author {
                Span::styled(" ● [EDITING]", Style::default().fg(Color::Yellow))
            } else {
                Span::raw("")
            },
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                if app.config_field_idx == 3 { "▶ Default License:                " } else { "  Default License:                " },
                if app.config_field_idx == 3 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            Span::styled(
                format!("< {} >", app.config.default_license),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                if app.config_field_idx == 4 { "▶ Default Template:               " } else { "  Default Template:               " },
                if app.config_field_idx == 4 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            Span::styled(
                if is_editing_tpl { format!("{}_", app.config.default_template) } else { app.config.default_template.clone() },
                if is_editing_tpl {
                    Style::default().fg(Color::Yellow).bg(Color::Rgb(40, 50, 70)).add_modifier(Modifier::BOLD)
                } else if app.config_field_idx == 4 {
                    Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::UNDERLINED)
                } else {
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                },
            ),
            if is_editing_tpl {
                Span::styled(" ● [EDITING]", Style::default().fg(Color::Yellow))
            } else {
                Span::raw("")
            },
        ]),
        Line::from(vec![
            Span::styled("   Favorite Templates (Pinned):    ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                if app.config.favorite_templates.is_empty() {
                    "None".to_string()
                } else {
                    app.config.favorite_templates.join(", ")
                },
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                if app.config_field_idx == 5 { "▶ Icon Mode (Glyph System):     " } else { "  Icon Mode (Glyph System):     " },
                if app.config_field_idx == 5 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            Span::styled(
                format!("< {} >", app.config.icon_mode),
                Style::default().fg(if app.config.use_nerd_font() { Color::Green } else { Color::Yellow }).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                if app.config.use_nerd_font() { " (Nerd Font enabled - pure Rust UTF-8 glyphs)" } else { " (Clean ASCII - universal terminal compatibility)" },
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                if app.config_field_idx == 6 { "▶ [ SAVE CONFIGURATION ] " } else { "  [ SAVE CONFIGURATION ] " },
                if app.config_field_idx == 6 { Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::Green) },
            ),
            Span::raw("   "),
            Span::styled(
                if app.config_field_idx == 7 { "▶ [ INITIALIZE MASTER WORKSPACE ] " } else { "  [ INITIALIZE MASTER WORKSPACE ] " },
                if app.config_field_idx == 7 { Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::Cyan) },
            ),
        ]),
    ];

    let form_box = Paragraph::new(form_lines).block(
        Block::default()
            .title(format!(" ▶ Configuration File: {} ", crate::config::get_config_file_path().display()))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(form_box, chunks[0]);

    let info_lines = vec![
        Line::from(Span::styled("Master Workspace Architecture:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("  • personal/   : ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("Personal projects, experimental prototypes, and portfolio work."),
        ]),
        Line::from(vec![
            Span::styled("  • clients/    : ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Client deliverables, briefs, draft reviews, and invoices."),
        ]),
        Line::from(vec![
            Span::styled("  • resources/  : ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("Master asset library (3D models, textures, audio SFX, fonts, stock footage)."),
        ]),
        Line::from(vec![
            Span::styled("  • archive/    : ", Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)),
            Span::raw("Yearly cold-storage archive for finished projects (e.g. archive/2026/)."),
        ]),
        Line::from(""),
        Line::from("  > Use the [INITIALIZE MASTER WORKSPACE] button above to seed the entire directory structure!"),
    ];

    let info_box = Paragraph::new(info_lines)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(" Workspace Architecture Guidelines ")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        );
    f.render_widget(info_box, chunks[1]);
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let status_text = if app.is_searching {
        Line::from(vec![
            Span::styled(" SEARCHING ", Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(format!(" Query: \"{}\" | Esc to clear", app.search_query)),
        ])
    } else if app.confirm_archive.is_some() {
        Line::from(vec![
            Span::styled(" CONFIRM ARCHIVE ", Style::default().fg(Color::Black).bg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" Press [Y] to confirm or [N/Esc] to cancel"),
        ])
    } else if app.doctor_popup.is_some() {
        Line::from(vec![
            Span::styled(" DOCTOR AUDIT ", Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Viewing health report | Esc to close"),
        ])
    } else if app.tree_popup.is_some() {
        Line::from(vec![
            Span::styled(" TREE VIEWER ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" [↑/↓] Scroll | Esc to close"),
        ])
    } else if app.manual_template_modal.is_some() {
        Line::from(vec![
            Span::styled(" MANUAL TEMPLATE CREATOR ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" Define custom template | Enter: Save | Esc: Cancel"),
        ])
    } else if let Some(ref msg) = app.status_message {
        Line::from(vec![
            Span::styled(" STATUS: ", Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(format!(" {}", msg)),
        ])
    } else {
        Line::from(vec![
            Span::styled(" READY ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(" [Tab] Sections | [/] Search | [q] Exit", Style::default().fg(Color::DarkGray)),
        ])
    };

    let status_bar = Paragraph::new(status_text);
    f.render_widget(status_bar, chunks[0]);

    let key_hints = match app.current_tab {
        0 => Line::from(vec![
            Span::styled("[F]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Pin "),
            Span::styled("[S]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Sort "),
            Span::styled("[I]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" IDE "),
            Span::styled("[$]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Shell "),
            Span::styled("[E]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" MD "),
            Span::styled("[O]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Open "),
            Span::styled("[D]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Doctor "),
            Span::styled("[P]", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            Span::raw(" Tree "),
            Span::styled("[A]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Archive "),
            Span::styled("[/]", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::raw(" Search "),
            Span::styled("[?]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" Help "),
            Span::styled("[q]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" Quit"),
        ]),
        1 => Line::from(vec![
            Span::styled("[Esc]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" Close "),
            Span::styled("[Tab]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" Next Field "),
            Span::styled("[Enter]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Edit/Create "),
            Span::styled("[←/→]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" Options "),
            Span::styled("[F]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Fav"),
        ]),
        _ => Line::from(vec![
            Span::styled("[Esc]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" Close "),
            Span::styled("[Tab]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" Field "),
            Span::styled("[Enter]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Edit/Save "),
            Span::styled("[←/→/Space]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" Cycle "),
            Span::styled("[Ctrl+S]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Save"),
        ]),
    };

    let hints_bar = Paragraph::new(key_hints).alignment(Alignment::Right);
    f.render_widget(hints_bar, chunks[1]);
}

fn render_size_warning(f: &mut Frame, area: Rect) {
    let warning_area = centered_rect(80, 50, area);
    f.render_widget(Clear, warning_area);
    let text = vec![
        Line::from(""),
        Line::from(Span::styled("[!] TERMINAL WINDOW TOO SMALL", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(format!("Current Dimensions:  {} cols × {} rows", area.width, area.height)),
        Line::from("Minimum Required:    75 cols × 20 rows"),
        Line::from(""),
        Line::from(Span::styled("Please resize or maximize your terminal window to continue.", Style::default().fg(Color::Cyan))),
    ];
    let block = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(" Dimension Alert ")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Yellow)),
        );
    f.render_widget(block, warning_area);
}

fn render_confirm_archive_modal(f: &mut Frame, name: &str, path: &Path) {
    let area = centered_rect(65, 38, f.area());
    f.render_widget(Clear, area);

    let text = vec![
        Line::from(""),
        Line::from(Span::styled("[!] CONFIRM PROJECT ARCHIVE", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("Project Title: ", Style::default().fg(Color::DarkGray)),
            Span::styled(name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Current Path:  ", Style::default().fg(Color::DarkGray)),
            Span::styled(path.display().to_string(), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from("Are you sure you want to move this project to the yearly archive?"),
        Line::from(""),
        Line::from(vec![
            Span::styled(" [ Y ] Yes, Archive Now ", Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("        "),
            Span::styled(" [ N / Esc ] Cancel ", Style::default().fg(Color::Black).bg(Color::Red).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let block = Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(" Safe Archive Confirmation ")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Yellow)),
        );
    f.render_widget(block, area);
}

fn render_doctor_modal(f: &mut Frame, name: &str, report: &crate::maintainer::AuditReport) {
    let area = centered_rect(75, 65, f.area());
    f.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Info Header
            Constraint::Length(2), // Gauge
            Constraint::Min(6),    // Issues List
            Constraint::Length(2), // Close hint
        ])
        .margin(1)
        .split(area);

    let container = Block::default()
        .title(format!(" Project Health Audit: {} ", name))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Color::Green));
    f.render_widget(container, area);

    let header_line = Line::from(vec![
        Span::styled("Detected Profile: ", Style::default().fg(Color::DarkGray)),
        Span::styled(&report.project_type, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("    "),
        Span::styled("Total Content: ", Style::default().fg(Color::DarkGray)),
        Span::raw(format!("{} files, {} directories", report.total_files, report.total_dirs)),
    ]);
    f.render_widget(Paragraph::new(header_line), chunks[0]);

    let (score_color, level) = if report.health_score >= 90 {
        (Color::Green, "EXCELLENT")
    } else if report.health_score >= 70 {
        (Color::Yellow, "GOOD")
    } else {
        (Color::Red, "NEEDS MAINTENANCE")
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(Style::default().fg(score_color).bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .percent(report.health_score as u16)
        .label(format!("Health Score: {}/100 ({})", report.health_score, level));
    f.render_widget(gauge, chunks[1]);

    let mut issue_items = Vec::new();
    if report.issues.is_empty() {
        issue_items.push(ListItem::new(Line::from(vec![
            Span::styled("[OK] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("Perfect health! All standard documentation, git config, and project files are in place.", Style::default().fg(Color::Green)),
        ])));
    } else {
        for (i, iss) in report.issues.iter().enumerate() {
            let (sev_str, sev_color) = match iss.severity {
                crate::maintainer::IssueSeverity::Critical => ("[CRITICAL]", Color::Red),
                crate::maintainer::IssueSeverity::Warning => ("[WARNING]", Color::Yellow),
                crate::maintainer::IssueSeverity::Info => ("[INFO]", Color::Cyan),
            };
            let lines = vec![
                Line::from(vec![
                    Span::styled(format!("{}. ", i + 1), Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{} ", sev_str), Style::default().fg(sev_color).add_modifier(Modifier::BOLD)),
                    Span::styled(&iss.title, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::raw("   "),
                    Span::styled(&iss.message, Style::default().fg(Color::DarkGray)),
                ]),
            ];
            issue_items.push(ListItem::new(lines));
        }
    }

    let issues_widget = List::new(issue_items).block(
        Block::default()
            .title(format!(" Audit Checks & Findings ({}) ", report.issues.len()))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain),
    );
    f.render_widget(issues_widget, chunks[2]);

    let close_hint = Paragraph::new("[ Esc / Enter / D to close ]")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(close_hint, chunks[3]);
}

fn render_tree_modal(f: &mut Frame, name: &str, lines: &[String], scroll: usize) {
    let area = centered_rect(80, 75, f.area());
    f.render_widget(Clear, area);

    let max_lines = area.height.saturating_sub(4) as usize;
    let visible_lines: Vec<ListItem> = lines
        .iter()
        .skip(scroll)
        .take(max_lines)
        .map(|l| ListItem::new(Line::from(l.as_str())))
        .collect();

    let tree_widget = List::new(visible_lines).block(
        Block::default()
            .title(format!(" Directory Tree: {} (Lines {}-{}/{}) ", name, scroll + 1, (scroll + max_lines).min(lines.len()), lines.len()))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(tree_widget, area);
}

fn render_popup(f: &mut Frame, message: &str) {
    let area = centered_rect(65, 30, f.area());
    f.render_widget(Clear, area);

    let is_error = message.contains("[ERROR]");
    let is_ok = message.contains("[OK]");
    let is_warning = message.contains("[WARNING]") || message.contains("[!]");

    let (border_color, title) = if is_error {
        (Color::Red, " ✕ ERROR ")
    } else if is_ok {
        (Color::Green, " ✓ SUCCESS ")
    } else if is_warning {
        (Color::Yellow, " ⚠ NOTICE ")
    } else {
        (Color::Cyan, " ◉ Notice ")
    };

    let popup = Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(border_color)),
        );
    f.render_widget(popup, area);
}

fn render_help_modal(f: &mut Frame) {
    let area = centered_rect(78, 80, f.area());
    f.render_widget(Clear, area);

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  UNIFIED WORKSPACE NAVIGATION & SHORTCUTS", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("    [N]                 ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("Open New Project (Scaffold) modal to seed a project"),
        ]),
        Line::from(vec![
            Span::styled("    [C]                 ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Open Workspace Settings / Configuration modal"),
        ]),
        Line::from(vec![
            Span::styled("    [Esc]               ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("Close any open modal and return to Workspace Dashboard"),
        ]),
        Line::from(vec![
            Span::styled("    [?] or [F1]         ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Toggle this Help cheat sheet modal anytime"),
        ]),
        Line::from(vec![
            Span::styled("    [q]                 ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw("Quit the application (when not editing a text field)"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  WORKSPACE DASHBOARD (PROJECT EXPLORER)", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("    [↑ / ↓] or [k / j]  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Navigate the list of workspace projects"),
        ]),
        Line::from(vec![
            Span::styled("    [← / →] or [h / l]  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Cycle category filters (All / Personal / Clients / Resources / Archive)"),
        ]),
        Line::from(vec![
            Span::styled("    [E]                 ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("Open in-TUI Markdown Editor / Viewer for project README"),
        ]),
        Line::from(vec![
            Span::styled("    [O] or [Enter]      ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("Open project directory in your default system file manager"),
        ]),
        Line::from(vec![
            Span::styled("    [D]                 ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("Run Doctor health audit & view visual gauge score modal"),
        ]),
        Line::from(vec![
            Span::styled("    [P] or [V]          ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            Span::raw("Interactive directory tree modal (use ↑/↓ to scroll)"),
        ]),
        Line::from(vec![
            Span::styled("    [A]                 ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Safely move project into yearly archive (confirms with Y/N dialog)"),
        ]),
        Line::from(vec![
            Span::styled("    [/]                 ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::raw("Instant search mode (search by project name, stack, or client)"),
        ]),
        Line::from(vec![
            Span::styled("    [R]                 ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw("Reload & refresh project list from disk"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  MODAL DIALOG CONTROLS", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("    [Tab] / [Shift+Tab] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Navigate form fields / cycle section options"),
        ]),
        Line::from(vec![
            Span::styled("    [Enter]             ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("Toggle edit mode on text fields, or submit action buttons"),
        ]),
        Line::from(vec![
            Span::styled("    [F]                 ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Pin / unpin template to top favorites"),
        ]),
        Line::from(vec![
            Span::styled("    [Ctrl+S]            ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("Save configuration instantly or save manual template"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("                [ Press Esc / ? / Enter / Space / q to Close ]", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" ⌨ Help & Keyboard Shortcuts Reference ")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(block, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_manual_template_modal(f: &mut Frame, form: &super::ManualTemplateForm) {
    let area = centered_rect(76, 68, f.area());
    f.render_widget(Clear, area);

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Define a custom manual template with structured directories and starter files.", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                if form.active_field == 0 { "▶ 1. Template Name:    " } else { "  1. Template Name:    " },
                if form.active_field == 0 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            Span::styled(
                if form.name.is_empty() && form.active_field != 0 { "(e.g. my-workflow, blog-engine)" } else { &form.name },
                if form.active_field == 0 {
                    Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::UNDERLINED)
                } else if form.name.is_empty() {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                },
            ),
        ]),
        Line::from(vec![
            Span::styled(
                if form.active_field == 1 { "▶ 2. Description:      " } else { "  2. Description:      " },
                if form.active_field == 1 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            Span::styled(
                &form.description,
                if form.active_field == 1 {
                    Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::UNDERLINED)
                } else {
                    Style::default().fg(Color::White)
                },
            ),
        ]),
        Line::from(vec![
            Span::styled(
                if form.active_field == 2 { "▶ 3. Directories:      " } else { "  3. Directories:      " },
                if form.active_field == 2 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            Span::styled(
                &form.directories,
                if form.active_field == 2 {
                    Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::UNDERLINED)
                } else {
                    Style::default().fg(Color::Yellow)
                },
            ),
            Span::styled(" (comma-separated)", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(
                if form.active_field == 3 { "▶ 4. Starter Files:    " } else { "  4. Starter Files:    " },
                if form.active_field == 3 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            Span::styled(
                &form.files,
                if form.active_field == 3 {
                    Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::UNDERLINED)
                } else {
                    Style::default().fg(Color::Green)
                },
            ),
            Span::styled(" (comma-separated)", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(
                if form.active_field == 4 { "▶ 5. Deliverables:     " } else { "  5. Deliverables:     " },
                if form.active_field == 4 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            Span::styled(
                &form.outputs,
                if form.active_field == 4 {
                    Style::default().fg(Color::White).bg(Color::DarkGray).add_modifier(Modifier::UNDERLINED)
                } else {
                    Style::default().fg(Color::Cyan)
                },
            ),
            Span::styled(" (target deliverables)", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(
                if form.active_field == 5 { "▶ 6. Target Location:  " } else { "  6. Target Location:  " },
                if form.active_field == 5 { Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) },
            ),
            if form.save_to_workspace {
                Span::styled("[Workspace: .seed/templates/]", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("[User Config: ~/.config/seed/templates/]", Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD))
            },
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                if form.active_field == 6 { "▶ [ SAVE MANUAL TEMPLATE ] " } else { "  [ SAVE MANUAL TEMPLATE ] " },
                if form.active_field == 6 { Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::Green) },
            ),
            Span::raw("    "),
            Span::styled(
                if form.active_field == 7 { "▶ [ CANCEL ] " } else { "  [ CANCEL ] " },
                if form.active_field == 7 { Style::default().fg(Color::Black).bg(Color::Red).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::Red) },
            ),
        ]),
    ];

    let block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" ◈ Create New Manual Template ")
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(block, area);
}

fn render_markdown_editor_modal(f: &mut Frame, modal: &MarkdownEditorModal) {
    let area = centered_rect(88, 86, f.area());
    f.render_widget(Clear, area);

    let (title, border_color) = match modal.mode {
        MarkdownModalMode::Preview => (
            format!(
                " ◈ Markdown Document Viewer: {} (Preview Mode) ",
                modal.project_name
            ),
            Color::Cyan,
        ),
        MarkdownModalMode::Edit => (
            format!(
                " ◈ Markdown Buffer Editor: {} {} ",
                modal.project_name,
                if modal.is_dirty { "● [Modified]" } else { "✓ [Saved]" }
            ),
            if modal.is_dirty { Color::Yellow } else { Color::Green },
        ),
    };

    let container = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(border_color));
    f.render_widget(container, area);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Top path subtitle
            Constraint::Min(4),    // Content
            Constraint::Length(1), // Status & key hints
        ])
        .margin(1)
        .split(area);

    let path_subtitle = Line::from(vec![
        Span::styled("File: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            modal.file_path.display().to_string(),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
        Span::raw("   "),
        Span::styled(
            format!("Lines: {}", modal.lines.len()),
            Style::default().fg(Color::DarkGray),
        ),
        Span::raw("   "),
        Span::styled("Mode: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            match modal.mode {
                MarkdownModalMode::Preview => "PREVIEW (Rich formatting)",
                MarkdownModalMode::Edit => "EDIT (Interactive buffer)",
            },
            Style::default().fg(if modal.mode == MarkdownModalMode::Edit { Color::Yellow } else { Color::Cyan }).add_modifier(Modifier::BOLD),
        ),
    ]);
    f.render_widget(Paragraph::new(path_subtitle), inner_chunks[0]);

    let content_area = inner_chunks[1];
    let visible_height = content_area.height as usize;

    match modal.mode {
        MarkdownModalMode::Preview => {
            let visible_lines: Vec<Line> = modal
                .lines
                .iter()
                .skip(modal.scroll_offset)
                .take(visible_height)
                .map(|l| format_markdown_preview_line(l))
                .collect();

            let preview_widget = Paragraph::new(visible_lines).wrap(Wrap { trim: false });
            f.render_widget(preview_widget, content_area);

            let footer_line = Line::from(vec![
                Span::styled(" [Tab / E] ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" Edit Buffer  "),
                Span::styled(" [↑ / ↓] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" Scroll  "),
                Span::styled(" [Ctrl+S] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" Save  "),
                Span::styled(" [Esc / q] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" Close Viewer  "),
                Span::styled(
                    format!("(Line {}/{})", modal.scroll_offset + 1, modal.lines.len()),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            f.render_widget(Paragraph::new(footer_line), inner_chunks[2]);
        }
        MarkdownModalMode::Edit => {
            let cursor_row = modal.cursor_row;
            let mut scroll_offset = modal.scroll_offset;
            if cursor_row < scroll_offset {
                scroll_offset = cursor_row;
            } else if cursor_row >= scroll_offset + visible_height {
                scroll_offset = cursor_row - visible_height + 1;
            }

            let mut edit_lines = Vec::new();
            for i in 0..visible_height {
                let line_idx = scroll_offset + i;
                if line_idx >= modal.lines.len() {
                    edit_lines.push(Line::from(vec![
                        Span::styled("   ~ │ ", Style::default().fg(Color::DarkGray)),
                    ]));
                    continue;
                }

                let line_text = &modal.lines[line_idx];
                let is_cursor_line = line_idx == cursor_row;

                let line_num_str = format!("{:4} │ ", line_idx + 1);
                let line_num_span = if is_cursor_line {
                    Span::styled(line_num_str, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                } else {
                    Span::styled(line_num_str, Style::default().fg(Color::DarkGray))
                };

                let mut spans = vec![line_num_span];

                if is_cursor_line {
                    let chars: Vec<char> = line_text.chars().collect();
                    let cursor_col = modal.cursor_col;
                    if cursor_col < chars.len() {
                        let before: String = chars[..cursor_col].iter().collect();
                        let cursor_char = chars[cursor_col].to_string();
                        let after: String = chars[cursor_col + 1..].iter().collect();

                        if !before.is_empty() {
                            spans.push(Span::styled(before, Style::default().fg(Color::White)));
                        }
                        spans.push(Span::styled(
                            cursor_char,
                            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
                        ));
                        if !after.is_empty() {
                            spans.push(Span::styled(after, Style::default().fg(Color::White)));
                        }
                    } else {
                        spans.push(Span::styled(line_text.clone(), Style::default().fg(Color::White)));
                        spans.push(Span::styled(
                            " ",
                            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
                        ));
                    }
                } else {
                    spans.push(Span::styled(line_text.clone(), Style::default().fg(Color::White)));
                }

                edit_lines.push(Line::from(spans));
            }

            let edit_widget = Paragraph::new(edit_lines);
            f.render_widget(edit_widget, content_area);

            let dirty_badge = if modal.is_dirty {
                Span::styled(" [MODIFIED] ", Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD))
            } else {
                Span::styled(" [SAVED] ", Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD))
            };

            let footer_line = Line::from(vec![
                dirty_badge,
                Span::raw(" "),
                Span::styled(
                    format!("Ln {}, Col {} ", modal.cursor_row + 1, modal.cursor_col + 1),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
                Span::styled("│ [Ctrl+S] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled("Save ", Style::default().fg(Color::White)),
                Span::styled("│ [Tab] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("Preview ", Style::default().fg(Color::White)),
                Span::styled("│ [Ctrl+T] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("Task Box ", Style::default().fg(Color::White)),
                Span::styled("│ [Esc] ", Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)),
                Span::styled("Preview Mode", Style::default().fg(Color::DarkGray)),
            ]);
            f.render_widget(Paragraph::new(footer_line), inner_chunks[2]);
        }
    }
}

fn format_markdown_preview_line(raw: &str) -> Line<'static> {
    if let Some(rest) = raw.strip_prefix("# ") {
        Line::from(vec![
            Span::styled("█ ", Style::default().fg(Color::Cyan)),
            Span::styled(rest.to_string(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ])
    } else if let Some(rest) = raw.strip_prefix("## ") {
        Line::from(vec![
            Span::styled("## ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(rest.to_string(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ])
    } else if let Some(rest) = raw.strip_prefix("### ") {
        Line::from(vec![
            Span::styled("### ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(rest.to_string(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ])
    } else if let Some(rest) = raw.strip_prefix("#### ") {
        Line::from(vec![
            Span::styled("#### ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(rest.to_string(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ])
    } else if let Some(rest) = raw.strip_prefix("> ") {
        Line::from(vec![
            Span::styled("  │ ", Style::default().fg(Color::Cyan)),
            Span::styled(rest.to_string(), Style::default().fg(Color::Cyan)),
        ])
    } else if let Some(rest) = raw.strip_prefix("- [x] ").or_else(|| raw.strip_prefix("* [x] ")) {
        Line::from(vec![
            Span::styled("  [✓] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(rest.to_string(), Style::default().fg(Color::DarkGray)),
        ])
    } else if let Some(rest) = raw.strip_prefix("- [ ] ").or_else(|| raw.strip_prefix("* [ ] ")) {
        Line::from(vec![
            Span::styled("  [ ] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(rest.to_string(), Style::default().fg(Color::White)),
        ])
    } else if let Some(rest) = raw.strip_prefix("- ").or_else(|| raw.strip_prefix("* ")) {
        Line::from(vec![
            Span::styled("   •  ", Style::default().fg(Color::Yellow)),
            Span::styled(rest.to_string(), Style::default().fg(Color::White)),
        ])
    } else if raw.starts_with("```") {
        Line::from(vec![
            Span::styled(format!("  {} ", raw), Style::default().fg(Color::Yellow).bg(Color::Rgb(25, 30, 36))),
        ])
    } else {
        Line::from(vec![
            Span::styled(format!("  {}", raw), Style::default().fg(Color::White)),
        ])
    }
}

fn render_confirm_discard_modal(f: &mut Frame) {
    let area = centered_rect(58, 24, f.area());
    f.render_widget(Clear, area);
    let block = Block::default()
        .title(" ⚠ Unsaved Changes ")
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Yellow));

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Warning: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("You have unsaved changes in this document!", Style::default().fg(Color::White)),
        ]),
        Line::from(Span::styled("  Discarding will lose recent edits made to this file.", Style::default().fg(Color::DarkGray))),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Y] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled("Discard Changes    ", Style::default().fg(Color::White)),
            Span::styled("[S] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("Save & Exit    ", Style::default().fg(Color::White)),
            Span::styled("[Esc/N] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("Keep Editing", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
    ];
    let p = Paragraph::new(text).block(block);
    f.render_widget(p, area);
}
