#!/usr/bin/env bash

set -e

original_dir="$(pwd)"

script_dir="$(dirname "$0")"

project_root="$(cd "$script_dir/.." && pwd)"

cargo run --example pkce_save_token

token_path="$project_root/token.json"

echo "Token saved to: $token_path"

cd "$original_dir"
