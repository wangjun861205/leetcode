struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        let mut buy_heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut sell_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        let mut amount: i64 = 0;
        for order in orders {
            // 购买订单
            if order[2] == 0 {
                let mut buy_order = order;
                while let Some(sell_order) = sell_heap.peek().cloned() {
                    // 当存在比当前订单价格低或者等于的售卖订单的时候
                    if sell_order.0 .0 <= buy_order[0] {
                        let mut sell_order = sell_heap.pop().unwrap().0;
                        // 当购买订单的数量小于售卖订单的数量时
                        if buy_order[1] < sell_order.1 {
                            // 将售卖订的数量减去购买订单的数量并跳出循环
                            sell_order.1 -= buy_order[1];
                            amount -= buy_order[1] as i64;
                            buy_order[1] = 0;
                            sell_heap.push(Reverse(sell_order));
                            break;
                            // 当购买订单的数量等于收买订单的数量时
                        } else if buy_order[1] == sell_order.1 {
                            amount -= buy_order[1] as i64;
                            buy_order[1] = 0;
                            break;
                            // 当购买订单的数量大于收买订单的数量时
                        } else {
                            buy_order[1] -= sell_order.1;
                            amount -= sell_order.1 as i64;
                            continue;
                        }
                    } else {
                        // 如果收买订单价格大于当前订单的价格
                        break;
                    }
                }
                // 检查购买订单的数量是否还有剩余
                if buy_order[1] > 0 {
                    amount += buy_order[1] as i64;
                    buy_heap.push((buy_order[0], buy_order[1]));
                } else {
                    continue;
                }
            } else {
                // 售卖订单
                let mut sell_order = order;
                while let Some(buy_order) = buy_heap.peek().cloned() {
                    // 如果售卖订单的价格小于等于购买订单的价格
                    if sell_order[0] <= buy_order.0 {
                        let mut buy_order = buy_heap.pop().unwrap();
                        if sell_order[1] < buy_order.1 {
                            buy_order.1 -= sell_order[1];
                            amount -= sell_order[1] as i64;
                            sell_order[1] = 0;
                            buy_heap.push(buy_order);
                            break;
                        } else if sell_order[1] == buy_order.1 {
                            amount -= sell_order[1] as i64;
                            sell_order[1] = 0;
                            break;
                        } else {
                            sell_order[1] -= buy_order.1;
                            amount -= buy_order.1 as i64;
                            continue;
                        }
                    } else {
                        break;
                    }
                }
                if sell_order[1] > 0 {
                    amount += sell_order[1] as i64;
                    sell_heap.push(Reverse((sell_order[0], sell_order[1])));
                }
            }
        }
        (amount % 1000000007) as i32
    }
}
fn main() {
    println!("Hello, world!");
}
