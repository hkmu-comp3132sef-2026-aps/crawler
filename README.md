# Hong Kong Schools Crawler

This is a crawler to get schools information in Hong Kong.

## Dependencies

Please install the following dependencies:

| Dependencies                                   | Description                            |
| ---------------------------------------------- | -------------------------------------- |
| [Rust](https://rust-lang.org/)                 | Programming language                   |
| [just](https://just.systems)                   | Command runner                         |
| [ls-lint](https://ls-lint.org/)                | Linting tool for directories and files |
| [typos-cli](https://github.com/crate-ci/typos) | Spell checker                          |

## Commands

The following commands are available:

### Default Command

This command will do linting and formatting.

```sh
just
```

### Linting

This command will lint the code.

```sh
just lint
```

### Formatting

This command will format the code.

```sh
just fmt
```

### Development

This command will start the crawler with `.env.development` file.

```sh
just dev
```

### Test

This command will start the crawler with `.env.test` file.

```sh
just tst
```

### Production

This command will start the crawler with `.env.production` file.

```sh
just prd
```
