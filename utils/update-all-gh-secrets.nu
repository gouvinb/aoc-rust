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

  $args_pair_list | par-each -k { |args|
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

  let script_lines = $args_pair_list | par-each -k { |args|
    $"echo \"$($args.var_name)\" > ($args.path)"
  } | insert 0 "" | insert 0 "#!/usr/bin/env sh"

  $script_lines | to text | save -f "utils/prepare_inputs_and_responses.sh"
}
