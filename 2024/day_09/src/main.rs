use itertools::Itertools;
use std::fs::read_to_string;

fn compact(mut disk: &mut [Option<usize>]) {
    while !disk.is_empty() {
        if disk.first().unwrap().is_some() {
            disk = &mut disk[1..];
            continue;
        }

        if disk.last().unwrap().is_none() {
            let len = disk.len();
            disk = &mut disk[..len - 1];
            continue;
        }

        disk[0] = disk.last_mut().unwrap().take();
    }
}

fn move_file(disk: &mut [Option<usize>], file_id: usize, gaps: &mut [(usize, usize)]) -> usize {
    let r = (0..disk.len())
        .rposition(|i| {
            disk[i] == Some(file_id)
                && i.checked_sub(1)
                    .and_then(|i| disk.get(i))
                    .copied()
                    .flatten()
                    != Some(file_id)
        })
        .unwrap();

    let file_len = disk[r..]
        .iter()
        .take_while(|x| **x == Some(file_id))
        .count();

    let gap_index = match gaps
        .iter()
        .take_while(|(i, _)| *i < r)
        .position(|&(_, len)| len >= file_len)
    {
        Some(x) => x,
        None => return r,
    };

    let l = gaps[gap_index].0;

    disk[r..r + file_len].fill(None);
    disk[l..l + file_len].fill(Some(file_id));
    gaps[gap_index].1 -= file_len;
    gaps[gap_index].0 += file_len;
    r
}

fn compress_no_fragment(mut disk: &mut [Option<usize>]) {
    let max_file_id = disk.iter().rev().copied().flatten().next().unwrap();
    let mut gaps = disk
        .iter()
        .enumerate()
        .chunk_by(|x| x.1)
        .into_iter()
        .filter(|x| x.0.is_none())
        .map(|x| x.1)
        .map(|c| c.map(|x| x.0).collect_vec())
        .map(|v| (v[0], v.len()))
        .collect_vec();

    for file_id in (0..=max_file_id).rev() {
        let r = move_file(disk, file_id, &mut gaps);
        disk = &mut disk[..r];
    }
}

fn process_input(input: &str) -> Vec<Option<usize>> {
    input
        .trim()
        .chars()
        .chunks(2)
        .into_iter()
        .enumerate()
        .map(|(i, c)| (i, c.collect_vec()))
        .map(|(i, v)| (i, (v[0], v.get(1).copied().unwrap_or('0'))))
        .map(|(i, (n, m))| (i, n.to_digit(10).unwrap(), m.to_digit(10).unwrap()))
        .flat_map(|(i, n, m)| [vec![Some(i); n as usize], vec![None; m as usize]].concat())
        .collect_vec()
}

fn checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .map(|(i, o)| i * o.unwrap_or_default())
        .sum()
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let mut disk = process_input(&input);
    let mut disk2 = disk.clone();
    compact(&mut disk);
    let output_1 = checksum(&disk);
    compress_no_fragment(&mut disk2);
    let output_2 = checksum(&disk2);
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
