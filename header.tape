Output header.gif

Set FontSize 30
Set Width 1200
Set Height 600

# Type a command in the terminal.
Type "cargo install --git https://github.com/dadav/xray2junit"
Sleep 500ms
Enter
Sleep 1s
Type "export PATH=$PATH:~/.cargo/bin/"
Sleep 500ms
Enter
Sleep 1s
Type "cat test.json"
Sleep 500ms
Enter
Sleep 1s
Type "xray2junit -i test.json | xmllint --pretty 1 - | bat"
Sleep 2s
Enter

Sleep 5s
