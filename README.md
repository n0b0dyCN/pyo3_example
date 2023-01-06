step to run:

1. venv: `python -m venv .venv`
2. activate: `source .venv/bin/activate`
3. build: `maturin develop`
4. run python: `python test.py`

Output:
```
['__class__', '__delattr__', '__dir__', '__doc__', '__eq__', '__format__', '__ge__', '__getattribute__', '__gt__', '__hash__', '__init__', '__init_subclass__', '__le__', '__lt__', '__module__', '__ne__', '__new__', '__reduce__', '__reduce_ex__', '__repr__', '__setattr__', '__sizeof__', '__str__', '__subclasshook__', 'a']
```

Expect:
Function b and c should  also be treated as python methods.