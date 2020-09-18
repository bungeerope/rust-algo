// 按照约瑟夫环数学公式进行倒推解题

pub fn last_remaining(n: i32, m: i32) -> i32 {
    let mut i = 2;
    let mut rst = 0;
    loop {
        if i > n {
            break;
        }
        rst = (rst + m) % i;
        i = i + 1;
    }
    return rst;
}
