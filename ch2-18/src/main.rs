fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house, and dark square is a picture
feverishly turned--in search of what? 
It is the same with books.
What do you we seek through millions of pages?";

    let mut lines: Vec<String> = Vec::new();
    let mut line_match_num: usize = 1;
    let mut ctx_range = 0..0;

    for (num, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            line_match_num = num;
            //let s = num - 2;
            let n = num + 2;
    
            ctx_range = num..n;
            break;
        }
    }

    for (num, line) in quote.lines().enumerate() {
        if ctx_range.contains(&num) {
            let formatted_line = format!("{}: {}", num, line);

           lines.push(formatted_line);
        }
    }

    for line in quote.lines() {
        println!("{}", line);
    }


}
