# Purpose
This is a Rust starter repository that has been set up to be a quick start for making a new Rust project using vscode as your editor.

Setting this up should give you a Rust project that has the following features for vscode
 * error highlighting
 * breakpoint debugging
 * compilation and running in vscode debug console via vscode launch options & `F5`

This has only been tested on OSX, any Linux or Windows specifics might be different.

## Setup Steps
1. install rust `https://www.rust-lang.org/tools/install`
  * Make sure to add `source $HOME/.cargo/env` to your `.zshrc` if you use `zsh` or else the build task won't find `cargo`
2. install rust language server (RLS) see most recent instructions [here](https://github.com/rust-lang/rls)
  * `rustup update`
  * `rustup component add rls rust-analysis rust-src rustfmt`
    * `rustfmt` was added extra from `rust-lang/rls` requirements
3. install the rust vscode extension [rls-vscode](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
4. For OSX/Linux install [CodeLLB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) extension (used for debugging rust code inside vscode)
  * I believe Windows will need [C/C++](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools) instead
5. Open vscode and select `File -> Open Workspace` then navigate into `{location of this repo}/.vscode/` and select `workspace.code-workspace`
  * If you are on mac and cannot see dot files press `shift+cmd+.` and they should show up
  * If VSCode doesn't suggest plugins automatically check out the recommended extensions in `worspace.code-workspace`

At this point you are all set to be able to debug! Either nvaigate to the debug panel in vscode and press the play button at the top. Or press F5 to compile and run.
Try setting a breakpoint just inside the main function to ensure that you have

## Renaming the project
After you have everything set up you might want to rename everything from `rust_starter` to your project name.
Easiest way is to do a global search for rust_starter but mainly look in the following
 * Cargo.lock
 * Cargo.toml
 * launch.json

# TODO
 * Make a debug docker container version of this so people don't have to install rust on their machine if they don't want
