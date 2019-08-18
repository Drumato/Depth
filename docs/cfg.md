# term

```
T -> num
T -> ( E )
```

# expr

priority -> **`E1` > `E2` > `E3`**

```
E1 -> T
E2 -> E1 * T
E2 -> E1 / T
E3 -> E2 + T
E3 -> E2 - T
```
