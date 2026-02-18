fn score_char(c: char) -> f32 {
    if (c < 'a' || c > 'z') { return 0.0; }
    let c = (c as u32 - ('a' as u32)) as u8;
 if c==0 {8.12} else
 if c==1 {1.49} else
 if c==2 {2.71} else
 if c==3 {4.32} else
 if c==4 {12.0} else
 if c==5 {2.30} else
 if c==6 {2.03} else
 if c==7 {5.92} else
 if c==8 {7.31} else
 if c==9 {0.10} else
 if c==10 {0.69} else
 if c==11 {3.98} else
 if c==12 {2.61} else
 if c==13 {6.95} else
 if c==14 {7.68} else
 if c==15 {1.82} else
 if c==16 {0.11} else
 if c==17 {6.02} else
 if c==18 {6.28} else
 if c==19 {9.10} else
 if c==20 {2.88} else
 if c==21 {1.11} else
 if c==22 {2.09} else
 if c==23 {0.17} else
 if c==24 {2.11} else
 if c==25 {0.07} else {0.0}
}

pub fn score(t: &str) -> f32 {
    let t = t.to_lowercase();
    t.chars().into_iter().map(|x| score_char(x)).sum()
}
