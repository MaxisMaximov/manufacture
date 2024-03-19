# Code Codex

## A standardized guideline for names in Manufacture

- [Variables](#variables)
  - [Input](#input)
  - [Output](#output)
  - [Inside function](#inside-function)
  - [Important variables](#important-variables)
  - [Common names](#common-names)
- [Functions](#functions)
  - [Public](#public)
  - [Private](#private)
  - [Groups](#groups)
- Structs
- Enums
- [Debug strings](#debug-strings)
- Misc

## Variables

### Input

Naming for any data you want the function to have

Syntax: `IN_[naMe]`

e.g.

```rust
  fn render_util_world(&mut self, IN_gameData: &mut SYS_DATA)  
```

```rust
  pub fn DATA_cacheData_ADD(&mut self, IN_dataIndex: &str, IN_data: CACHE_TYPE)
```

### Output

Naming for any data that will be returned but requires intialization first, such as Lists or Vectors

Syntax: `OUT_[naMe]`

e.g.

```rust
  let mut OUT_genTiles: Vec<(usize, usize)> = Vec::new();  
  ...  
  return OUT_genTiles;
```

### Inside function

Naming for variables inside the function to not confuse them with global variables

Syntax: `W_[naMe]`

e.g.

```rust
  let mut w_iterCoords: system::coords = IN_pos;
```

```rust
  let w_cellCoords = IN_pos[0] + IN_pos[1] * system::SYS_REND_BUFFER_X;
```

### Important variables

Naming for system constants or other important variables

#### System

Syntax: `[CATE_GORY]_[SUB_CATEGORY]_[NA_ME]`

e.g.

```rust
pub const SYS_TICKRATE_BASE: u8 = 8;
```

#### Important

Syntax: `[CATEGORY]_[naMe]`

e.g.

```rust
  let TIMER_renderTime = Instant::now()
```

### Common names

Common names for reocurring variables  
Following this is advised

Position: `IN_pos` | `w_[...]Pos`  
System data: `&mut IN_sysData`  
Indexing for Hashmaps: `IN_dataIndex`  
Timers for speed check: `TIMER_[...]Time`

## Functions

### Public

Naming for functions that will be used outside the struct/module

Syntax: `[CATEGORY]_[naMe]`

e.g.

```rust
  pub fn MAIN_renderGame(&mut self, IN_sysData: &mut DATA_master)
```

### Private

Naming for functions that will be used internally by struct/module **only**

Syntax: `[cateGory]_[subCategory]_[naMe]`  

e.g.

```rust
  fn render_util_text(&mut self, IN_sysData: &mut DATA_master)
```

### Groups

Naming for groups of functions that do similar operations

Syntax: `[CATEgoRy]_[groUp]_[TYPE]`

e.g.

```rust
  pub fn DATA_cacheData_GET(&self, IN_dataIndex: &str)
```

## Structs

TODO

## Enums

TODO

## Debug Strings

Naming for strings that will be used for debug

Syntax: `.DEBUG_[cateGory]/#[CATEGORY]_[naMe]`

e.g.

```json
  ".DEBUG_sys":{
    "#SYS_dataContainerInit": "Data Container initialized",
```

## Misc

TODO
