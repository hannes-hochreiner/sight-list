#!/usr/bin/env -S nu --stdin
use std/log

def main [] {}

def "main vendor" [
  src: string
] {
  augment_path
  print $env
  let out = $env.out
  let cargo_home = $"($out)/cargo_home"

  cd $src
  mkdir $cargo_home
  CARGO_HOME=$cargo_home cargo vendor $out -q
}

def "main build" [
  src: string
  deps: string
  package: string
] {
  augment_path
  print $env

  let out = $env.out
  let cargo_target = $"($out)/cargo_target"
  let cargo_home = $"($out)/cargo_home"

  mkdir $cargo_home

  {
    "source": {
      "crates-io": {
        "replace-with": "vendored-sources"
      }
      "vendored-sources": {
        "directory": $deps
      }
    }
  } | to toml | save $"($cargo_home)/config.toml"

  cd $src
  mkdir $cargo_target
  mkdir $"($out)/bin"
  CARGO_HOME=$cargo_home CARGO_TARGET_DIR=$cargo_target cargo build --release --offline --frozen --verbose
  cp $"($out)/cargo_target/release/($package)" $"($out)/bin/($package)"

  mkdir $"($out)/var"
  mkdir $"($out)/var/templates"
  mkdir $"($out)/var/static"
  log info "copying templates"
  cp -r templates $"($out)/var"
  log info "copying static"
  cp -r static $"($out)/var"
  log info "build complete"

  if ($cargo_home | path exists) {
    rm -r $cargo_home
  }
  rm -r $cargo_target
}

def --env augment_path [] {
  $env.PATH = [
    ...$env.PATH
    ...($env.buildInputs | split row -r '\s+' | each {|item| $"($item)/bin"})
  ]
}