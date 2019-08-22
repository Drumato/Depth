# term

priority -> **`T1` > `T2`**

```
T1 -> -T
T1 -> *T
T1 -> &T
T2 -> num
T2 -> ident
T2 -> ( E )

P -> i8
P -> i16
P -> i32
P -> i64
P -> Pointer<P>
```

# expr

priority -> **`E1` > `E2` > `E3` > `E4` > `E5` > `E6`**

```
E1 -> T
E2 -> E1 * T
E2 -> E1 / T
E2 -> E1 % T
E3 -> E2 + T
E3 -> E2 - T
E4 -> E3 >> T
E4 -> E3 << T
E5 -> E4 < T
E5 -> E4 <= T
E5 -> E4 > T
E5 -> E4 >= T
E6 -> E5 == T
E6 -> E5 != T
```

# stmt

```
S -> E
S -> return E
S -> if E S (else S)
S -> let T : P = E
S -> { S*n }
```
