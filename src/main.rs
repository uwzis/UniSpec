// src/main.rs
mod agent;
mod cli;
mod commands;
mod fs;
mod mcp;
mod platypus;
mod tui;
mod version;

use anyhow::Result;
use clap::Parser;

use crate::agent::{connector as agent_connector, mode as agent_mode};
use crate::cli::{
    AreaCommands, AreaOrderCommands, AutoCommands, ChangeCommands, ConnectorCommands,
    IndexCommands, IngestCommands, ModeCommands, OrderCommands, ParseCommands, PattyCommands,
    PkgCommands, QueueCommands, SpecCommands, TopicCommands,
};
use crate::cli::{Cli, Commands};
use crate::commands::{
    area, change, index, ingest, init, init_editor, queue, repo, set, spec, topic,
};

fn get_show_platypus() -> bool {
    crate::fs::config::get_paddy_enabled().unwrap_or(true)
}

/// Resolve a CLI-supplied area override to a concrete area name.
/// Falls back to `.agent/config.toml`'s `area` field, then to "Staging".
fn resolve_area_from_config(area: Option<String>) -> String {
    area.unwrap_or_else(|| {
        crate::fs::config::load_config()
            .ok()
            .map(|c| c.area)
            .unwrap_or_else(|| "Staging".to_string())
    })
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            // Default: launch TUI
            if !crate::fs::spec_dir().exists() {
                eprintln!("No spec folder found. Run unispec init to initialize a project.");
                std::process::exit(1);
            }
            tui::main::main()?;
        }
        Some(Commands::Init {
            root,
            amazon_q,
            antigravity,
            auggie,
            claude_code,
            cline,
            codex,
            codebuddy,
            continue_,
            costrict,
            crush,
            cursor,
            factory,
            gemini_cli,
            github,
            iflow,
            kilo_code,
            kiro,
            opencode,
            pi,
            qoder,
            qwen_code,
            roo_code,
            windsurf,
            trae,
        }) => {
            let mut editors = Vec::new();

            if amazon_q {
                editors.push("amazon-q");
            }
            if antigravity {
                editors.push("antigravity");
            }
            if auggie {
                editors.push("auggie");
            }
            if claude_code {
                editors.push("claude-code");
            }
            if cline {
                editors.push("cline");
            }
            if codex {
                editors.push("codex");
            }
            if codebuddy {
                editors.push("codebuddy");
            }
            if continue_ {
                editors.push("continue");
            }
            if costrict {
                editors.push("costrict");
            }
            if crush {
                editors.push("crush");
            }
            if cursor {
                editors.push("cursor");
            }
            if factory {
                editors.push("factory");
            }
            if gemini_cli {
                editors.push("gemini-cli");
            }
            if github {
                editors.push("github");
            }
            if iflow {
                editors.push("iflow");
            }
            if kilo_code {
                editors.push("kilo-code");
            }
            if kiro {
                editors.push("kiro");
            }
            if opencode {
                editors.push("opencode");
            }
            if pi {
                editors.push("pi");
            }
            if qoder {
                editors.push("qoder");
            }
            if qwen_code {
                editors.push("qwen-code");
            }
            if roo_code {
                editors.push("roo-code");
            }
            if windsurf {
                editors.push("windsurf");
            }
            if trae {
                editors.push("trae");
            }

            let root_path = root.as_deref().unwrap_or_else(|| std::path::Path::new("."));

            if !editors.is_empty() {
                let results = init_editor::run_init_editors(root_path, &editors)?;
                init_editor::print_editor_results(&results);
            } else {
                init::run_init(root.as_deref())?;
            }
        }
        Some(Commands::Set { area }) => {
            set::run_set(&area)?;
            if get_show_platypus() {
                platypus::happy();
            }
        }
        Some(Commands::Area(area_cmd)) => match area_cmd {
            AreaCommands::Add { name } => {
                area::run_add(&name)?;
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            AreaCommands::Remove { name } => {
                area::run_remove(&name)?;
                if get_show_platypus() {
                    platypus::sad();
                }
            }
            AreaCommands::List => area::run_list()?,
            AreaCommands::Rename { old, new } => {
                area::run_rename(&old, &new)?;
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            AreaCommands::Default { name } => {
                area::run_default(&name)?;
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            AreaCommands::Health => area::run_health()?,
            AreaCommands::Order { action } => match action {
                AreaOrderCommands::Show => {
                    println!("{}", area::run_area_order_show()?);
                }
                AreaOrderCommands::Add { areas, position } => {
                    println!("{}", area::run_area_order_add(areas, position)?);
                }
                AreaOrderCommands::Remove { areas } => {
                    println!("{}", area::run_area_order_remove(areas)?);
                }
                AreaOrderCommands::Reset => {
                    println!("{}", area::run_area_order_reset()?);
                }
            },
        },
        Some(Commands::Topic(topic_cmd)) => match topic_cmd {
            TopicCommands::Add {
                topic,
                area,
                short,
                content,
            } => {
                let area = resolve_area_from_config(area);
                topic::run_new(&topic, &area, Some(&short), Some(&content))?;
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            TopicCommands::List { area, hierarchy: _ } => {
                let area = resolve_area_from_config(area);
                topic::run_list(&area, false)?
            }
            TopicCommands::Push { topic, area, from } => {
                let area = resolve_area_from_config(area);
                topic::run_push(&topic, &area, from.as_deref())?;
                if get_show_platypus() {
                    platypus::working();
                }
            }
            TopicCommands::Pull { topic, area } => {
                let area = resolve_area_from_config(area);
                topic::run_pull(&topic, &area)?;
                if get_show_platypus() {
                    platypus::working();
                }
            }
            TopicCommands::Remove { topic, area, force } => {
                let area = resolve_area_from_config(area);
                topic::run_delete(&topic, &area, force)?;
                if get_show_platypus() {
                    platypus::sad();
                }
            }
            TopicCommands::Show { topic, all, from } => {
                // When neither --from nor --all is given, default to the
                // configured area so `unispec topic show <name>` picks the
                // expected one.
                let resolved_from = if all {
                    from
                } else {
                    Some(resolve_area_from_config(from))
                };
                topic::run_show(&topic, all, resolved_from.as_deref())?
            }
            TopicCommands::Progress { area } => topic::run_progress(area.as_deref())?,
            TopicCommands::Order { action } => match action {
                OrderCommands::Show { area } => {
                    let area = area.unwrap_or_else(|| "Working".to_string());
                    println!("{}", topic::run_order(&area)?);
                }
                OrderCommands::Add {
                    area,
                    topics,
                    position,
                } => {
                    let area = area.unwrap_or_else(|| "Working".to_string());
                    println!("{}", topic::run_order_add(&area, topics, position)?);
                }
                OrderCommands::Remove { area, topics } => {
                    let area = area.unwrap_or_else(|| "Working".to_string());
                    println!("{}", topic::run_order_remove(&area, topics)?);
                }
                OrderCommands::Reset { area } => {
                    let area = area.unwrap_or_else(|| "Working".to_string());
                    println!("{}", topic::run_order_reset(&area)?);
                }
            },
        },
        Some(Commands::Index(index_cmd)) => match index_cmd {
            IndexCommands::Add {
                topic,
                path,
                area,
                link_type,
                tags,
                annotation,
                exports,
                descriptions,
                export_types,
                signatures,
            } => {
                let area = area.unwrap_or_else(|| "Working".to_string());
                let link_type = link_type.unwrap_or_else(|| index::detect_type(&path));
                index::run_add(
                    &topic,
                    &path,
                    &area,
                    &link_type,
                    tags.as_deref(),
                    annotation.as_deref(),
                    exports.as_deref(),
                    descriptions.as_deref(),
                    export_types.as_deref(),
                    signatures.as_deref(),
                )?;
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            IndexCommands::Remove { topic, path } => {
                index::run_remove(&topic, &path)?;
                if get_show_platypus() {
                    platypus::sad();
                }
            }
            IndexCommands::List { topic, path, tag } => {
                index::run_list(topic.as_deref(), path.as_deref(), tag.as_deref())?;
            }
            IndexCommands::Find { query, by } => {
                index::run_find(&query, &by)?;
            }
            IndexCommands::Full => index::run_full()?,
            IndexCommands::Watch => index::run_watch()?,
            IndexCommands::Cleanup => index::run_cleanup()?,
            IndexCommands::Tags => index::run_tags()?,
            IndexCommands::Graph => index::run_graph()?,
            IndexCommands::Backlinks { topic } => index::run_backlinks(&topic)?,
            IndexCommands::Exports { topic } => index::run_exports(topic.as_deref())?,
            IndexCommands::Query { query, by } => index::run_query(&query, &by)?,
            IndexCommands::Depends { topic } => index::run_depends(&topic)?,
            IndexCommands::Lookup { id } => index::run_lookup(&id)?,
            IndexCommands::Callers { symbol } => index::run_callers(&symbol)?,
        },
        Some(Commands::Mcp { path }) => {
            let path_str = path.map(|p| p.to_string_lossy().to_string());
            mcp::server::run_mcp_server(path_str.as_deref())?;
        }
        Some(Commands::Spec(spec_cmd)) => match spec_cmd {
            SpecCommands::Show { name: _ } => {
                let master_path = crate::fs::spec_dir().join("master.md");
                if master_path.exists() {
                    let content = std::fs::read_to_string(&master_path)?;
                    println!("{}", content);
                } else {
                    println!("No master spec found. Create spec/master.md to add context.");
                }
            }
            SpecCommands::Add {
                topic,
                area,
                short,
                spec_content,
                task_content,
            } => {
                let area = area.unwrap_or_else(|| {
                    crate::fs::config::load_config()
                        .ok()
                        .map(|c| c.area)
                        .unwrap_or_else(|| "Staging".to_string())
                });
                let out = spec::run_spec_add(
                    &topic,
                    Some(&area),
                    &short,
                    &spec_content,
                    &task_content,
                )?;
                println!(
                    "✅ Spec and task created for '{}' in {}/\n  {}\n  {}",
                    out.topic,
                    out.area,
                    out.spec_path.display(),
                    out.task_path.display(),
                );
                if get_show_platypus() {
                    platypus::happy();
                }
            }
        },
        Some(Commands::Queue(queue_cmd)) => match queue_cmd {
            QueueCommands::Add {
                topic,
                area,
                position,
            } => {
                let area = resolve_area_from_config(area);
                let out = queue::run_queue_add(&topic, &area, position)?;
                println!(
                    "✅ Added '{}' to {}/{} at position {}",
                    out.topic,
                    out.area,
                    out.queue_file,
                    if out.position < 0 {
                        "end".to_string()
                    } else {
                        out.position.to_string()
                    }
                );
                if get_show_platypus() {
                    platypus::happy();
                }
            }
        },
        Some(Commands::Change(change_cmd)) => match change_cmd {
            ChangeCommands::Add {
                topic,
                change: change_name,
                area,
                proposal,
                design,
                spec_content,
                task_content,
            } => {
                let area = resolve_area_from_config(area);
                let out = change::run_change_add(
                    &topic,
                    Some(&area),
                    &change_name,
                    &proposal,
                    design.as_deref(),
                    &spec_content,
                    &task_content,
                )?;
                println!(
                    "✅ Change '{}' created for topic '{}' in {}/\n  {}\n  {}\n  {}{}",
                    out.change,
                    out.topic,
                    out.area,
                    out.proposal_path.display(),
                    out.spec_path.display(),
                    out.task_path.display(),
                    out.design_path
                        .as_ref()
                        .map(|p| format!("\n  {}", p.display()))
                        .unwrap_or_default()
                );
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            ChangeCommands::List {
                topic,
                area,
                archived,
            } => {
                let area = resolve_area_from_config(area);
                let changes = change::run_change_list(&topic, Some(&area), archived)?;
                if changes.is_empty() {
                    println!("No changes for '{}' in {}/", topic, area);
                } else {
                    println!("Changes for '{}' in {}/:", topic, area);
                    for c in &changes {
                        let mut bits = vec![];
                        if c.has_proposal {
                            bits.push("proposal");
                        }
                        if c.has_design {
                            bits.push("design");
                        }
                        if c.has_spec {
                            bits.push("spec");
                        }
                        if c.has_task {
                            bits.push("task");
                        }
                        println!("  - {} [{}] ({})", c.name, c.status, bits.join(", "));
                    }
                }
            }
            ChangeCommands::Archive {
                topic,
                change: change_name,
                area,
            } => {
                let area = resolve_area_from_config(area);
                let out = change::run_change_archive(&topic, Some(&area), &change_name)?;
                println!(
                    "✅ Archived change '{}' for topic '{}' in {}/\n  {} -> {}",
                    out.change,
                    out.topic,
                    out.area,
                    out.from.display(),
                    out.to.display()
                );
                if get_show_platypus() {
                    platypus::celebrating();
                }
            }
        },
        Some(Commands::Mode(mode_cmd)) => match mode_cmd {
            ModeCommands::List => {
                let modes = agent_mode::list_modes()?;
                if modes.is_empty() {
                    println!("No modes found.");
                    println!("Modes should be placed in .agent/modes/ or ~/.config/unispec/modes/");
                } else {
                    println!("Available Modes:\n");
                    for mode in &modes {
                        let active = if mode.is_active { " [ACTIVE]" } else { "" };
                        let source = if mode.source == agent_mode::ModeSource::Global {
                            " (global)"
                        } else {
                            ""
                        };
                        println!(
                            "  {} - {}{}{}",
                            mode.name, mode.display_name, active, source
                        );
                        println!("    {}", mode.description);
                        println!();
                    }
                }
            }
            ModeCommands::Info { name } => {
                let config = agent_mode::get_mode_info(&name)?;
                let path = agent_mode::get_mode_path(&name)?;
                let is_global = path.starts_with(crate::fs::global_modes_dir());
                println!("Mode: {}", config.mode.name);
                println!("Display Name: {}", config.mode.display_name);
                println!("Version: {}", config.mode.version);
                println!("Location: {}", if is_global { "global" } else { "local" });
                if let Some(author) = config.author {
                    println!("Author: {}", author.name);
                }
                println!("\nDescription:");
                println!("{}", config.mode.description);
                println!("\nAreas:");
                println!("  Default: {}", config.areas.default.join(", "));
                println!("  Protected: {}", config.areas.protected.join(", "));
                println!("\nCapabilities:");
                println!("  Spec Writing: {}", config.capabilities.spec_writing);
                println!("  Building: {}", config.capabilities.building);
                println!("  Verification: {}", config.capabilities.verification);
                println!("  Connectors: {}", config.capabilities.connectors);
                println!(
                    "  Custom Workflows: {}",
                    config.capabilities.custom_workflows
                );
            }
            ModeCommands::Activate { name } => {
                let result = agent_mode::run_activate(&name)?;
                println!("{}", result);
                if get_show_platypus() {
                    platypus::celebrating();
                }
            }
            ModeCommands::Add { path, global } => {
                let result = agent_mode::add_mode(&path, global)?;
                println!("{}", result);
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            ModeCommands::Remove { name, global } => {
                let result = agent_mode::remove_mode(&name, global)?;
                println!("{}", result);
                if get_show_platypus() {
                    platypus::sad();
                }
            }
            ModeCommands::Current => {
                let current = crate::agent::current_mode()?;
                println!("Current mode: {}", current);
            }
        },
        Some(Commands::Connector(conn_cmd)) => match conn_cmd {
            ConnectorCommands::New {
                name,
                description,
                command,
                args,
                env,
                working_dir,
                timeout,
            } => {
                let args: Vec<String> = args
                    .map(|s| s.split_whitespace().map(|s| s.to_string()).collect())
                    .unwrap_or_default();
                let env: Vec<(String, String)> = env
                    .map(|s| {
                        s.split(',')
                            .filter_map(|pair| {
                                let mut parts = pair.split('=');
                                match (parts.next(), parts.next()) {
                                    (Some(k), Some(v)) => {
                                        Some((k.trim().to_string(), v.trim().to_string()))
                                    }
                                    _ => None,
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                let result = agent_connector::run_new(
                    &name,
                    &description,
                    &command,
                    &args,
                    &env,
                    working_dir.as_deref(),
                    timeout,
                )?;
                println!("{}", result);
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            ConnectorCommands::List => {
                agent_connector::run_list()?;
            }
            ConnectorCommands::Delete { name } => {
                let result = agent_connector::run_delete(&name)?;
                println!("{}", result);
                if get_show_platypus() {
                    platypus::sad();
                }
            }
            ConnectorCommands::Run { name, args } => {
                let output = agent_connector::run_run(&name, &args)?;
                println!("{}", output);
            }
            ConnectorCommands::Edit { name, description } => {
                let result = agent_connector::run_edit(&name, description.as_deref())?;
                println!("{}", result);
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            ConnectorCommands::Mcp => {
                let config = agent_connector::generate_mcp_config()?;
                println!("{}", config);
            }
        },
        Some(Commands::Pkg(pkg_cmd)) => match pkg_cmd {
            PkgCommands::List { repo } => {
                repo::list_packages(repo.as_deref())?;
            }
            PkgCommands::Search { query, repo } => {
                repo::search_packages(&query, repo.as_deref())?;
            }
            PkgCommands::Install {
                package,
                global,
                repo,
            } => {
                if package.starts_with("http") || package.contains("github.com") {
                    repo::install_from_url(&package, global)?;
                } else {
                    repo::install_package(&package, global, repo.as_deref())?;
                }
            }
            PkgCommands::Remove { package, global } => {
                repo::remove_package(&package, global)?;
            }
            PkgCommands::Installed { global } => {
                repo::list_installed(global)?;
            }
        },
        Some(Commands::Ingest(ingest_cmd)) => match ingest_cmd {
            IngestCommands::Run {
                path,
                area,
                topic,
                languages,
                watch,
            } => {
                ingest::run_ingest(&path, &area, topic.as_deref(), languages.as_deref(), watch)?;
                if get_show_platypus() {
                    platypus::happy();
                }
            }
            IngestCommands::Watch { path: _, topic: _ } => {
                let config = crate::fs::config::get_ingest_config()?;
                if config.auto_index {
                    println!("🔄 Live auto-indexing is enabled in .agent/config.toml");
                    println!("   Changes to indexed files will be tracked automatically");
                } else {
                    println!("🔄 To enable auto-indexing, set in .agent/config.toml:");
                    println!("   [ingest]");
                    println!("   auto_index = true");
                }
            }
            IngestCommands::Recursive { path, area } => {
                let result = ingest::run_ingest_recursive(&path, &area)?;
                println!("{}", result);
            }
            IngestCommands::Stop => {
                println!("🛑 Stopping file watcher...");
            }
        },
        Some(Commands::Parse(parse_cmd)) => match parse_cmd {
            ParseCommands::File {
                path,
                language,
                item_type,
                pattern,
                json,
            } => {
                let result = crate::agent::code_parser::parse_file_to_json(
                    &path,
                    language.as_deref(),
                    &item_type,
                    pattern.as_deref(),
                )?;
                if json {
                    println!("{}", result);
                } else {
                    println!("{}", result);
                }
            }
        },
        Some(Commands::Auto(auto_cmd)) => match auto_cmd {
            AutoCommands::Build {
                topic,
                area,
                spec_file: _,
            } => {
                let result =
                    crate::agent::auto::build::run_auto_build(topic.as_deref(), area.as_deref(), None)?;
                println!("Build result: {:?}", result);
            }
            AutoCommands::Verify { topic, area } => {
                let result = crate::agent::auto::verify::run_auto_verify(&topic, area.as_deref())?;
                println!("Verification result: {:?}", result);
            }
            AutoCommands::Test {
                topic,
                pre_script,
                post_script,
            } => {
                let result = crate::agent::auto::test::run_auto_test(
                    topic.as_deref(),
                    pre_script.as_deref(),
                    post_script.as_deref(),
                )?;
                println!("Test result: {:?}", result);
            }
            AutoCommands::Agent {
                topic,
                session_id,
                parent_topic,
                area,
                workflow,
            } => {
                let result = crate::agent::auto::agent::run_agent(
                    &topic,
                    session_id.as_deref(),
                    parent_topic.as_deref(),
                    area.as_deref(),
                    workflow.as_deref(),
                )?;
                println!("Agent result: {:?}", result);
            }
        },
        Some(Commands::Patty(paddy_cmd)) => match paddy_cmd {
            PattyCommands::Enable => {
                crate::fs::config::set_paddy_enabled(true)?;
                println!("Platypus enabled!");
                platypus::happy();
            }
            PattyCommands::Disable => {
                crate::fs::config::set_paddy_enabled(false)?;
                println!("Platypus disabled.");
                platypus::sad();
            }
            PattyCommands::Status => {
                let enabled = get_show_platypus();
                if enabled {
                    println!("Platypus is enabled");
                } else {
                    println!("Platypus is disabled");
                }
            }
        },
    }

    Ok(())
}
