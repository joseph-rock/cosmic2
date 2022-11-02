# cosmic2

A program which generates the cosmic number chain for integers 1 - 1,000,000.  


## What is a cosmic number?

A cosmic number is a number in which the english spelling has the same amount of characters as the number it represents. The only positive cosmic number is 4.

> length of "four" = 4
  
  
## What is a cosmic chain?

Any number not cosmic can be chained together, inevetabally reaching 4.

> 1 is 3 // length of "one" = 3  
> 3 is 5 // length of "three" = 5  
> 5 is 4 // length of "five" = 4  
> 4 is cosmic

Length of larger numbers with multiple words compounded are calculated without spaces, dashes, or the word "and".

> Example: 123
>
> :x: "one hundred and thirty-two" // length = 26  
> :heavy_check_mark: "onehundredthirtytwo"  // length = 19
  
  
## Converting numbers to english

Integers can be broken down into a handful of common "atomic" english words.

These are 1 - 19

> one, two, three ... eighteen, nineteen

And multiples of 10 from 20 - 90

> twenty, thirty, forty ... eighty, ninety

Numbers can also be assigned a "magnitude"

> hundred, thousand, million, billion ...

This program calculates "hundred" as the only magnitude that can only be associated with an integer from 1 - 9. All others can be associated with any integer from 1 - 999.

> Example: 1600
>
> :x: "sixteen hundred" // length = 14 *without space(s)  
> :heavy_check_mark: "one thousand six hundred"  // length = 21 *without space(s)  

We can break a number down into groups as typically represented by a comma "," in the United States. 

> Example: 12,450,052  
> Groups: [12], [450], [52]

These groups will be an integer from 0 - 999. 0 is not represented as "zero" = length 4, rather it is the absence of a group.

> Example: 1,000,001  
> Groups: [1], [0], [1]
>
> :x: "one million zero thousand one" // length = 25 *without space(s)  
> :heavy_check_mark: "one million one" // length = 13 *without space(s)

Therefore, every group can be represented by the length of a number from 1-999 followed by the optional length of the magnitude identifyer. The resulting sum of all groups is the length of the entire english representation of a number.
