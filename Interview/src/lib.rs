use std::fmt::{Error};

use rand::{self, Rng};

//完成时间15~20分钟
//时间复杂度O(n)
/*
    先统计所有用户的总积分，后生产0..sum的随机数r表示中将用户坐标所在区间中的一个数，
    如果用户的积分区间包含了r，返回用户id
*/
pub fn Lottery(arr:&Vec<usize>)->Result<usize,&'static str>{
    
    let sum:usize = arr.iter().sum();
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..sum);
    let mut s = 0;
    for (idx,v) in arr.iter().enumerate(){
        s+=*v;
        if s > r{
            return Ok(idx);
        }
    }
    return Err("lottery err");
}

// unit test 完成时间大概30分钟
// 积分为1的某次测试方差[729.0, 169.0, 9.0, 169.0, 4.0, 784.0, 576.0, 2601.0, 49.0, 3364.0]
/*

*/
#[cfg(test)]
mod test{
    use super::Lottery;
    use rand::{self, Rng};
    #[test]
    fn test(){

        let arr:Vec<usize> = vec![1,1,1,1,1,1,1,1,1,1];
        // let arr = gen_rand_arr(50000, 1, 500000);
        let lottery_num = 10000;
        let variance = test_lottery(&arr,lottery_num);
        println!("variance:{:?}",variance);

        {//最大测试数
            let arr = gen_rand_arr(50000, 0, 50000);
            let variance = test_lottery(&arr, 1);
            // println!("variance:{:?}",variance);
        }
    }

    /// 进行lottery_num次抽奖测试，返回方差
    fn test_lottery(arr:&Vec<usize>,lottery_num:usize)->Vec<f64>{

        let sum:f64 = arr.iter().sum::<usize>() as f64;
        let mut win_count = vec![0_f64;arr.len()];
        for _ in 0..lottery_num{
            let id = Lottery(&arr).unwrap();
            win_count[id]+=1.0;
        }
        let mut expect:Vec<f64> = vec![0.0;arr.len()];
        for id in 0..arr.len(){
            expect[id] = ((arr[id] as f64) / (sum as f64)) * lottery_num as f64;
        }
        // println!("expect:{:?}",expect);
        for id in 0..arr.len(){
            let sub = win_count[id]-expect[id];
            win_count[id] = sub * sub;
        }

        win_count
    }

    /// 生成用户数为arr_len的积分列表，积分值在[low..high)之间
    fn gen_rand_arr(arr_len:usize,low:usize,high:usize)->Vec<usize>
    {
        let mut arr = vec![0;arr_len];
        let mut rng = rand::thread_rng();
        for v in arr.iter_mut(){
            *v = rng.gen_range(low..high);
        }

        arr
    }
}