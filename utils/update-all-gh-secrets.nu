#!/usr/bin/env nu

### Init

$env.config.table.show_empty = false

### src

# Update all gh secrets
def main [] {
  let file_list = (ls **/*.txt | where $it.name =~ "inputs|responses")
  let file_name_list = ($file_list | par-each -k { |it| $it.name })
  let path_array_list = ($file_name_list | par-each -k { |path| $path | split row "/" })
  let args_pair_list = ($path_array_list | par-each -k { |row|
    {
      var_name: $"($row.0 | str upcase | str substring 0..($in | str length | $in - 1))_($row.1 | str upcase)_($row.2 | str replace ".txt" "" | str upcase)",
      path: ($row | str join "/")
    }
  })

  let clean_github_action_secrets_path_script = "utils/clean_github_action_secrets.nu"

  if ($clean_github_action_secrets_path_script | path exists) {
    nu $clean_github_action_secrets_path_script
  } else {
    echo $"Skip \"($clean_github_action_secrets_path_script)\""
  }

  $args_pair_list | each { |args|
    gh secret set $"($args.var_name)" --body $"(cat $args.path)"
  }

  open .github/workflows/rust.yml |
    upsert jobs.build.env {} |
    to yaml |
    save -f .github/workflows/rust.yml

  $args_pair_list | each { |args|
    open .github/workflows/rust.yml |
      upsert jobs.build.env { |yml|
        $yml | get jobs.build.env | insert $"($args.var_name)" $"${{ secrets.($args.var_name) }}"
      } |
      to yaml |
      save -f .github/workflows/rust.yml
  }

  let $script_clean_lines = $args_pair_list |
    par-each -k { |args|
      $"  \"($args.var_name)\""
    } |
    insert 0 "#!/usr/bin/env nu" |
    insert 1 "" |
    insert 2 "$env.config.table.show_empty = false" |
    insert 3 "" |
    insert 4 "[" |
    append "] | each { |$it| gh secret delete $it }"

  $script_clean_lines | to text | save -f $clean_github_action_secrets_path_script


  let script_prepare_lines = $args_pair_list | par-each -k { |args|
    $"echo \"$($args.var_name)\" > ($args.path)"
  } | insert 0 "#!/usr/bin/env sh" | insert 1 ""

  $script_prepare_lines | to text | save -f "utils/prepare_inputs_and_responses.sh"
}
