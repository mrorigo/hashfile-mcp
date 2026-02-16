# Publishing Hashfile MCP to Registry

This guide explains how to publish `hashfile-mcp` to the [MCP Registry](https://github.com/modelcontextprotocol/registry).

## Prerequisites

- [mcp-publisher](https://github.com/modelcontextprotocol/registry) tool installed
- GitHub account
- Existing `server.json` (created in this repo)

## Step 1: Create GitHub Release

Since the registry points to GitHub release assets, you must first create the release:

1.  Push the `v0.3.0` tag to GitHub:
    ```bash
    git push origin v0.3.0
    ```

2.  Create a new Release on GitHub for `v0.3.0`.

3.  Upload the following binary assets to the release (built via CI or locally):
    - `hashfile-mcp-x86_64-apple-darwin.tar.gz`
    - `hashfile-mcp-aarch64-apple-darwin.tar.gz`
    - `hashfile-mcp-x86_64-unknown-linux-gnu.tar.gz`
    - `hashfile-mcp-aarch64-unknown-linux-gnu.tar.gz`
    - `hashfile-mcp-x86_64-pc-windows-msvc.zip`

   > **Note**: Ensure these filenames match exactly what is in `server.json`.

## Step 2: Compute SHA-256 Hashes

The registry requires the SHA-256 hash of each asset for integrity verification.

1.  Download the assets you just uploaded.
2.  Compute the hash for each file:

    ```bash
    openssl dgst -sha256 hashfile-mcp-*.tar.gz
    ```

    Or use the provided helper script (ensure you invoke it after the release is public):

    ```bash
    ./get_hashes.sh
    ```

## Step 3: Update server.json

Open `server.json` and replace the placeholder hashes (`e3b0c442...`) with the actual hashes you computed in Step 2.

```json
{
  "identifier": ".../hashfile-mcp-x86_64-apple-darwin.tar.gz",
  "fileSha256": "YOUR_COMPUTED_HASH_HERE"
}
```

Repeat for all 5 packages.

## Step 4: Login to Registry

Authenticate with the registry using your GitHub account:

```bash
mcp-publisher login github
```

Follow the on-screen instructions to authorize the application.

## Step 5: Publish

Publish your server metadata to the registry:

```bash
mcp-publisher publish
```

If successful, you will see:
```
✓ Successfully published
✓ Server io.github.mrorigo/hashfile version 0.3.0
```

## Step 6: Verify

Check if your server is listed:

```bash
curl "https://registry.modelcontextprotocol.io/v0.1/servers?search=io.github.mrorigo/hashfile"
```