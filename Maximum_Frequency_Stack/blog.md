Design a stack-like data structure to push elements to the stack and pop the most frequent element from the stack.

Implement the FreqStack class:

FreqStack() constructs an empty frequency stack.
void push(int val) pushes an integer val onto the top of the stack.
int pop() removes and returns the most frequent element in the stack.
If there is a tie for the most frequent element, the element closest to the stack's top is removed and returned.

Example 1:

> Input  
> ["FreqStack", "push", "push", "push", "push", "push", "push", "pop", "pop", "pop", "pop"]  
> [[], [5], [7], [5], [7], [4], [5], [], [], [], []]  
> Output  
> [null, null, null, null, null, null, null, 5, 7, 5, 4]

Explanation
FreqStack freqStack = new FreqStack();
freqStack.push(5); // The stack is [5]
freqStack.push(7); // The stack is [5,7]
freqStack.push(5); // The stack is [5,7,5]
freqStack.push(7); // The stack is [5,7,5,7]
freqStack.push(4); // The stack is [5,7,5,7,4]
freqStack.push(5); // The stack is [5,7,5,7,4,5]
freqStack.pop(); // return 5, as 5 is the most frequent. The stack becomes [5,7,5,7,4].
freqStack.pop(); // return 7, as 5 and 7 is the most frequent, but 7 is closest to the top. The stack becomes [5,7,5,4].
freqStack.pop(); // return 5, as 5 is the most frequent. The stack becomes [5,7,4].
freqStack.pop(); // return 4, as 4, 5 and 7 is the most frequent, but 4 is closest to the top. The stack becomes [5,7].

Constraints:

- 0 <= val <= 109
- At most 2 \* 104 calls will be made to push and pop.
- It is guaranteed that there will be at least one element in the stack before calling pop.

---

1. 既然是每次 pop 出出现频率最高的元素， 首先想到的自然是 BinaryHeap
2. 我们不能寄希望于去修改已经压入 BinaryHeap 的元素， 因为这个操作的时间复杂度是我们不能接受的， 但是我们可以每次压入和弹出新的元素， 每个元素描述的是某个数字的当前计数， 这样当元素被弹出时相当于将该数字恢复到上一个状态

---

```rust
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Pair(i32, i32, i32);

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.1 == other.1 {
            return self.2.partial_cmp(&other.2);
        }
        return self.1.partial_cmp(&other.1);
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 == other.1 {
            return self.2.cmp(&other.2);
        }
        return self.1.cmp(&other.1);
    }
}

struct FreqStack {
    index: i32,
    heap: BinaryHeap<Pair>,
    counts: HashMap<i32, i32>,
}

impl FreqStack {
    fn new() -> Self {
        Self {
            index: 0,
            heap: BinaryHeap::new(),
            counts: HashMap::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let entry = self.counts.entry(val).or_insert(0);
        *entry += 1;
        self.heap.push(Pair(val, *entry, self.index));
        self.index += 1;
    }

    fn pop(&mut self) -> i32 {
        let Pair(val, _, _) = self.heap.pop().unwrap();
        *self.counts.get_mut(&val).unwrap() -= 1;
        return val;
    }
}
```
