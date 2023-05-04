# Escr

Escr stands for expression script because everything is an expression.

# Basic Usage

## Types

* numbers
* strings
* functions
* code
* structs

## Semi Colon Rules

A semi colon must be placed at the end of each line, except the last line of a block.

The last line of a block is the return value of that block,
for example:

```
if 1 then
    println(3 + 3);
    1
fi
```

The last line in the block is `1`, so it is the return value of the if block. The last line of the program is the if block, so it is the return value of the program

## Variables

`var x = 3`

## While Loops

```
while 1
```

## Creating a functions

```
var name(params) =
    code...
rav
```

## Code Type

The code type is a type that contians ast nodes, and can be run the same way a function could

```
var name = code
    3 + 3
edoc;

name()
```


## Structs

```
struct Person = 
    age,
    name
end;

var p1 = Person(10, "Brandon")
```

* Struct fields cannot be set after an instance of the struct is created.
    * Instead a copy must be made with the `set` function, example:

```
var p1 = set(p1, "age", p1.age + 1)
```
