The goal of this project is to have a way to intuitively explore a system of equations for selecting discressionary values.


Some examples of this are with purchasing a house. The down payment impacts the monthly mortgage. The cost of the house impacts how much down you can afford and the mortgage. The expected rents also impact what you can do. The idea is to play with some knobs and values and arrive at a value that _feels_ good. Then, to use this value in your search.


Another example is with trading rules and how they impact position sizing, exit_trigger price, enter_trigger price, size, and risk/reward. Risk capital per trade and other values impact all of these numbers. It'd be nice to have some dials to twist, given risk limits and entering other values to get a _feel_ for the trade your making and to help gain intuitive understanding of there relationships.


Another example is with heat production and ventilation requirements. How much ventilation do we need to keep output air X degrees above intake air given a heating element. Again, the goal is intuative exploration of the relationships so the user can make a decision.


This is not meant to be hard-coded for any one of the above examples. The end goal is to have some way of specifying a set of equations.
Example:
```
a = b + c
d = a ^ 2 + 3 + c
b = a + 2
```

There are nodes and update paths etc.

Anyways. Hopefully that explains the idea and we can feel this out and what we'd like for eq-explorer.