# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.4 (2023-02-17)

### Bug Fixes

 - <csr-id-e14dc7d475373d2c266e84ff8f1826c68a34ab92/> note that crates have been renamed from `git-*` to `gix-*`.
   This also means that the `git-*` prefixed crates of the `gitoxide` project
   are effectively unmaintained.
   Use the crates with the `gix-*` prefix instead.
   
   If you were using `git-repository`, then `gix` is its substitute.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 7 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - note that crates have been renamed from `git-*` to `gix-*`. ([`e14dc7d`](https://github.com/Byron/gitoxide/commit/e14dc7d475373d2c266e84ff8f1826c68a34ab92))
</details>

## 0.2.3 (2023-02-09)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 28 calendar days.
 - 31 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#691](https://github.com/Byron/gitoxide/issues/691)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **Uncategorized**
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
</details>

## 0.2.2 (2023-01-09)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 9 calendar days.
 - 9 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-ref v0.23.0, gix-config v0.15.0, gix-command v0.2.2, gix-diff v0.26.0, gix-discover v0.12.0, gix-mailmap v0.9.0, gix-pack v0.30.0, gix-odb v0.40.0, gix-transport v0.25.2, gix-protocol v0.26.1, gix-revision v0.10.0, gix-refspec v0.7.0, gix-worktree v0.12.0, git-repository v0.32.0 ([`ffb5b6a`](https://github.com/Byron/gitoxide/commit/ffb5b6a21cb415315db6fd5294940c7c6deb4538))
    - prepare changelogs prior to release ([`4381a03`](https://github.com/Byron/gitoxide/commit/4381a03a34c305f31713cce234c2afbf8ac60f01))
    - fix warning on windows ([`3569b9f`](https://github.com/Byron/gitoxide/commit/3569b9f4412c98b21c1f7045cc2cf8e84ccd33f0))
</details>

## 0.2.1 (2022-12-30)

### New Features

 - <csr-id-8a67c13bf0b9b8646430876768155f62e243ef52/> allow setting more information when preparing commands.
   This includes
   
   - disallowing the usage of a shell

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 38 calendar days.
 - 38 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-features v0.26.0, gix-actor v0.16.0, gix-attributes v0.8.0, gix-object v0.25.0, gix-ref v0.22.0, gix-config v0.14.0, gix-command v0.2.1, gix-url v0.13.0, gix-credentials v0.9.0, gix-diff v0.25.0, gix-discover v0.11.0, gix-traverse v0.21.0, gix-index v0.11.0, gix-mailmap v0.8.0, gix-pack v0.29.0, gix-odb v0.39.0, gix-transport v0.25.0, gix-protocol v0.26.0, gix-revision v0.9.0, gix-refspec v0.6.0, gix-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - prepare changelogs prior to release ([`30d8ca1`](https://github.com/Byron/gitoxide/commit/30d8ca19284049dcfbb0de2698cafae1d1a16b0c))
    - make fmt ([`511ed00`](https://github.com/Byron/gitoxide/commit/511ed0000397a5b268530c8f5362e7d25b7c1594))
    - Merge branch 'adjustments-for-cargo' ([`f8c562a`](https://github.com/Byron/gitoxide/commit/f8c562a559e6dc3377583cc7200585dad7c3d481))
    - allow setting more information when preparing commands. ([`8a67c13`](https://github.com/Byron/gitoxide/commit/8a67c13bf0b9b8646430876768155f62e243ef52))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
</details>

## 0.2.0 (2022-11-21)

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `gix-features` and `gix-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 62 calendar days.
 - 62 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gix-hash v0.10.0, gix-features v0.24.0, gix-date v0.3.0, gix-actor v0.14.0, gix-glob v0.5.0, gix-path v0.6.0, gix-quote v0.4.0, gix-attributes v0.6.0, gix-config-value v0.9.0, gix-tempfile v3.0.0, gix-lock v3.0.0, gix-validate v0.7.0, gix-object v0.23.0, gix-ref v0.20.0, gix-sec v0.5.0, gix-config v0.12.0, gix-command v0.2.0, gix-prompt v0.2.0, gix-url v0.11.0, gix-credentials v0.7.0, gix-diff v0.23.0, gix-discover v0.9.0, gix-bitmap v0.2.0, gix-traverse v0.19.0, gix-index v0.9.0, gix-mailmap v0.6.0, gix-chunk v0.4.0, gix-pack v0.27.0, gix-odb v0.37.0, gix-packetline v0.14.0, gix-transport v0.23.0, gix-protocol v0.24.0, gix-revision v0.7.0, gix-refspec v0.4.0, gix-worktree v0.9.0, git-repository v0.29.0, gix-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.1.0 (2022-09-20)

The first usable release.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 26 calendar days.
 - 26 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Allow programs to communicate errors by default ([`5a2168e`](https://github.com/Byron/gitoxide/commit/5a2168e62f664d463fc8849efecccf7e90b382cd))
    - fix docs ([`f86364c`](https://github.com/Byron/gitoxide/commit/f86364c4e2d9efd04027978679232946494a4734))
    - fix CI ([`6565b97`](https://github.com/Byron/gitoxide/commit/6565b97d7d293ae881590960bf3e29f46fdb2cd1))
    - remove `allow prompt` builder method as typical prompt implementations don't need it ([`0236d75`](https://github.com/Byron/gitoxide/commit/0236d753805003d5a09505fab7da0b5b47392c45))
    - A builder method to allow prompts specifically ([`3be1fc7`](https://github.com/Byron/gitoxide/commit/3be1fc7d97f87893cecbe5d880576ab690bb205f))
    - Only actually use the shell if it appears to be required ([`830ee07`](https://github.com/Byron/gitoxide/commit/830ee07d943725e55a40a546b3a1b7ecefb75c4b))
    - support for multiple arguments with shell-script support ([`d8e8b54`](https://github.com/Byron/gitoxide/commit/d8e8b541bd776a267aca6dbfb8e7e793e264885b))
    - Squelch errors by default ([`1cb2e96`](https://github.com/Byron/gitoxide/commit/1cb2e967416b0fa5c6d32a0ad0b015b41f81e92c))
    - Add a way to transform a `Prepare` into a `Command` for even more flexibility ([`eeedd2c`](https://github.com/Byron/gitoxide/commit/eeedd2cab3c201109aa5bd986eb38c1f31d5fd20))
    - set version to 0.1 to avoid surprises like happened with `gix-date` ([`1322f72`](https://github.com/Byron/gitoxide/commit/1322f72fd2bd310c1c3c859ee4b49f47cdfaf100))
    - add remaining docs ([`6a39e62`](https://github.com/Byron/gitoxide/commit/6a39e62bb4aebf9c48daddf007c95b2117b4454d))
    - basic support for 'sh' based execution ([`8c61b0b`](https://github.com/Byron/gitoxide/commit/8c61b0bded71dff223e24ae68f8cf7fc50195ce9))
    - First sketch of gix-command API ([`cd4a608`](https://github.com/Byron/gitoxide/commit/cd4a608f0b8ef3adeb7a7f1979f653b63e77ad4d))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - fix dependency declaration ([`9a8369d`](https://github.com/Byron/gitoxide/commit/9a8369db9cd91a4f7447dc2d363bfb1972e5e9b1))
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **Uncategorized**
    - Release gix-command v0.1.0, gix-prompt v0.1.0, gix-url v0.9.0, gix-credentials v0.5.0, gix-diff v0.19.0, gix-mailmap v0.4.0, gix-chunk v0.3.2, gix-pack v0.23.0, gix-odb v0.33.0, gix-packetline v0.13.0, gix-transport v0.20.0, gix-protocol v0.20.0, gix-revision v0.5.0, gix-refspec v0.2.0, git-repository v0.24.0, gix-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0 ([`f5c36d8`](https://github.com/Byron/gitoxide/commit/f5c36d85755d1f0f503b77d9a565fad6aecf6728))
    - Release gix-hash v0.9.10, gix-features v0.22.5, gix-date v0.2.0, gix-actor v0.12.0, gix-glob v0.4.0, gix-path v0.5.0, gix-quote v0.3.0, gix-attributes v0.4.0, gix-config-value v0.8.0, gix-tempfile v2.0.5, gix-validate v0.6.0, gix-object v0.21.0, gix-ref v0.16.0, gix-sec v0.4.0, gix-config v0.8.0, gix-discover v0.5.0, gix-traverse v0.17.0, gix-index v0.5.0, gix-worktree v0.5.0, gix-testtools v0.9.0, gix-command v0.1.0, gix-prompt v0.1.0, gix-url v0.9.0, gix-credentials v0.5.0, gix-diff v0.19.0, gix-mailmap v0.4.0, gix-chunk v0.3.2, gix-pack v0.23.0, gix-odb v0.33.0, gix-packetline v0.13.0, gix-transport v0.20.0, gix-protocol v0.20.0, gix-revision v0.5.0, gix-refspec v0.2.0, git-repository v0.24.0, gix-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - thanks clippy ([`0dc1da5`](https://github.com/Byron/gitoxide/commit/0dc1da5e636b2eecc26fcfa0ecd814af3b78ed29))
</details>

## 0.0.0 (2022-08-25)

Initial release to reserve the name.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - prepare changelog prior to release ([`579e8f1`](https://github.com/Byron/gitoxide/commit/579e8f138963a057d87837301b097fd804424447))
    - first frame of `gix-command` crate ([`436632a`](https://github.com/Byron/gitoxide/commit/436632a3822d3671c073cdbbbaf8e569de62bb09))
 * **Uncategorized**
    - Release gix-command v0.0.0 ([`6c27e94`](https://github.com/Byron/gitoxide/commit/6c27e94c8ed6fb6155704a04d876ab6129b3b413))
</details>
