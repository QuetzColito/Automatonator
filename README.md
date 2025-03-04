# Automatonator - parse draw.io Automatons

## Usage

Download the provided binary and run (dont forget to make it executable):

```
automatonator -h
```

If there isnt a binary provided for your System refer to [Building](#building)

## Automaton Format

Accepted Formats are .xml and .drawio, everything else will be interpreted as text.
The automaton type can be specified with a flag (see --help). If it isn't, it has to be part of the filename.

#### Currently supported Types (Case-insensitive)

|     |                                   |
| --- | --------------------------------- |
| dfa | Deterministic Finite Automaton    |
| nfa | Nondeterministic Finite Automaton |
| pda | Push-down Automaton               |


### XML or Drawio

Vertices with the style `shape=doubleEllipse` (like the one provided in the Scratchpad)
will be interpreted as final states, every other vertex will be a normal state.
To mark a state as a start state, have an edge connected to it that is not connected to anything else.
You can add labels to your vertices if you want, but these will be completely ignored.
The program generates its own state identifiers (numbers starting at 1) based on the xml-ids of the vertices.

Edges connecting two vertices will be interpreted as state transitions.
Their labels will be read to get information for the transition,
the format for this varies based on [automaton type](#automaton-types).
**However** multiline labels will always be interpreted as multiple state transitions with the same source and target.

Edges connecting to one vertex will mark that state as a start state.

Edges connecting to nothing and everything else will be ignored.

### Text-based Format

`<name>` can be any text without whitespace to identify a state, but it will be replaced with generated ids.

`<label>` has to be the in the state transition format of the [automaton type](#automaton-types).

lines will be interpreted based on the first character/word (all chars up to the first whitespace).

| char       | Pattern                 | Interpretation                              |
| ---------- | ----------------------- | ------------------------------------------- |
| `c` or `t` | N/A                     | Ignored                                     |
| `s`        | `s <name>`              | vertex marked as a start state              |
| `f`        | `f <name>`              | vertex marked as final state                |
| `<name>`   | `<name> <name> <label>` | transition from first state to second state |

## Automaton Types

### DFA

**Label Format**: single char, the character that was read by the automaton.

From each state, there is only one transition for each character.

Has exactly one start state (if multiple are given, the last that was read will be used)

Can have any amount of final states.

### NFA

**Label Format**: single char, the character that was read by the automaton.

Has at least one start state.

Can have any amount of final states.

### PDA

**Label Format**: `<char>,<StackChar>,<StackChars>`

- `<char>` is the read character
- `<StackChar>` is the character currently on top of the stack
- `<StackChars>` are the characters the character on top of the stack will be replaced with

Has at least one start state.

Can have any amount of final states. However if it has no final states it can accept when the stack is emptied

------------

## Building

clone this repo and run `nix develop` in it to enter a shell the cargo version this was developed with (not tested on macOs).

If you do not have nix, having `cargo 1.85.0` or later will probably work as well.

you can now run

```
cargo run -- -h
```

or whatever other cargo stuff you wanna do.

The statically linked Linux Release Binary is built using

```
cargo build --target=x86_64-unknown-linux-musl
```

do this either inside the development-shell or make sure to add the target yourself.
