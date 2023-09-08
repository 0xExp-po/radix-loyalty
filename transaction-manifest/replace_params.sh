#!/bin/bash

# Read parameters from "parameters.txt" and store them in arrays
keys=()
values=()

while IFS='=' read -r key value; do
  keys+=("$key")
  values+=("$value")
done < "manifest_params.txt"

# Function to replace placeholders in a file
replace_placeholders() {
  local file="$1"
  local content
  content=$(<"$file")

  for ((i = 0; i < ${#keys[@]}; i++)); do
    echo "looking at $file and key $key"
    key="${keys[i]}"
    value="${values[i]}"
    content=${content//"<$key>"/"$value"}
  done

  echo "$content" > "$file"
  echo "Replaced placeholders in $file"
}

# Process *.rtm files in the current directory and its subdirectories
find . -type f -name "*.rtm" | while read -r file; do
  replace_placeholders "$file"
done
