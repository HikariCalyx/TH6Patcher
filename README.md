# TH6Patcher

A simple patcher for original release of *Touhou Koumakyou ~ the Embodiment of Scarlet Devil*.

## What does it do?

It would rename hardcoded Japanese filenames inside of game executable to English to allow it run in any language, without keeping the filename in gibberish.

Nothing else are modified.

## Required files, and produced result

This patch will only work on unmodified Japanese release. Do not use it on unofficial translation patch (e.g. Chinese patch).

| Required File | Patched Result   |
------------------------------------
| 東方紅魔郷.exe | th06.exe         |
| custom.exe    | custom.exe       |
| 紅魔郷ED.dat   | koumakyouED.dat |
| 紅魔郷ST.dat   | koumakyouST.dat |
| 紅魔郷CM.dat   | koumakyouCM.dat |
| 紅魔郷MD.dat   | koumakyouMD.dat |
| 紅魔郷IN.dat   | koumakyouIN.dat |
| 紅魔郷TL.dat   | koumakyouTL.dat |

Additionally, it will rename 東方紅魔郷.cfg to koumakyou.cfg if exists.

## Credit
- ZUN, or Team Shanghai Alice for creating this game.