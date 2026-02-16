#!/bin/bash
VERSION="v0.3.0"
BASE_URL="https://github.com/mrorigo/hashfile-mcp/releases/download/$VERSION"
FILES=(
  "hashfile-mcp-aarch64-apple-darwin.tar.gz"
  "hashfile-mcp-x86_64-unknown-linux-gnu.tar.gz"
  "hashfile-mcp-aarch64-unknown-linux-gnu.tar.gz"
  "hashfile-mcp-aarch64-unknown-linux-musl.tar.gz"
  "hashfile-mcp-x86_64-pc-windows-msvc.zip"
)

echo "Computing hashes for $VERSION..."
echo "{"

for i in "${!FILES[@]}"; do
  file="${FILES[$i]}"
  url="$BASE_URL/$file"
  
  # Download
  if curl -L -s -f -o "$file" "$url"; then
     # Compute hash
     hash=$(openssl dgst -sha256 "$file" | awk '{print $NF}')
     # Remove file
     rm "$file"
     
     # Print JSON line
     comma=""
     if [ $i -lt $((${#FILES[@]}-1)) ]; then comma=","; fi
     echo "  \"$file\": \"$hash\"$comma"
  else
     echo "  \"$file\": \"ERROR: Failed to download (Release exists?)\"" >&2
  fi
done

echo "}"
