# seed completion for fish shell

set -l commands tui workspace archive new init check doctor sync maintain list template-new info tree capture fav pin init-templates icons completions help
set -l templates coding cli rust fastapi react typescript python go cpp c bash data-science docker cad 3dmodel logo vector-artwork webdesign photo-edit video-edit music podcast game presentation book research-paper newsletter portfolio client-project manual

complete -c seed -f

# Main commands
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a tui -d "Launch interactive TUI dashboard"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a workspace -d "Initialize complete master workspace structure"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a archive -d "Archive project to archive/<year>/"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a new -d "Create and scaffold new project"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a init -d "Interactive project wizard"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a check -d "Audit project health & missing files"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a doctor -d "Audit project health & missing files"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a sync -d "Sync and repair project files"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a maintain -d "Sync and repair project files"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a list -d "List available templates"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a template-new -d "Create custom manual template"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a info -d "Show template details"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a tree -d "Visualize directory tree"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a capture -d "Capture directory to template"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a fav -d "Toggle pinned favorite template"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a init-templates -d "Initialize templates directory"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a icons -d "Configure icon mode (nerdfont/ascii)"
complete -c seed -n "not __fish_seen_subcommand_from $commands" -a completions -d "Generate shell completions"

# Subcommand completions
complete -c seed -n "__fish_seen_subcommand_from completions" -a "bash zsh fish powershell"
complete -c seed -n "__fish_seen_subcommand_from icons" -a "nerdfont ascii toggle"
complete -c seed -n "__fish_seen_subcommand_from info" -a "$templates"
complete -c seed -n "__fish_seen_subcommand_from new" -s t -l template -a "$templates" -d "Template"
complete -c seed -n "__fish_seen_subcommand_from new" -s c -l category -a "personal client resource" -d "Category"
complete -c seed -n "__fish_seen_subcommand_from archive check doctor sync maintain tree capture" -a "(__fish_complete_directories)"
