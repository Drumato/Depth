# term

priority -> **`T1` > `T2`**

```
T1 -> -(T)
T2 -> num
T2 -> ( E )

```

# expr

priority -> **`E1` > `E2` > `E3` > `E4`**

```
E1 -> T
E2 -> E1 * T
E2 -> E1 / T
E2 -> E1 % T
E3 -> E2 + T
E3 -> E2 - T
E4 -> E3 >> T
E4 -> E3 << T
```
