# db-meter-plugin

A plugin that measures the loudness of the signal and exposes it as midi messages

The loudness is measured according to the [EBU R128 loudness standard](https://tech.ebu.ch/docs/tech/tech3341.pdf)

To avoid MIDI compatibility issues the values are exposed as Note OFF messages: 

|            | type     | Channel | Note | Velocity                               |
|------------|----------|---------|------|----------------------------------------|
| integrated | Note OFF | 4       | C1   | 0-127 (rounded negated value in LUFS)  |
| short_term | Note Off | 4       | D1   | 0-127 (rounded negated value in LUFS)  |

