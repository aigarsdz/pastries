<div align="center"">
  <img src="pastries.webp" alt="Two cupcakes" width="250">
</div>

## Pastries

Have you ever seen a package or library in which all functionality is contained
in a single file? There are heaps of them. Additionally, there are very useful
code examples in GitHub gists, Pastebin, and other places. A lot of reusable
code that can be pasted directly into your project, but doing that manually can
be a hurdle. **Pastries** command-line tool combines the convenience of package
managers with the simplicity of copy-pasting code:

- Automatic installation and updating
- No central repository
- No accounts
- Full control of the code
- Utilization of your project's version control system

### Installation

Download the appropriate executable file from
[releases](https://github.com/aigarsdz/pastries/releases), rename it to `pastries`
and put it in a directory that is in your PATH variable.

### Usage

Adding a dependency.

```bash
pastries add --name package_name --uri 'https://example.com/file.js' --path './src/path/to/local_file.js'

pastries add --name package_name --uri 'https://example.com/file.js' --path './src/path/to/local_file.js' --update never

pastries add --name package_name --uri './shared/local/file.js' --path './src/path/to/local_file.js' --update always --local
```

#### Options

- **--name** - the package name. It is used for update and remove commands.
- **--uri** - the source URL or file path.
- **--path** - the file path where to put the dependency.
- **--local** - indicates that the source is a local file.
- **--update** - specifies when to update the dependency. Possible options are
  `always`, `never` and `on_change`. The default is `on_change` which means the
  dependency will be updated if the source and the target files are different.

Updating a dependency.

```bash
pastries update

pastries update all

pastries update package_name
```

Removing a dependency.

```bash
pastries remove package_name
```
