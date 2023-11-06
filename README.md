# OSR Helper

- [OSR Helper](#osr-helper)
	- [TODO](#todo)
- [Dice](#dice)
	- [Basic Operators](#basic-operators)
	- [Multi Operators](#multi-operators)
	- [Dice Operators](#dice-operators)
	- [Utility Operators](#utility-operators)
- [Tables](#tables)
- [Flowers](#flowers)

The OSR Helper is a set of tools and utilities to assist in running tabletop RPGs.

## TODO

- Fix cylical tables
- Trim table names


# Dice
The dice command rolls dice based on an expression.

## Basic Operators
These do math, collecting totaled dice results for each side and performing an operation.
- Addition `+`
- Subtraction `-` 
  - *Note: also works to negate numbers*
- Multiplication `*`
- Division `/`

```
2d4 + 2d4 => {1, 2} + {2, 3} => 8
```

## Multi Operators
Like the basic operators, however the totaled sum of the right side is applied to each value of the right side.
- Addition `++`
- Subtraction `--`
- Multiplication `**`
- Division `//`

```
3d4 ++ 2 => (1d4 + 2, 1d4 + 2, 1d4 + 2) => 3, 6, 4

3d6 ++ 2d4 => (1d6 + 2d4, 1d6 + 2d4, 1d6 + 2d4) => 11, 12, 7
```

## Dice Operators
Dice operations are for rolling and manipulating dice.
- Roll `d`
  - This one is the real money maker
- Keep Highest `kh`
- Keep Lowest `kl`
- Drop Highest `dh`
- Drop Lowest `dl`

```
2d20dl1 => {8, 14} dl 1 => 14

4d6kh3 => {3, 2, 5, 3} kh 3 =>  5, 3, 3
```

---

***NOTE***

Dice operations work with negative numbers.

- `-1d4` and `1d-4` will roll a negative number while `-1d-4` will result in a positive number.
- `4d20kh-3` will roll four dice that have 20 sides and keep the lowest 3 results.

```
-1d4 => -3

1d-4 => -2

-1d-4 => 4

4d20kh-2 => {17, 10, 10, 2} kh -2 => 2, 10 
```

---

## Utility Operators
**Commas - `,`**

Commas can be used to manually separate results

```
1d4, 1d4, 1d4 => 3, 3, 2
```

**Parens - `( )`**

Parenthesis can be used for logical grouping

```
2d(4+2) => 2d6 => 11
```

**Collect - `=`**

The collect operator reduces a collection of results into their sum

```
=(1d6, 1d6, 1d6) => 16
```

**Repeat - `@`**

The repeat operator collects the expression on the left and evaluates the expression on the right that many times

```
4@2 => 2, 2, 2, 2

4@1d4 => 4, 3, 4, 1

3@2d4 => 3, 3, 1, 2, 3, 4

6@=3d6 => 5, 15, 11, 6, 13, 8
```

# Tables

TODO

# Flowers

TODO