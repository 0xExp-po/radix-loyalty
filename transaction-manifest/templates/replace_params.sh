#!/bin/bash

# Read parameters from "parameters.txt" and store them in arrays
keys=()
values=()

while IFS='=' read -r key value; do
  keys+=("$key")
  values+=("$value")
done < "manifest_params.txt"

output_dir="../manifests"
mkdir -p "$output_dir"
rm -f "$output_dir"/*

# Function to replace placeholders in a file
replace_placeholders() {
  local file="$1"
  local content
  content=$(<"$file")

  for ((i = 0; i < ${#keys[@]}; i++)); do
    key="${keys[i]}"
    echo "looking at $file and key $key"
    value="${values[i]}"
    content=${content//"<$key>"/"$value"}
  done

  echo "$content" > "$output_dir/$file"
  echo "Replaced placeholders in $output_dir/$file"
}

# Process *.rtm files in the current directory and its subdirectories
find . -type f -name "*.rtm" | while read -r file; do
  replace_placeholders "$file"
done