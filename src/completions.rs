//! Shell completion script generators for bash, zsh, fish, and powershell

pub fn generate_bash_completions() -> String {
    r#"# seed completion for bash
_seed_completions() {
    local cur prev words cword
    _init_completion || return

    local commands="tui workspace archive new init check doctor sync maintain list template-new info tree capture fav pin init-templates icons completions help"
    local templates="coding cli rust fastapi react typescript python go cpp c bash data-science docker cad 3dmodel logo vector-artwork webdesign photo-edit video-edit music podcast game presentation book research-paper newsletter portfolio client-project manual"

    case "${prev}" in
        seed)
            COMPREPLY=($(compgen -W "${commands}" -- "${cur}"))
            return 0
            ;;
        -t|--template)
            COMPREPLY=($(compgen -W "${templates}" -- "${cur}"))
            return 0
            ;;
        -c|--category)
            COMPREPLY=($(compgen -W "personal client resource archive" -- "${cur}"))
            return 0
            ;;
        completions)
            COMPREPLY=($(compgen -W "bash zsh fish powershell" -- "${cur}"))
            return 0
            ;;
        icons)
            COMPREPLY=($(compgen -W "nerdfont ascii toggle" -- "${cur}"))
            return 0
            ;;
        info)
            COMPREPLY=($(compgen -W "${templates}" -- "${cur}"))
            return 0
            ;;
        archive|check|doctor|sync|maintain|tree|capture)
            _filedir -d
            return 0
            ;;
    esac

    if [[ "${cur}" == -* ]]; then
        COMPREPLY=($(compgen -W "--help --version -t --template -c --category --client --dir --git --no-git" -- "${cur}"))
    else
        COMPREPLY=($(compgen -W "${commands}" -- "${cur}"))
    fi
}

complete -F _seed_completions seed
"#.to_string()
}

pub fn generate_zsh_completions() -> String {
    r#"#compdef seed

_seed() {
    local -a commands
    commands=(
        'tui:Launch the interactive full-screen Terminal User Interface'
        'workspace:Initialize the complete master workspace structure'
        'archive:Archive a completed project into archive/<year>/'
        'new:Create a new project directory and scaffold files'
        'init:Interactively initialize a project in current or new directory'
        'check:Audit and check project structure and health score'
        'doctor:Audit and check project structure and health score'
        'sync:Maintain, sync, and repair project files and directories'
        'maintain:Maintain, sync, and repair project files and directories'
        'list:List all available built-in preset and manual templates'
        'template-new:Create a new manual template with custom directories and files'
        'info:Show detailed contents and structure of a template'
        'tree:Visualize directory tree with file details and stats'
        'capture:Capture an existing directory structure into a template'
        'fav:Manage favorite templates pinned to the top of lists'
        'pin:Pin or unpin a project to the top of the workspace dashboard'
        'init-templates:Initialize template folders'
        'icons:Configure or toggle icon display mode (nerdfont or ascii)'
        'completions:Generate shell completion script (bash, zsh, fish, powershell)'
        'help:Print help message'
    )

    local -a templates
    templates=(
        coding cli rust fastapi react typescript python go cpp c bash
        data-science docker cad 3dmodel logo vector-artwork webdesign
        photo-edit video-edit music podcast game presentation book
        research-paper newsletter portfolio client-project manual
    )

    _arguments -C \
        '1: :->command' \
        '*:: :->args'

    case $state in
        command)
            _describe -t commands 'seed commands' commands
            ;;
        args)
            case $words[1] in
                new)
                    _arguments \
                        '--template=[Template to use]:template:($templates)' \
                        '-t[Template to use]:template:($templates)' \
                        '--category=[Project category]:category:(personal client resource)' \
                        '-c[Project category]:category:(personal client resource)' \
                        '--client=[Client name]:client:' \
                        '--dir=[Custom output directory]:directory:_files -/' \
                        '--git[Initialize git repository]' \
                        '--no-git[Do not initialize git repository]' \
                        '*:Project name:'
                    ;;
                info)
                    _arguments '1:Template:($templates)'
                    ;;
                icons)
                    _arguments '1:Mode:(nerdfont ascii toggle)'
                    ;;
                completions)
                    _arguments '1:Shell:(bash zsh fish powershell)'
                    ;;
                archive|check|doctor|sync|maintain|tree|capture)
                    _files -/
                    ;;
                *)
                    _files
                    ;;
            esac
            ;;
    esac
}

_seed "$@"
"#.to_string()
}

pub fn generate_fish_completions() -> String {
    r#"# seed completion for fish shell

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
"#.to_string()
}

pub fn generate_powershell_completions() -> String {
    r#"# seed completion for PowerShell
Register-ArgumentCompleter -Native -CommandName seed -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)
    $commands = @('tui', 'workspace', 'archive', 'new', 'init', 'check', 'doctor', 'sync', 'maintain', 'list', 'template-new', 'info', 'tree', 'capture', 'fav', 'pin', 'init-templates', 'icons', 'completions', 'help')
    $templates = @('coding', 'cli', 'rust', 'fastapi', 'react', 'typescript', 'python', 'go', 'cpp', 'c', 'bash', 'data-science', 'docker', 'cad', '3dmodel', 'logo', 'vector-artwork', 'webdesign', 'photo-edit', 'video-edit', 'music', 'podcast', 'game', 'presentation', 'book', 'research-paper', 'newsletter', 'portfolio', 'client-project', 'manual')

    $elements = $commandAst.Elements
    if ($elements.Count -le 2) {
        $commands | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
            [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
        }
    }
}
"#.to_string()
}

pub fn generate_completions(shell: &str) -> Option<String> {
    match shell.to_lowercase().as_str() {
        "bash" => Some(generate_bash_completions()),
        "zsh" => Some(generate_zsh_completions()),
        "fish" => Some(generate_fish_completions()),
        "powershell" | "pwsh" => Some(generate_powershell_completions()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completions_generation() {
        assert!(generate_completions("bash").is_some());
        assert!(generate_completions("zsh").is_some());
        assert!(generate_completions("fish").is_some());
        assert!(generate_completions("powershell").is_some());
        assert!(generate_completions("unknown").is_none());

        let bash = generate_bash_completions();
        assert!(bash.contains("complete -F _seed_completions seed"));
        assert!(bash.contains("workspace"));
        assert!(bash.contains("archive"));

        let zsh = generate_zsh_completions();
        assert!(zsh.contains("#compdef seed"));

        let fish = generate_fish_completions();
        assert!(fish.contains("complete -c seed"));
    }
}
