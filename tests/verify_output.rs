use std::fs;

fn main() {
    let content = "Hello World\nLine 2";
    let tagged = hashfile_mcp::hashline::tag_content(content);
    let hash = hashfile_mcp::hashline::compute_file_hash(content);
    let total_lines = content.lines().count();
    
    let output = format!(
        "[Metadata: total_lines={}, file_hash={}]\n{}",
        total_lines, hash, tagged
    );
    
    println!("{}", output);
}
