# Contributing

We welcome contributions to this project. By contributing, you agree to the
terms outlined below.

## Commit Message Format

HyperI projects use [Conventional Commits](https://www.conventionalcommits.org/)
and [semantic-release](https://semantic-release.gitbook.io/) for automated
versioning and changelog generation. All commits must follow this format:

```
<type>(<scope>): <subject>

[optional body]

[optional footer(s)]
```

### Types

| Type | Description | Version Bump |
|------|-------------|--------------|
| `feat` | A new feature | Minor (0.X.0) |
| `fix` | A bug fix | Patch (0.0.X) |
| `docs` | Documentation only | None |
| `style` | Code style (formatting, semicolons, etc.) | None |
| `refactor` | Code change that neither fixes a bug nor adds a feature | None |
| `perf` | Performance improvement | Patch (0.0.X) |
| `test` | Adding or correcting tests | None |
| `build` | Changes to build system or dependencies | None |
| `ci` | Changes to CI configuration | None |
| `chore` | Other changes that don't modify src or test files | None |
| `revert` | Reverts a previous commit | Varies |

### Breaking Changes

For breaking changes that require a major version bump, add `!` after the type
or include `BREAKING CHANGE:` in the footer:

```
feat!: remove deprecated API endpoints

BREAKING CHANGE: The /v1/users endpoint has been removed. Use /v2/users instead.
```

### Examples

```
feat(auth): add OAuth2 support for Google login

fix(api): handle null response from upstream service

docs: update installation instructions for Windows

refactor(core)!: restructure module exports

BREAKING CHANGE: Named exports are now used instead of default exports.
```

### Scope

Scope is optional but recommended. Use it to indicate the area of the codebase
affected (e.g., `api`, `auth`, `core`, `cli`, `docs`).

## Semantic Versioning

This project follows [Semantic Versioning 2.0.0](https://semver.org/):

- **MAJOR** (X.0.0): Breaking changes that require users to modify their code
- **MINOR** (0.X.0): New features that are backwards-compatible
- **PATCH** (0.0.X): Bug fixes and minor improvements

Versions are automatically determined by semantic-release based on commit
messages. Do not manually update version numbers.

## Developer Certificate of Origin

This project uses the Developer Certificate of Origin (DCO) to ensure that
contributors have the right to submit their contributions.

By making a contribution to this project, you certify that:

1. The contribution was created in whole or in part by you and you have the
   right to submit it under the license indicated in the file; or

2. The contribution is based upon previous work that, to the best of your
   knowledge, is covered under an appropriate open source license and you
   have the right under that license to submit that work with modifications;
   or

3. The contribution was provided directly to me by some other person who
   certified (1), (2) or (3) and you have not modified it.

4. You understand and agree that this project and the contribution are public
   and that a record of the contribution (including all personal information
   you submit with it, including your sign-off) is maintained indefinitely
   and may be redistributed consistent with this project or the license(s)
   involved.

## How to Sign Off Your Commits

You must sign off each commit to indicate your acceptance of the DCO. Combine
the signoff with your conventional commit message:

```
git commit --signoff -m "feat(auth): add two-factor authentication"
```

This produces:

```
feat(auth): add two-factor authentication

Signed-off-by: Your Name <your.email@example.com>
```

Make sure your Git configuration has your correct name and email:

```
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

## License for Contributions

All contributions to this project are licensed under the Business Source
License 1.1 (BUSL-1.1), the same license that covers the project.

Each version of the software (including your contributions) will automatically
become available under the Apache License, Version 2.0 on the third
anniversary of its release.

## How to Contribute

1. **Fork the repository** and create your branch from `main`
2. **Make your changes** following the commit message format above
3. **Sign off your commits** with the DCO
4. **Test your changes** to ensure they work as expected
5. **Submit a pull request** with a clear description of what you've done

### Pull Request Checklist

- [ ] Commits follow the conventional commit format
- [ ] All commits are signed off (DCO)
- [ ] Tests pass (if applicable)
- [ ] Documentation is updated (if applicable)

## CI/CD Workflow

When your pull request is merged to `main`:

1. **semantic-release** analyses commit messages since the last release
2. Determines the next version number based on commit types
3. Generates/updates the CHANGELOG
4. Creates a new GitHub release with release notes
5. Publishes the package (if applicable)

This happens automatically - no manual intervention required.

## CI is slow here, so iterate off it

**A push to find out what CI says costs the better part of an hour.** Treat CI
as the gate that proves a finished change, never as the type checker.

Measured on this repo, per release run:

| Stage | Elapsed |
|---|---|
| Quality | ~18 min |
| Build, linux-amd64 | ~30 min |
| Build, linux-arm64 | ~6-16% longer again than amd64 |

The cause is this repo's shape rather than a misconfiguration. `dfe-transforms`
carries one module per data stream, over two thousand of them, so it compiles as
a single `rustc` holding about 13 GiB whatever `-j` says, and the binary then
links under fat LTO in one thread. Neither step spreads across cores, which is
why a bigger runner buys very little.

**It gets worse when PGO and BOLT land.** That pipeline is not one build with a
flag -- it is four sequential cargo passes (PGO instrument, PGO optimise, BOLT
instrument, BOLT optimise) plus two fixed 300-second workload runs, and a failed
BOLT attempt retries the pair. On a binary whose single pass already takes 30
minutes that projects to roughly two hours per architecture, and about three if
the retry fires.

### What to run instead

Everything below runs locally and answers in seconds:

```bash
# The same gates CI runs, before pushing.
hyperi-ci check

# One source against the corpus, ~6s once built. The name is a corpus directory
# (okta, cisco_ios, panw) and must match exactly. A filtered run SKIPS both
# ratchets, so it tells you about that source and nothing else.
DFE_COMPAT_ONLY=okta cargo test -p dfe-transforms --test compat_corpus

# The whole corpus, ~18s once built. This is the one that moves the ratchets.
cargo test -p dfe-transforms --test compat_corpus
```

Keep a `bacon` session running while you edit -- it recompiles on save and gives
verdicts without you spending a CI run on them. The first build of the workspace
is cold and takes around twenty minutes, and every one after it is incremental.

Two traps worth knowing before you burn a run on them:

- **Never set `CARGO_BUILD_JOBS` by hand.** `~/.local/bin/cargo` derives it, and
  a hand-set value beats both the shim and the project's own pinning.
- **The compat corpus is gitignored and its absence PASSES.** A run in a git
  worktree measures nothing and still reports success, so pass
  `DFE_COMPAT_CORPUS=/projects/dfe-transform-elastic/testdata/compat` there.

## Questions

If you have questions about contributing, please open an issue or contact us.
