<h1 align=center>⚡<code>vlt</code>⚡</h1>

<div align=center>
  <a href=https://crates.io/dieselftw/vlt>
    <img src=https://img.shields.io/crates/v/vlt.svg alt="crates.io version">
  </a>
    
  <a href=https://github.com/dieselftw/vlt/.github/workflows/ci.yaml>
    <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/dieselftw/vlt/.github%2Fworkflows%2Fci.yml">
  </a>
</div>

<h3 align=center> Rust CLI for managing .env files across environments.
<br><br>

<code>cargo install vlt</code>
</h3>

<div align="center">
  <img src="https://github.com/user-attachments/assets/52bbfd04-0981-4905-8bf9-6b2a4fbd77bc" alt="vlt demo" width="650" />
</div>

## Using vlt

Run `vlt` from the root of the project whose environment files you want to manage.

### 1. Initialize a project

```bash
vlt init
```

This sets up `.vlt/`, adds `vlt` to `.gitignore`, and then asks for the first setup step:

- `Scan all variables`
- `Skip for now`

If you scan right away, `vlt` walks the codebase immediately and builds `.env.base`.
If you skip, no environment template is created yet.

### 2. Scan source code for env vars

```bash
vlt scan
vlt scan --apply
```

Approved variables are added to:

- `.env.base` first
- `.vlt/env.rules` as discovered string rules so the rest of the CLI can keep working

`--apply` skips the selector and adds every discovered missing variable automatically.

### 3. Create another environment

```bash
vlt create staging
vlt create prod
```

### 4. Generate an .env.example file

```bash
vlt generate
```

This writes `.env.example` with all known keys and comment metadata from `.vlt/env.rules`.

### 5. Activate an environment

```bash
vlt use dev
vlt use staging
```

This copies `.vlt/env.<name>` to `.env` and records the active environment in `.vlt/config.toml`.

### 6. Inspect status

```bash
vlt status
```

This shows:

- the active environment
- available environments
- missing values in the active environment
- drift between `.env` and `.vlt/env.rules`

### 7. Compare environments

```bash
vlt diff dev prod
```

This compares keys only and never prints secret values.

### 8. Sync missing keys

```bash
vlt sync prod staging
```

This adds keys that exist in the source environment but not in the target environment. New keys are added as blank placeholders so you can fill them safely.

### 9. Validate the active `.env`

```bash
vlt validate
```

This checks required values, booleans, ints, floats, and enums against `.vlt/env.rules`.

### 10. Import or export environment files

```bash
vlt import staging ./ops/staging.env
vlt export staging ./exports/staging.env
```

`import` loads values from an existing env file into `.vlt/env.<name>` and updates project templates for any new keys.
`export` writes `.vlt/env.<name>` to a standalone env file at the path you choose.
