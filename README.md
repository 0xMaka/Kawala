<!--  
  @title  : README.md 
  @author : Maka 
  @notice : follow the white rabbit go full kawala 😉🧀🃏
<3 -->

# Kawala - Just a Kwl Data Companion (High in Kawality)

   <img src="misc/assets/kawala-800.png" width=800 >
            <!--  p.s good vibes for you -->
              
---

#### What can it be used for?
Kawala should be useful to anyone interested in analysing or crafting calldata. <br>
Ranging from support at a protocol looking for a quick summary, to an operator debugging custom streams for their bot. <br>
See tests and examples for inspiration.

---

<table class="fixed-align">
  <tbody>
    <tr>
  <td>  
      
| codebase                     | description                                           |
|------------------------------|-------------------------------------------------------|
| => [lib.rs](src/lib.rs)      | Core lib, base types, Kawala specific functionality.  | 
| => [kwl32.rs](src/kwl32.rs)  | Foundational mod, handles the 32 byte word operations.| 
| => [bai.rs](src/bai.rs)      | Foundational mod, handles the hex / byte conversions. |
| => [examples](examples/)     | Examples, focused on realistic scenarios.             |
| => [tests](tests/)           | Kawala test suite.                                    |
|      <img width=135/>        |                     <img width=430/>                  |
| =>     👨‍💻 =>🐇=>🐨         |  4920646f6e2774206576656e207365652074686520636f64652e | <!-- I don't even see the code. -->
  </td>

  <td valign="top", valign="right">

|   [version history](misc/kawalas_log.md)  | 
|-------------------------------------------|
|  => [v0.1.3](misc/kawalas_log.md#v013)    |
|  => [v0.1.2](misc/kawalas_log.md#v012)    |
|  => [v0.1.1](misc/kawalas_log.md#v011)    | 
|  =>             --------                  | 
|            <img width=100/>               |

  </td>
    </tr>
  </tbody>
</table>

<table class="fixed-align">
  <tbody>
   <tr>
  <th>
    ʕ•ﻌ•ʔ
  </th>
    </tr>  
    <tr>
  <td valign="top", valign="center">
     
```rust
/* install *///------------//* import *///------------------------->
/* Run in dir:            */ use kawala::{ View, Calldata, WithSig }; 
cargo add kawala           // 
// or add to Cargo.toml:   //* init *///------------------------------------------------------->
kawala = "0.1.3"             let mut view = View::new(Calldata::from_hex("ff"), WithSig::False);
``` 
</td>
  </tr>
 
  <tr>
<td>
  <img width=800/>
</td>
  </tr>
</table>

_..416c6c204920736565206973206172726179206f66667365742c2075696e7432353620616e6420616464726573732e_ ![image](misc/assets/glider.png)
                    <!--  All I see is array offset, uint256 and address. -->
                                                                                                           <!-- say free <3 -->

