# Selected Music ID:
This Pointer will give "159" or "4294967295" when current choice is not a music (national match etc.), or a music id.

| Base Address | Offset 0 | Offset 1 | Offset 2 | Offset 3 |
| - | - | - | - | - |
| "chusanApp.exe"+0185F37C | 3C | 4 | 1B8 | 794 |
| "chusanApp.exe"+0185F37C | 3C | 10 | 1B8 | 794 |

# Selected Music Difficult:

There are two Pointers, during selecting status they will be the same (0, 1, 2, 3, 4, 5, the id of difficult), but one will be 0 and another one will be 3 when enter actual playing status.

## Pointers 1 (Become 0)

There are two offsets, both of are working:

| Base Address | Offset 0 | Offset 1 | Offset 2 | Offset 3 |
| - | - | - | - | - |
| "chusanApp.exe"+0185F37C | 3C | 4 | 1B4 | 2B0 |
| "chusanApp.exe"+0185F37C | 3C | 10 | 1B4 | 2B0 |

## Pointers 2 (Become 3)

It doesn't work with pointer search, probably some dymanic pointers or current settings is not enough.

Known Address pair:
19287620 - 8E479240
192D1AE0 - 914C4C90
1931D9A8 - 91324CA0

# Status Enum

This pointer may not be the real status enum but it can be used for checking current status. When the value is 4, current status is selecting, and when the value is 6, the status is playing.

| Base Address | Offset 0 | Offset 1 | Offset 2 | Offset 3 | Offset 4 |
| - | - | - | - | - | - |
| "chusanApp.exe"+0185F37C | C8 | 28 | 544 | 0 | 74 |