# Code Codex

## A standardized guideline for names in Manufacture

- [Variables](#variables)
  - [Input](#input-variables)
  - [Output](#output-variables)
  - [Inside function](#variables-inside-function)
  - [Important variables](#constant-variables)
  - [Temporary values](#temporary-values)
  - [Common names](#common-variable-names)
- [Functions](#functions)
  - [Public](#public-functions)
  - [Private](#private-functions)
  - [Groups](#function-groups)
- [Structs](#structs)
  - [Naming](#struct-naming)
  - [Fields](#struct-fields)
  - [Functions](#struct-functions)
- [Enums](#enums)
- [Debug strings](#debug-strings)
- [Misc](#misc)

## Variables

### Input Variables

Naming for any data you want the function to have

Syntax: `IN_[naMe]`

e.g.

```rust
  fn render_util_world(&mut self, IN_gameData: &mut SYS_DATA)  
```

```rust
  pub fn DATA_cacheData_ADD(&mut self, IN_dataIndex: &str, IN_data: CACHE_TYPE)
```

### Output Variables

Naming for any data that will be returned but requires intialization first, such as Lists or Vectors

Syntax: `OUT_[naMe]`

e.g.

```rust
  let mut OUT_genTiles: Vec<(usize, usize)> = Vec::new();  
  ...  
  return OUT_genTiles;
```

### Variables inside function

Naming for variables inside the function to not confuse them with global variables

Syntax: `W_[naMe]`

e.g.

```rust
  let mut W_iterCoords: system::coords = IN_pos;
```

```rust
  let W_cellCoords = IN_pos[0] + IN_pos[1] * system::SYS_REND_BUFFER_X;
```

### Constant variables

Naming for system constants or other important variables

#### System Variables

Syntax: `[CATE_GORY]_[SUB_CATEGORY]_[NA_ME]`

e.g.

```rust
pub const SYS_TICKRATE_BASE: u8 = 8;
```

#### Important variables

Syntax: `[CATEGORY]_[naMe]`

e.g.

```rust
  let TIMER_renderTime = Instant::now()
```

### Temporary values

Primarily to stop compiler from whining

Syntax: `idkfa_[naMe]`

```rust
  let mut idkfa = DATA_LOCK.DATA_player.p_pos
```

### Common variable names

Common names for reocurring variables  
Following this is advised

Position: `IN_pos` | `w_[...]Pos`  
System data: `&mut IN_sysData`  
Indexing for Hashmaps: `IN_dataIndex`  
Timers for speed check: `TIMER_[...]Time`

## Functions

### Public Functions

Naming for functions that will be used outside the struct/module

Syntax: `[CATEGORY]_[naMe]`

e.g.

```rust
  pub fn MAIN_renderGame(&mut self, IN_gameData: &mut DATA_master)
```

### Private Functions

Naming for functions that will be used internally by struct/module **only**

Syntax: `[cateGory]_[subCategory]_[naMe]`  

e.g.

```rust
  fn render_util_text(&mut self, IN_sysData: &mut DATA_master)
```

### Function Groups

Naming for groups of functions that do similar operations

Syntax: `[CATEgoRy]_[groUp]_[TYPE]`

e.g.

```rust
  pub fn DATA_cacheData_GET(&self, IN_dataIndex: &str)
```

## Structs

### Struct Naming

Naming for structs

Syntax: `[TYPE]_[naMe]`

e.g.

```rust
  pub struct SYS_dataMaster
```

### Struct Fields

Naming for struct fields

Syntax: `[CATEGORY]_[naMe]`

e.g.

```rust
  pub DATA_debug: HashMap<String, DATA_debugItem>,
```

### Struct Functions

Naming for struct functions

Same rules as those in [Functions](#functions) section

Additionally, if the struct is specialized in one thing only (i.e. Renderer, Logic), it must have a `MAIN_` function to start their process

## Enums

Naming for enums

Syntax: `[TYPE]_[naMe]`

e.g.

```rust
  pub enum DATA_cacheType {
```

## Debug Strings

Naming for strings that will be used for debug

Syntax: `.DEBUG_[cateGory]/#[CATEGORY]_[naMe]`

e.g.

```json
  ".DEBUG_sys":{
    "#SYS_dataContainerInit": "Data Container initialized",
```

## Misc

Naming for miscellaneous things

### Global values lock

Syntax: `[NAME]_LOCK`

```rust
  let mut DATA_LOCK = SYS_data.lock().unwrap()
```
