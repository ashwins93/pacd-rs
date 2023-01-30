# v0.2.1-alpha

## Fixes

- Fixed CLI crashing on template errors

# v0.2.0 Alpha

## Features

- Generate sites by running `pacd [OPTIONS] <SITE_DIR>`
- Enable watch mode with `pacd --watch`
- File names with the pattern `[collection].liquid` are expanded by looking for
  arrays in the data JSON file with the key `collection`.
