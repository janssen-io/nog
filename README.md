# Nog

![preview](https://user-images.githubusercontent.com/32014449/107612664-0490ac00-6c47-11eb-9620-e754aa38b5b0.png)

## Documentation

https://timuntersberger.github.io/nog

## Syntax highlighting

Since nog uses a custom language we will provide official plugins for syntax highlighting.

* [vim](https://github.com/TimUntersberger/nog.vim)
* [vscode](https://marketplace.visualstudio.com/items?itemName=TimUntersberger.nogscript-language-support)

## Download

In almost all cases the [development](https://github.com/TimUntersberger/nog/releases/tag/development-release) release is the way to go.

## Known Problems

### Window gets managed on wrong monitor

If you are using something like PowerLauncher for launching applications you might encounter this problem with `mutli_monitor` enabled.

The problem is that the focus returns to the previous window after PowerLauncher closes, before spawning the new window.

1. PowerLauncher opens
2. You tell it to launch notepad for example
3. PowerLauncher closes -> focus returns to previous application
4. notepad launches

If the previous application mentioned in step 3 is managed by nog, the workspace will change to its grid. The only way to fix this (at least that I know of) is if we implement our own application launcher that is connected with nog. 

## Contributions

* Thank you [@ramirezmike](https://github.com/ramirezmike) for designing and implementing the graph based tile organizer

## Development

### Create Executable

```
$env:NOG_VERSION="<version>"
cargo build --release
./rcedit ./target/release/nog.exe --set-icon ./assets/logo.ico
```
