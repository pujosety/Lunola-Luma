# Contributor and model metadata

The generated contributor and model metadata is retained for project attribution, but the Lunola Luma
About dialog no longer displays Contributors or Models tabs; it shows the product description,
**By Pujo Setyo** byline, version, and engine attribution. This follows the shared craftrules standard
[`standards/contributors.md`](https://github.com/storytold/craftrules/blob/main/standards/contributors.md)
for collecting and verifying self-chosen names.

## Decision

- Two files, both **compiled into the binary** by the UI crate's `build.rs` (nothing is read from
  disk or the network at run time; the web build has them too):
  - [`contributors/contributors.json`](../contributors/contributors.json): commit stats per GitHub
    username. **Generated** by `craftrules/scripts/contributors.py`; never hand-edit.
  - [`contributors/people.toml`](../contributors/people.toml): the names people chose for
    themselves. **Each contributor adds only their own entry**, in their own PR.
- Each generated contributor record contains the GitHub username, consented display/real names, PR
  and commit counts, line/binary change counts, and first/last commit dates.

## Application UI

- File ▸ About Lunola Luma displays the product identity and **By Pujo Setyo** only. Upstream
  contributor attribution remains in the repository history, `contributors/` data, and legal notices.

## Credit yourself

Add your entry to `contributors/people.toml` in a PR you commit yourself (every field optional):

```toml
[people.your-github-username]
real_name = "Your Name"
display_name = "Nickname"
sync_github_name = true   # use the name on your public GitHub profile as display name
```

or, with craftrules checked out next to `craft-apps/`:

```sh
python3 ../../craftrules/scripts/contributors.py --add-me . --real-name "Your Name" --sync-github-name
```

Names are included in the generated metadata after refresh: the script checks that each entry was
committed by its own user (or by a maintainer with a `consent_source`). Delete your entry to remove
your names.

## Refreshing (maintainers)

```sh
python3 ../../craftrules/scripts/contributors.py .           # regenerate + verify; needs git, Python 3.11+, gh
python3 ../../craftrules/scripts/contributors.py --check .   # verify people.toml only
```

Commit the updated `contributors/contributors.json`. Git author names and emails are never recorded.
