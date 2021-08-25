Given the string croakOfFrogs, which represents a combination of the string "croak" from different frogs, that is, multiple frogs can croak at the same time, so multiple “croak” are mixed. Return the minimum number of different frogs to finish all the croak in the given string.

A valid "croak" means a frog is printing 5 letters ‘c’, ’r’, ’o’, ’a’, ’k’ sequentially. The frogs have to print all five letters to finish a croak. If the given string is not a combination of valid "croak" return -1.

Example 1:

> Input: croakOfFrogs = "croakcroak"  
> Output: 1

Explanation:

One frog yelling "croak" twice.

Example 2:

> Input: croakOfFrogs = "crcoakroak"  
> Output: 2

Explanation:

The minimum number of frogs is two.  
The first frog could yell "crcoakroak".  
The second frog could yell later "crcoakroak".

Example 3:

> Input: croakOfFrogs = "croakcrook"  
> Output: -1

Explanation:  
The given string is an invalid combination of "croak" from different frogs.

Example 4:

> Input: croakOfFrogs = "croakcroa"  
> Output: -1

Constraints:

- 1 <= croakOfFrogs.length <= 10^5
- All characters in the string are: 'c', 'r', 'o', 'a' or 'k'.

---

挺有意思的一道题

我们分别对"croak"中的这 5 个字母进行计数  
count_c  
count_r  
count_o  
count_a  
count_k
在我们对 croakOfFrogs 进行计数的过程中的任何一个时间点，这 5 个计数都要保证具有如下关系:  
count_c >= count_r >= count_o >= count_a >= count_k  
如果不能满足这种关系，那一定是非法的字符串
