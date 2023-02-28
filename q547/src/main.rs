use std::{io::stdin, collections::{HashSet, VecDeque}};

pub fn find_circle_num(mut is_connected: Vec<Vec<i32>>) -> i32 {
    let len = is_connected.len();
    let mut visited = HashSet::new();
    
    let mut count = 0;
    for i in 0..len {
        if !visited.contains(&i) {
            bfs(i, &mut is_connected, &mut visited);
            count += 1;
        }
    }
    count
}

fn bfs(i: usize, is_connected: &mut Vec<Vec<i32>>, visited: &mut HashSet<usize>) {
    let mut queue = VecDeque::new();
    queue.push_back(i);
    
    while let Some(i) = queue.pop_front() {
        visited.insert(i);
        
        for j in 0..is_connected[i].len() {
            if is_connected[i][j] == 1 && !visited.contains(&j) {
                visited.insert(j);
                queue.push_back(j);   
            }
        }    
    }
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("can not read line");
    let mut v:Vec<i32> = line.trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut mat = vec![Vec::new();v.len()];
    mat[0].append(&mut v);
    for i in mat.iter_mut().skip(1){
        let mut line = String::new();
        stdin().read_line(&mut line).expect("can not read line");
        let mut v:Vec<i32> = line.trim()
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        i.append(&mut v);
    }

    find_circle_num(mat);
}
