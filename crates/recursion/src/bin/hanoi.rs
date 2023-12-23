const NUM_DISKS: usize = 3;

fn main() {
    let size: usize = sorting::get_count("What size board?").unwrap_or(NUM_DISKS);
    let mut posts = vec![vec![0; size]; 3];

    // Put the disks on the first post in order, smallest first (on top).
    (0..size).for_each(|i| posts[0][i] = i + 1);

    // Initial setup.
    draw_posts(&mut posts, size);

    move_disks(&mut posts, size, 1, 2, 3);

    // Drawn in move_disk after move.
    // draw_posts(&mut posts, size);
    println!("Ok");
}

fn move_disk(posts: &mut [Vec<usize>], from_post: usize, to_post: usize) {
    let from_row: usize = posts[from_post - 1]
        .iter()
        // First position in the column that is not 0.
        .position(|i| *i != 0)
        .unwrap_or(0);

    let to_row: usize = posts[to_post - 1]
        .iter()
        // First position from the right in the column that is 0.
        .rposition(|i| *i == 0)
        .unwrap_or(0);

    (posts[from_post - 1][from_row], posts[to_post - 1][to_row]) =
        (posts[to_post - 1][to_row], posts[from_post - 1][from_row]);

    draw_posts(posts, posts[0].len());
}

fn move_disks(posts: &mut [Vec<usize>], num: usize, from: usize, to: usize, tmp_post: usize) {
    match num {
        0 => (),
        n => {
            move_disks(posts, n - 1, from, tmp_post, to);
            move_disk(posts, from, to);
            move_disks(posts, n - 1, tmp_post, to, from);
        }
    }
}

fn draw_posts(posts: &mut [Vec<usize>], num_disks: usize) {
    (0..num_disks).for_each(|row| {
        posts.iter().for_each(|post| print!("{} ", post[row]));
        println!();
    });

    println!("=====")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_disk() {
        let size = 5;
        let mut posts = vec![vec![0; size]; 3];

        // Put the disks on the first post in order, smallest first (on top).
        for i in 0..size {
            posts[0][i] = i + 1;
        }

        move_disk(&mut posts, 1, 3);
        assert_eq!(
            posts,
            vec![
                vec![0, 2, 3, 4, 5],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1]
            ]
        );
        move_disk(&mut posts, 1, 2);
        assert_eq!(
            posts,
            vec![
                vec![0, 0, 3, 4, 5],
                vec![0, 0, 0, 0, 2],
                vec![0, 0, 0, 0, 1]
            ]
        );
        move_disk(&mut posts, 3, 2);
        assert_eq!(
            posts,
            vec![
                vec![0, 0, 3, 4, 5],
                vec![0, 0, 0, 1, 2],
                vec![0, 0, 0, 0, 0]
            ]
        );
        move_disk(&mut posts, 1, 3);
        assert_eq!(
            posts,
            vec![
                vec![0, 0, 0, 4, 5],
                vec![0, 0, 0, 1, 2],
                vec![0, 0, 0, 0, 3]
            ]
        );
        move_disk(&mut posts, 2, 1);
        assert_eq!(
            posts,
            vec![
                vec![0, 0, 1, 4, 5],
                vec![0, 0, 0, 0, 2],
                vec![0, 0, 0, 0, 3]
            ]
        );
        move_disk(&mut posts, 2, 3);
        assert_eq!(
            posts,
            vec![
                vec![0, 0, 1, 4, 5],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 2, 3]
            ]
        );
        move_disk(&mut posts, 1, 3);
        assert_eq!(
            posts,
            vec![
                vec![0, 0, 0, 4, 5],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 1, 2, 3]
            ]
        );
    }
}
