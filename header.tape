Output header.gif

Set FontSize 24
Set Width 1200
Set Height 800

Type "cat test.json"
Sleep 500ms
Enter
Sleep 1s
Type "xray2junit -i test.json | xmllint --pretty 1 - | bat"
Sleep 2s
Enter
Sleep 5s
