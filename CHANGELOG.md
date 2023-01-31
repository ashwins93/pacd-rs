# v0.3.0-alpha

## Features

- Optionally accept tar and tar.gz files as input.
- New CLI commands - `pacd build` and `pacd pack`.
- Create `tar.gz` archives with the pack command.

# v0.2.1-alpha

## Fixes

- Fixed CLI crashing on template errors

# v0.2.0 Alpha

## Features

- Generate sites by running `pacd [OPTIONS] <SITE_DIR>`
- Enable watch mode with `pacd --watch`
- File names with the pattern `[collection].liquid` are expanded by looking for
  arrays in the data JSON file with the key `collection`.
