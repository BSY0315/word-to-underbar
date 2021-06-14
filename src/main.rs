use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

fn get_firstword(line: &str) -> String {
    let bytes = line.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\t' {
            return String::from(&line[0..i]);
        }
    }
    String::from(&line[..])
}

fn convert_undersore(line: &mut String) {
    let word = get_firstword(&line[..]);
    let is_contain_y = match word.chars().last().unwrap() {
        'y' => true,
        _ => false,
    };

    let count = word.len();
    let y_word = format!("{}{}", &word[0..count - 1], 'i');

    let mut underbar = String::new();
    for _i in 0..count {
        underbar.push('_');
    }

    for i in count..line.len() - count {
        if word == &line[i..i + count] {
            line.replace_range(i..i + count, &underbar[..]);
        } else if is_contain_y {
            if y_word == &line[i..i + count] {
                line.replace_range(i..i + count, &underbar[..]);
            }
        }
    }
}

fn main() {
    let filename = "word master.txt";
    println!("In file \"{}\"", filename);

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(filename)
        .expect("failed to open file");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file");

    let mut contents: Vec<String> = vec![];
    for item in content.split_terminator('\n') {
        contents.push(format!("{}{}", item, '\n').to_lowercase());
    }

    file.write_all(b"\n=======================converted========================\n")
        .expect("failed to write");
    for mut item in contents {
        /* ???? 공백 에러가 가끔씩 나옴 뭐지 */
        /* 유니코드 \u{a0} 공백문자가 삽입되서 chars boundary 에러가 나옴*/
        item = item.replace(' ', " ");
        convert_undersore(&mut item);
        file.write(item.as_bytes()).expect("failed to write");
    }
    println!("completely convert word to underbar!");
}
