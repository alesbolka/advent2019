fn main() {
  task2();
}

fn runIntCode(mut input: Vec<u32>) -> u32 {
  let mut ii = 0;
  loop {
      if ii >= input.len() {
          panic!("No exit code found in intCode");
      }

      match input[ii] {
          1  => {
              let a   = input[ii+1] as usize;
              let b   = input[ii+2] as usize;
              let out = input[ii+3] as usize;
              input[out] = input[a] + input[b];
          },
          2  => {
              let a   = input[ii+1] as usize;
              let b   = input[ii+2] as usize;
              let out = input[ii+3] as usize;
              input[out] = input[a] * input[b];
          },
          99 => break input[0],
          _ => panic!("Invalid opcode {}", input[ii])
      }
      ii += 4;
  }
}

fn task2() {
  let input: Vec<u32> = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,6,19,1,19,5,23,2,13,23,27,1,10,27,31,2,6,31,35,1,9,35,39,2,10,39,43,1,43,9,47,1,47,9,51,2,10,51,55,1,55,9,59,1,59,5,63,1,63,6,67,2,6,67,71,2,10,71,75,1,75,5,79,1,9,79,83,2,83,10,87,1,87,6,91,1,13,91,95,2,10,95,99,1,99,6,103,2,13,103,107,1,107,2,111,1,111,9,0,99,2,14,0,0];
  let expected: u32 = 19690720;
  let mut result: u32 = 0;


  'outer: for ii in (0..99) {
      for jj in (0..99) {
          let mut clone = input.clone();
          clone[1] = ii;
          clone[2] = jj;
          let out = runIntCode(clone);
          if out == expected {
              result = 100 * ii + jj;
              break 'outer;
          }
      }
  };

  println!("{}", result);
}
