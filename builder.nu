#!/usr/bin/env -S nu --stdin
use std/log

def main [
  src: string
] {
}

def "main fetch" [
  src: string
] {
  print $env
  $env.PATH = [
    ...$env.PATH
    ...($env.buildInputs | split row -r '\s+' | each {|item| $"($item)/bin"})
  ]
  let out = $env.out

  cd $src
  mkdir $"($out)/cargo_home"
  CARGO_HOME=$"($out)/cargo_home" cargo fetch
  rm $"($out)/cargo_home/.global-cache"
}

def "main build" [
  src: string,
  deps: string
] {
  print $env
  $env.PATH = [
    ...$env.PATH
    ...($env.buildInputs | split row -r '\s+' | each {|item| $"($item)/bin"})
  ]
  let out = $env.out

  cd $src
  mkdir $"($out)/cargo_target"
  mkdir $"($out)/bin"
  CARGO_HOME=$"($deps)/cargo_home" CARGO_TARGET_DIR=$"($out)/cargo_target" cargo build --release --offline --frozen --verbose
  cp $"($out)/cargo_target/release/sight-list" $"($out)/bin/sight-list"
  rm -r $"($out)/cargo_target"
  mkdir $"($out)/var"
  mkdir $"($out)/var/templates"
  mkdir $"($out)/var/static"
  log info "copying templates"
  cp -r templates $"($out)/var"
  log info "copying static"
  cp -r static $"($out)/var"
  log info "build complete"
}
