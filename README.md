# Markdown Splitter

Utility tool to split a Markdown file into chunks following annotations

## Annotation syntax

In order to identify which part of the document should be splitted we can use 'export' open and closing tags

```markdown
# This is a document the is going to be splitted

<!-- export -->
This unnamed part will generate a new `export.md`
<!-- /export -->

This part will be ignored

<!-- export part1 -->
This contents will generate a new `part1.md`
<!-- /export part1 -->

<!-- export part-two.md -->
This contents will generate a new `part-two.md`
<!-- /export -->

```

# Usage

```bash
mds markdown.md
```

```bash
tree
.
├── export.md
├── markdown.md
├── part1.md
└── part-two.md
```

# Options

```bash
mds markdown.md
`export.md` created
`part1.md` created
`part-two.md` created
```

Specifying a default output file
```bash
mds markdown.md -o export.md
`export.md` created
```

# Use Case
Maintaining a diary or a blog in one file with annotations. This tool allows to split the file into multiple markdown files for each articles. Those markdown articles are now easy to publish with static site generators like Gatsby.

# Build
```bash
cargo build
```

# Test
```bash
cargo test
```

# TODO
- [ ] Support multiple export tags per file.
- [ ] Support named export tags.