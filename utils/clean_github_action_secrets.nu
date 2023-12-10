#!/usr/bin/env nu

$env.config.table.show_empty = false

[
  "INPUT_2023_01"
  "INPUT_2023_01_1_EXAMPLE"
  "INPUT_2023_01_2_EXAMPLE"
  "INPUT_2023_02"
  "INPUT_2023_02_EXAMPLE"
  "INPUT_2023_03"
  "INPUT_2023_03_EXAMPLE"
  "RESPONSE_2023_01_1_EXAMPLE"
  "RESPONSE_2023_01_2_EXAMPLE"
  "RESPONSE_2023_02_1_EXAMPLE"
  "RESPONSE_2023_02_2_EXAMPLE"
  "RESPONSE_2023_03_EXAMPLE"
] | each { |$it| gh secret delete $it }