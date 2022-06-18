# Minesweeper

A simple command line minesweeper game.

```text
rust-minesweeper 

USAGE:
    rust-minesweeper [OPTIONS]

OPTIONS:
    -c <COLUMNS>        Number of columns [default: 10]
    -h, --help          Print help information
    -r <ROWS>           Number of rows [default: 10]
```

Ex:
```text
$ rust-minesweeper 
board: 100, mines: 10
   1  2  3  4  5  6  7  8  9  10 
 1 .  .  .  .  .  .  .  .  .  .  
 2 .  .  .  .  .  .  .  .  .  .  
 3 .  .  .  .  .  .  .  .  .  .  
 4 .  .  .  .  .  .  .  .  .  .  
 5 .  .  .  .  .  .  .  .  .  .  
 6 .  .  .  .  .  .  .  .  .  .  
 7 .  .  .  .  .  .  .  .  .  .  
 8 .  .  .  .  .  .  .  .  .  .  
 9 .  .  .  .  .  .  .  .  .  .  
10 .  .  .  .  .  .  .  .  .  .  

(o, f, q): o 1 1
board: 100, mines: 10
   1  2  3  4  5  6  7  8  9  10 
 1    1  .  .  .  .  .  .  .  .  
 2 1  2  .  .  .  .  .  .  .  .  
 3 .  .  .  .  .  .  .  .  .  .  
 4 .  .  .  .  .  .  .  .  .  .  
 5 .  .  .  .  .  .  .  .  .  .  
 6 .  .  .  .  .  .  .  .  .  .  
 7 .  .  .  .  .  .  .  .  .  .  
 8 .  .  .  .  .  .  .  .  .  .  
 9 .  .  .  .  .  .  .  .  .  .  
10 .  .  .  .  .  .  .  .  .  .  

(o, f, q): o 10 1
board: 100, mines: 10
   1  2  3  4  5  6  7  8  9  10 
 1    1  .  .  .  .  1           
 2 1  2  .  .  .  .  1           
 3 .  .  .  .  .  .  2           
 4 .  2  1  1  .  .  1           
 5 1  1     1  2  2  1           
 6                               
 7 1  1           1  1  1        
 8 .  2  1        1  .  1        
 9 .  .  1        1  1  1        
10 .  .  1                       

(o, f, q): 
```
# Crate docs
[docs](https://darrell-roberts.github.io/rust-minesweeper/rust_minesweeper/)