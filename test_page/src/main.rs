//let start = std::cmp::max(0, staking_count.saturating_sub(1).saturating_sub(offset));
//let end = std::cmp::max(0, start.saturating_sub(limit.saturating_sub(1));
//(start..end).map(|i| read_pubkey(&self.storage, &stakingkey_keyspace, &format!("{}", i))?).collect()

fn main() {
    let offset:u64=1;
    let limit:u64= 100;
    let count:u64=5;
    let start=count.saturating_sub(offset);
    let end=start.saturating_sub(limit);
    let mut m: Vec<u64>=(end..start).map(|i| {
        i
    }).collect();
    m.reverse();
    println!("{:?}", m);
}
