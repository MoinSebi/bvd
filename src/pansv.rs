use gfa_reader::{NCGfa, NCPath};
use hashbrown::HashSet;
use crate::bifurcation_helper::path2combi2;


pub fn pansv_plus_index(graph: &NCGfa<()>) -> Vec<u128> {
    let mut res = vec![0; graph.nodes.len()*2 + 2];
    for (i, x) in graph.paths.iter().enumerate() {
        let index = path2combi2(&x);
        for y in index.iter() {
            let ii = y.0 as usize * 2 + y.1 as usize;
            res[ii] |= 1 << i;
        }

    }
    for x in res.iter(){
        //println!("dasd {:032b}", x);
        continue;
    }
    res
}

pub fn pansv_plus(graph: &NCGfa<()>, index: Vec<u128>){
    let mut bubbles2: HashSet<(usize, u32)> = HashSet::new();
    for (gg, x) in graph.paths.iter().enumerate(){

        let mut bubbles = Vec::new();

        let index2 = path2combi2(&x);


        let mut old: Vec<(u128, u32)> = Vec::new();
        let ii = index2[0].0 as usize * 2 + index2[0].1 as usize;

        let mut before_bits = 0;
        before_bits |= 1 << gg;
        let mut before_bubble= 0;

        for y in index2.iter() {
            //println!("\nold {:?}", old);
            let ii = y.0 as usize * 2 + y.1 as usize;
            let news = index[ii];
            let mut dd = 0;
            let mut trigger = false;
            while dd < old.len(){
                let result = news & old[dd].0;
                //println!("news {:#032b}  old {:#032b}  result {:#032b} \t ii {} old1 {}", news, old[dd].0, result, ii, old[dd].1);
                if result == old[dd].0 {
                    trigger = true;
                    bubbles.push((ii, old[dd].1));
                    old.remove(dd);
                    //println!("BUBBLE2 ");

                } else {
                    dd += 1;

                }
            }
            //println!("BEFORE {:#032b} {:#032b} {:#032b}", before_bits, news, (before_bits & news));

            // if ((before_bits & news) == news && (before_bits & news) != before_bits) || (before_bits & news) != before_bits{
            //     println!("dsajkdha");
            //     old.push((before_bits, before_bubble));
            // }

            if (before_bits & news) != before_bits{
                //println!("dasdha");
                old.push((before_bits, before_bubble));
            }

            //println!("old {:?}", old);

            if !trigger {
                if ((before_bits & news) == before_bits && ((before_bits != news) || (before_bubble == ii as u32))) || ((before_bits & news) != news && (before_bits & news) != before_bits) {
                    for x in old.iter().rev() {
                        let result = news & x.0;
                        if result == news {
                            //println!("highli {} {}", ii, x.1);
                            bubbles.push((ii, x.1));
                            break;
                        }
                    }
                }
            }
            // if (before_bits & news) != before_bits{
            //     println!("NEW2");
            //     old.push((before_bits, before_bubble));
            //
            //  }

            before_bits = news;
            before_bubble = ii as u32;

        }
        bubbles2.extend(bubbles);
        //println!("bub {:?} {}\n", bubbles, bubbles.len());
    }
    println!("{}", bubbles2.len());
}


pub fn pansv_index(graph: &NCGfa<()>) -> Vec<u32> {
    let mut res = vec![0; graph.nodes.len()*2 + 2];
    for (i, x) in graph.paths.iter().enumerate() {
        let index = path2combi2(&x);
        for y in index.iter() {
            let ii = y.0 as usize * 2 + y.1 as usize;
            res[ii] += 1
        }

    }
    for x in res.iter(){
        //println!("dasd {:032b}", x);
        continue;
    }
    res
}

pub fn pansv(graph: &NCGfa<()>, index: &Vec<u32>) -> Vec<(u32, u32)>{
    let mut res = HashSet::new();

    for x in graph.paths.iter(){
        let index2 = path2combi2(&x);
        let mut last_test = graph.paths.len() as u32;
        let mut last_bub = 0;
        let mut open: Vec<(u32, u32)> = Vec::new();
        let mut resss: Vec<(u32, u32)> = Vec::new();



        for y in index2.iter() {
            let mut trigger = false;
            let ii = y.0 as usize * 2 + y.1 as usize;
            let val = index[ii];
            let mut ii2 = 0;

            if val > last_test {
                while ii2 < open.len() {
                    if open[ii2].0 <= val {
                        trigger = true;
                        resss.push((open[ii2].1, ii as u32));
                        open.remove(ii2);
                    } else {
                        ii2 += 1;
                    }
                }
                if !trigger {
                    if open.len() > 0 {
                        resss.push((open.last().unwrap().1, ii as u32));
                    }
                }

            }



            if val < last_test{
                open.push((last_test, last_bub));
            }
            last_bub = ii as u32;
            last_test = val;


        }
        res.extend(resss);

    }
    let g = res.iter().cloned().collect::<Vec<_>>();
    g
}


