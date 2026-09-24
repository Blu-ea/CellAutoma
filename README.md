# CellAutoma

## A Cellular Automaton in Rust

This project is a Cellular automaton game, that can simulate multiple rules set (such as Conway game of life)

## Config File

The GameRules are define in a toml file.

- `[engine]` define the global rule for the simulation
- `[cells]` contain a map, using the cell name as key, and cell object as value
- `[[cells.<name>.rules]]` is a list of all the rule that apply to this cell type

### - Engine Object -

|          Key          |  Type  | Default | Description                                                                   |
| :-------------------: | :----: | :-----: | :---------------------------------------------------------------------------- |
|    `default_cell`     |  `u8`  |   `0`   | Define wich cell type are the dead one                                        |
|      `max_width`      | `u32`  |   `0`   | **WIP/Unused**: The max size of the grid space                                |
|     `max_height`      | `u32`  |   `0`   | **WIP/Unused**: The max size of the grid space                                |
|     `wrap_around`     | `bool` | `false` | **WIP/Unused**: If set to `true`, the border wrap arround                     |
| `simulation_distance` |  `u8`  |   `1`   | **WIP/Unused**: How far a cell look around to see cell's that are around them |

<details>
  <summary> <i>Exemple:</i> </summary>

```toml
[engine]
max_width = 1000
max_height = 1000
wrap_around = false
default_cell = 0
simulation_dist = 1
```

</details>

### - Cell Object -

|   Key   |   Type   | Description                                                                                         |
| :-----: | :------: | :-------------------------------------------------------------------------------------------------- |
|  `id`   |   `u8`   | Is used to represent this type, **must** be unique                                                  |
| `color` | `string` | Is the color of the cell in the vulkan render *(Terminal preview: WIP)*, must be write as `#RRGGBB` |

<details>
  <summary> <i>Exemple:</i> </summary>

```toml
[Cell.<name>]
id = 0
color = "#ffffff"
```

</details>

### - Rule Object -

|     Key      |  Type  | Description                                                |
| :----------: | :----: | :--------------------------------------------------------- |
|  `look_for`  | `[u8]` | A list of cell that the rule will look at to count         |
|    `min`     |  `u8`  | The minimum of cell needed to activate the rule            |
|    `max`     |  `u8`  | The maximum of cell needed to activate the rule            |
| `next_state` |  `u8`  | The next ID of the cell after the rule have been activated |

<details>
  <summary> <i>Exemple:</i> </summary>

```toml
[[cells.<name>.rules]]
look_for = [1, 2]   # will only count cells of ID 1 and 2
min = 3             # If the counter is at least above 3
max = 3             # and the counter is at most under 3
next_state = 1      # the cell will be of this type on the next iteration
```

</details>

----

#### Exemple - [Conway game of life](gameConfigFile/conway_rules.toml)

```toml
[engine]
default_cell = 0
simulation_dist = 1

[cells.Dead]
id = 0
color = "#454545"

    [[cells.Dead.rules]]
    look_for = [1]
    min = 3
    max = 3
    next_state = 1

[cells.Alive]
id = 1
color = "#FFFFFF"

    [[cells.Alive.rules]]
    look_for = [1]
    min = 0
    max = 1
    next_state = 0

    [[cells.Alive.rules]]
    look_for = [1]
    min = 4
    max = 8
    next_state = 0
```

## More information

- Only one Rule ca be aplied by cell during the simulation.
- If multiple rule can be applied to a cell, the first one defined will be used.
- The whole board contain by default only dead cells.
- Dead cells will not be simulated on they own, they must be arround a live cell to be simulated, so rules for dead cells might be applied appropriatly.
