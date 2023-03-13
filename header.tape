Output header.gif

Set PlaybackSpeed 0.75
Set TypingSpeed 75ms
Set FontSize 24
Set Width 1200
Set Height 800

Type "cat test.json"

Sleep 2s

Enter

Sleep 1s

Type "xray2junit -i test.json | xmllint --pretty 2 - | bat"

Sleep 2s

Enter

Sleep 5s

Ctrl+d

Sleep 2s

Ctrl+d

Sleep 2s

Ctrl+d

Sleep 2s

Ctrl+d

Sleep 15s
