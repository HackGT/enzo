# enzo

Workspace and repo management made fun ;)

**this is experimental, not ready for use yet**

## Commands

```
enzo clone <src> <dst> [--name=<new-name>]
```
Clones the repo in `<src>` into the workspace `<dst>`. `<src>` is of the form `username/repo_name`. host = github.com; protocol = https;

```
enzo new <src> <dst> [--name=<new-name>]
```
Creates a new repo from the repo in `<src>` into the workspace `<dst>`. `<src>` is of the form `username/repo_name`. host = github.com; protocol = https;
User will be prompted to provide a name if no name is provided.

