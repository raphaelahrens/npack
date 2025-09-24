npack
====

Package manager for neovim.
It is a fork from pack

Install
-------

Currently only FreeBSD is supported

Usage
-----

All tasks should be done through `npack` command. `npack` will create a file named
*packfile* under `$VIM_CONFIG_PATH/.pack/` and all plugins are tracked in the file.
Plugin config files are stored under `$VIM_CONFIG_PATH/.pack/`. The config files
will be concatenated and stored under `$VIM_CONFIG_PATH/plugin/_pack.vim` automatically.
These files are all managed by `npack`. Never change the files manually.

By default, if `$VIM_CONFIG_PATH` is not set, `npack` will create and install all files under `~/.vim`(default vim packagepath).
If using custom location by setting `$VIM_CONFIG_PATH` variable, you need to add the following at the top of your `.vimrc`:

```
set packpath+=$VIM_CONFIG_PATH
```

#### `npack` command

```bash

# Show general usage

$ npack -h
```

#### Install plugins

```bash
$ npack help install

# install plugins


# npack install <github_user/github_repo>

$ npack install maralla/completor.vim
$ npack install maralla/completor.vim maralla/completor-neosnippet

# install all plugins

$ npack install

# install optional plugin

$ npack install altercation/vim-colors-solarized -o

# install to a specific category

$ npack install pangloss/vim-javascript -c lang

# install a plugin for types

$ npack install maralla/rope.vim --for python
$ npack install mattn/emmet-vim --for html,jinja,xml

# install a plugin loaded for a command

$ npack install gregsexton/gitv --on Gitv

# install a plugin and build after installed

$ npack install Shougo/vimproc.vim --build 'make'
```

#### Config a plugin

```bash
$ npack config maralla/completor.vim

# This command will open an editor, enter vim scripts as the config for the plugin


# For example:


#


#   let g:completor_css_omni_trigger = '([\w-]+|@[\w-]*|[\w-]+:\s*[\w-]*)$'

```

#### List installed plugins

```bash
$ npack list
```

#### Uninstall plugins

Simple uninstall a plugin will not remove plugin config file. To remove a plugin
config file use `npack uninstall <plugin> -a` or `npack config <plugin> -d`.

```bash
$ npack uninstall maralla/completor.vim
$ npack uninstall maralla/completor.vim maralla/completor-neosnippet
```

#### Update plugins

```bash
$ npack update
$ npack update maralla/completor.vim
$ npack update maralla/completor.vim maralla/completor-neosnippet
```

Misc
----

#### Shell completions

For bash, move `contrib/pack.bash` to `$XDG_CONFIG_HOME/bash_completion` or `/etc/bash_completion.d/`.

For fish, move `contrib/pack.fish` to `$HOME/.config/fish/completions/`.

For zsh, move `contrib/_pack` to one of your `$fpath` directories.

License
-------

Distributed under the terms of the [MIT](LICENSE) license.
