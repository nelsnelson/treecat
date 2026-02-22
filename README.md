# treecat

Print files under one or more paths. In a Git work tree, respects .gitignore.


## Usage

Default to current directory:

```sh
treecat
```

Print files under specific paths:

```sh
treecat path1 path2
```


## Install

```sh
make
sudo make install
```

## Behavior

* If no paths are provided, defaults to `.` (the current working directory).
* In a Git work tree, files are collected from the repo root and Git ignore rules are honored (.gitignore, exclude, and global ignores).
* Any `.git` directory is pruned from traversal (its contents are never visited), even if a broader path like `.` is given.
* Absolute paths outside the repo are skipped with a warning on stderr.
* Files named `.DS_Store` and AppleDouble files starting with `._` are skipped.
* For files that look like text, prints the file contents (streamed), and adds a trailing newline if the file does not end with one.
* Otherwise, prints a best-effort MIME type (fallback `application/octet-stream`) and the file size in bytes.
* Uses Rayon for parallel processing; output is not interleaved between files, but overall file order is best-effort (depends on traversal and which worker finishes first). 
