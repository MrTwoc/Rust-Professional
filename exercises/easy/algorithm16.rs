/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place. 
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};

// 定义一个函数，用于将输入的矩阵顺时针旋转90度
pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    // 获取原矩阵的行数（rows）和列数（cols）
    let rows = matrix.len();
    let cols = matrix[0].len();

    // 创建目标矩阵容器，维度转换为[原列数 x 原行数]
    // 初始化全零矩阵用于存放旋转后的元素
    let mut transposed = vec![vec![0; rows]; cols];
    
    // 遍历原矩阵所有元素坐标(i,j)
    for i in 0..rows {
        for j in 0..cols {
            // 旋转映射：原坐标(i,j)的元素会移动到新矩阵的坐标(j, rows-1-i)位置
            // rows-1-i 实现行坐标的逆向排列，完成顺时针旋转的核心逻辑
            transposed[j][rows - 1 - i] = matrix[i][j];
        }
    }

    // 将计算结果通过解引用赋值回原始矩阵
    *matrix = transposed;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![3, 1],
            vec![4, 2],
        ]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![
            vec![1],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![1],
        ]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![5, 3, 1],
            vec![6, 4, 2],
        ]);
    }
}
