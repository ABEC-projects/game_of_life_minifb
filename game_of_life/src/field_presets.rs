use std::usize;
const HIGHT: usize = 640;
const WIGHT: usize = 360;
pub fn full() -> Vec<bool> {
    let mut field: Vec<bool> = Vec::new();
    for _i in 0..WIGHT * HIGHT {
        field.push(true);
    }
    field
}
pub fn r_pentomino() -> Vec<bool> {
    let mut field: Vec<bool> = Vec::new();
    for _i in 0..WIGHT * HIGHT {
        field.push(false);
    }

    field[convert(50, 50)] = true;
    field[convert(51, 50)] = true;
    field[convert(50, 51)] = true;
    field[convert(49, 51)] = true;
    field[convert(50, 52)] = true;
    field
}
pub fn square() -> Vec<bool> {
    let mut field: Vec<bool> = Vec::new();
    for _i in 0..WIGHT * HIGHT {
        field.push(false);
    }

    field[convert(50, 50)] = true;
    field[convert(51, 50)] = true;
    field[convert(50, 51)] = true;
    field[convert(51, 51)] = true;
    field
}

fn convert(x: usize, y: usize) -> usize {
    x + y * 640
}
/*
pub fn heart() -> Vec<bool> {
    let mut field: Vec<bool> = vec![];
    for i in 0..100 {
        field.push(vec![]);
        for _j in 0..100 {
            field[i].push(false);
        }
    }
    field[49][50] = true;
    field[51][50] = true;
    field[49][51] = true;
    field[50][51] = true;
    field[51][51] = true;
    field[49][52] = true;
    field[50][52] = true;
    field[51][52] = true;
    field
}*/
