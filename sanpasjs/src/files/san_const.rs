pub fn sanjaiyan_html_content(san_project_name: &String) -> String {
    format!(
        r###"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>{san_project_name}</title>
        <script src="./index.js" defer></script>
    </head>
    <body>
        
    </body>
    </html>"###
    )
}

pub const SANJAIYAN_PASCAL_CONTENT: &[u8; 122] = br###"program SanPasJs(input, output);
var name: string;
begin
  name := "SanPasJs";
  writeln("Future of web is ", name);
end;
"###;
