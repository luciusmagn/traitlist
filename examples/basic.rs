use std::fmt::Display;

use traitlist::{tlist, LinkedList, TList, ToIndexable};

fn takes_two(list: TList![i32, i32]) {
    drop(list)
}

fn takes_anything<T, L>(list: L)
where
    T: Display,
    L: LinkedList<Item = T>,
{
    for item in list.iter() {
        println!("{}", item);
    }
}

fn main() {
    let mut my_list = tlist![1, 2, 3];

    for num in my_list.iter() {
        println!("{}", num);
    }

    for num in my_list.iter_mut() {
        *num *= 2;
    }

    for num in my_list.into_iter() {
        println!("{}", num);
    }

    let mut my_other_list = tlist![1, 1, 2, 3, 4];

    println!("[1] {}", my_other_list.indexable()[1]);

    (&mut my_other_list).indexable()[0] = 1;

    for num in my_other_list.iter() {
        println!("{}", num);
    }

    let (head, tail) = my_other_list.remove(2);

    println!("head: {}, tail: {:?}", head, tail);

    let insert_list = tlist![1, 1, 1, 1];
    let insert_list = insert_list.insert(2, 2);
    println!("{:?}", insert_list);

    takes_two(tlist![1, 2]);
    takes_anything(insert_list);
}
