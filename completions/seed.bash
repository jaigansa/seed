# seed completion for bash
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
