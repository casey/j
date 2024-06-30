use super::*;

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq)]
pub(crate) enum Shell {
  Bash,
  Elvish,
  Fish,
  #[value(alias = "nu")]
  Nushell,
  Powershell,
  Zsh,
}

impl Shell {
  pub(crate) fn script(self) -> RunResult<'static, String> {
    match self {
      Self::Bash => completions::clap(clap_complete::Shell::Bash),
      Self::Elvish => completions::clap(clap_complete::Shell::Elvish),
      Self::Fish => completions::clap(clap_complete::Shell::Fish),
      Self::Nushell => Ok(completions::NUSHELL_COMPLETION_SCRIPT.into()),
      Self::Powershell => completions::clap(clap_complete::Shell::PowerShell),
      Self::Zsh => completions::clap(clap_complete::Shell::Zsh),
    }
  }
}

fn clap(shell: clap_complete::Shell) -> RunResult<'static, String> {
  fn replace(haystack: &mut String, needle: &str, replacement: &str) -> RunResult<'static, ()> {
    if let Some(index) = haystack.find(needle) {
      haystack.replace_range(index..index + needle.len(), replacement);
      Ok(())
    } else {
      Err(Error::internal(format!(
        "Failed to find text:\n{needle}\n…in completion script:\n{haystack}"
      )))
    }
  }

  let mut script = {
    let mut tempfile = tempfile().map_err(|io_error| Error::TempfileIo { io_error })?;

    clap_complete::generate(
      shell,
      &mut crate::config::Config::app(),
      env!("CARGO_PKG_NAME"),
      &mut tempfile,
    );

    tempfile
      .rewind()
      .map_err(|io_error| Error::TempfileIo { io_error })?;

    let mut buffer = String::new();

    tempfile
      .read_to_string(&mut buffer)
      .map_err(|io_error| Error::TempfileIo { io_error })?;

    buffer
  };

  match shell {
    clap_complete::Shell::Bash => {
      for (needle, replacement) in completions::BASH_COMPLETION_REPLACEMENTS {
        replace(&mut script, needle, replacement)?;
      }
    }
    clap_complete::Shell::Fish => {
      script.insert_str(0, completions::FISH_RECIPE_COMPLETIONS);
    }
    clap_complete::Shell::PowerShell => {
      for (needle, replacement) in completions::POWERSHELL_COMPLETION_REPLACEMENTS {
        replace(&mut script, needle, replacement)?;
      }
    }
    clap_complete::Shell::Zsh => {
      for (needle, replacement) in completions::ZSH_COMPLETION_REPLACEMENTS {
        replace(&mut script, needle, replacement)?;
      }
    }
    _ => {}
  }

  Ok(script.trim().into())
}

const NUSHELL_COMPLETION_SCRIPT: &str = r#"def "nu-complete just" [] {
    (^just --dump --unstable --dump-format json | from json).recipes | transpose recipe data | flatten | where {|row| $row.private == false } | select recipe doc parameters | rename value description
}

# Just: A Command Runner
export extern "just" [
    ...recipe: string@"nu-complete just", # Recipe(s) to run, may be with argument(s)
]"#;

const FISH_RECIPE_COMPLETIONS: &str = r#"function __fish_just_complete_recipes
        if string match -rq '(-f|--justfile)\s*=?(?<justfile>[^\s]+)' -- (string split -- ' -- ' (commandline -pc))[1]
          set -fx JUST_JUSTFILE "$justfile"
        end
        just --list 2> /dev/null | tail -n +2 | awk '{
        command = $1;
        args = $0;
        desc = "";
        delim = "";
        sub(/^[[:space:]]*[^[:space:]]*/, "", args);
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", args);

        if (match(args, /#.*/)) {
          desc = substr(args, RSTART+2, RLENGTH);
          args = substr(args, 0, RSTART-1);
          gsub(/^[[:space:]]+|[[:space:]]+$/, "", args);
        }

        gsub(/\+|=[`\'"][^`\'"]*[`\'"]/, "", args);
        gsub(/ /, ",", args);

        if (args != ""){
          args = "Args: " args;
        }

        if (args != "" && desc != "") {
          delim = "; ";
        }

        print command "\t" args delim desc
  }'
end

# don't suggest files right off
complete -c just -n "__fish_is_first_arg" --no-files

# complete recipes
complete -c just -a '(__fish_just_complete_recipes)'

# autogenerated completions
"#;

const ZSH_COMPLETION_REPLACEMENTS: &[(&str, &str)] = &[
  (
    r#"    _arguments "${_arguments_options[@]}" : \"#,
    r"    local common=(",
  ),
  (
    r"'*--set=[Override <VARIABLE> with <VALUE>]:VARIABLE: :VARIABLE: ' \",
    r"'*--set=[Override <VARIABLE> with <VALUE>]: :(_just_variables)' \",
  ),
  (
    r"'()-s+[Show recipe at <PATH>]:PATH: ' \
'()--show=[Show recipe at <PATH>]:PATH: ' \",
    r"'-s+[Show recipe at <PATH>]: :(_just_commands)' \
'--show=[Show recipe at <PATH>]: :(_just_commands)' \",
  ),
  (
    "'*::ARGUMENTS -- Overrides and recipe(s) to run, defaulting to the first recipe in the \
     justfile:' \\
&& ret=0",
    r#")

    _arguments "${_arguments_options[@]}" $common \
        '1: :_just_commands' \
        '*: :->args' \
        && ret=0

    case $state in
        args)
            curcontext="${curcontext%:*}-${words[2]}:"

            local lastarg=${words[${#words}]}
            local recipe

            local cmds; cmds=(
                ${(s: :)$(_call_program commands just --summary)}
            )

            # Find first recipe name
            for ((i = 2; i < $#words; i++ )) do
                if [[ ${cmds[(I)${words[i]}]} -gt 0 ]]; then
                    recipe=${words[i]}
                    break
                fi
            done

            if [[ $lastarg = */* ]]; then
                # Arguments contain slash would be recognised as a file
                _arguments -s -S $common '*:: :_files'
            elif [[ $lastarg = *=* ]]; then
                # Arguments contain equal would be recognised as a variable
                _message "value"
            elif [[ $recipe ]]; then
                # Show usage message
                _message "`just --show $recipe`"
                # Or complete with other commands
                #_arguments -s -S $common '*:: :_just_commands'
            else
                _arguments -s -S $common '*:: :_just_commands'
            fi
        ;;
    esac

    return ret
"#,
  ),
  (
    "    local commands; commands=()",
    r#"    [[ $PREFIX = -* ]] && return 1
    integer ret=1
    local variables; variables=(
        ${(s: :)$(_call_program commands just --variables)}
    )
    local commands; commands=(
        ${${${(M)"${(f)$(_call_program commands just --list)}":#    *}/ ##/}/ ##/:Args: }
    )
"#,
  ),
  (
    r#"    _describe -t commands 'just commands' commands "$@""#,
    r#"    if compset -P '*='; then
        case "${${words[-1]%=*}#*=}" in
            *) _message 'value' && ret=0 ;;
        esac
    else
        _describe -t variables 'variables' variables -qS "=" && ret=0
        _describe -t commands 'just commands' commands "$@"
    fi
"#,
  ),
  (
    r#"_just "$@""#,
    r#"(( $+functions[_just_variables] )) ||
_just_variables() {
    [[ $PREFIX = -* ]] && return 1
    integer ret=1
    local variables; variables=(
        ${(s: :)$(_call_program commands just --variables)}
    )

    if compset -P '*='; then
        case "${${words[-1]%=*}#*=}" in
            *) _message 'value' && ret=0 ;;
        esac
    else
        _describe -t variables 'variables' variables && ret=0
    fi

    return ret
}

_just "$@""#,
  ),
];

const POWERSHELL_COMPLETION_REPLACEMENTS: &[(&str, &str)] = &[(
  r#"$completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText"#,
  r#"function Get-JustFileRecipes([string[]]$CommandElements) {
        $justFileIndex = $commandElements.IndexOf("--justfile");

        if ($justFileIndex -ne -1 -and $justFileIndex + 1 -le $commandElements.Length) {
            $justFileLocation = $commandElements[$justFileIndex + 1]
        }

        $justArgs = @("--summary")

        if (Test-Path $justFileLocation) {
            $justArgs += @("--justfile", $justFileLocation)
        }

        $recipes = $(just @justArgs) -split ' '
        return $recipes | ForEach-Object { [CompletionResult]::new($_) }
    }

    $elementValues = $commandElements | Select-Object -ExpandProperty Value
    $recipes = Get-JustFileRecipes -CommandElements $elementValues
    $completions += $recipes
    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText"#,
)];

const BASH_COMPLETION_REPLACEMENTS: &[(&str, &str)] = &[
  (
    r#"            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi"#,
    r#"                if [[ ${cur} == -* ]] ; then
                    COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                    return 0
                elif [[ ${COMP_CWORD} -eq 1 ]]; then
                    local recipes=$(just --summary 2> /dev/null)

                    if echo "${cur}" | \grep -qF '/'; then
                        local path_prefix=$(echo "${cur}" | sed 's/[/][^/]*$/\//')
                        local recipes=$(just --summary 2> /dev/null -- "${path_prefix}")
                        local recipes=$(printf "${path_prefix}%s\t" $recipes)
                    fi

                    if [[ $? -eq 0 ]]; then
                        COMPREPLY=( $(compgen -W "${recipes}" -- "${cur}") )
                        return 0
                    fi
                fi"#,
  ),
  (
    r"local i cur prev opts cmd",
    r"local i cur prev words cword opts cmd",
  ),
  (
    r#"    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}""#,
    r#"
    # Modules use "::" as the separator, which is considered a wordbreak character in bash.
    # The _get_comp_words_by_ref function is a hack to allow for exceptions to this rule without
    # modifying the global COMP_WORDBREAKS environment variable.
    if type _get_comp_words_by_ref &>/dev/null; then
        _get_comp_words_by_ref -n : cur prev words cword
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
        prev="${COMP_WORDS[COMP_CWORD-1]}"
        words=$COMP_WORDS
        cword=$COMP_CWORD
    fi
"#,
  ),
  (r"for i in ${COMP_WORDS[@]}", r"for i in ${words[@]}"),
  (
    r"elif [[ ${COMP_CWORD} -eq 1 ]]; then",
    r"elif [[ ${cword} -eq 1 ]]; then",
  ),
  (
    r#"COMPREPLY=( $(compgen -W "${recipes}" -- "${cur}") )"#,
    r#"COMPREPLY=( $(compgen -W "${recipes}" -- "${cur}") )
                        if type __ltrim_colon_completions &>/dev/null; then
                            __ltrim_colon_completions "$cur"
                        fi"#,
  ),
];
