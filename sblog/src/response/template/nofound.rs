pub const TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>404 No Found</title>
  </head>
  <body
      style="
        width: 100vw;
        height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-direction: column;
        font-size: 1.5em;
        background-color: #16161a;
        color: #aeaeae;
        overflow: hidden;
      "
    >
      <span style="position:relative; bottom: -50px;">Oops! Page not found</span>
      <div style="font-size: 15em">
        <div><span>4</span><span>0</span><span>4</span></div>
      </div>
      <span style="position: relative; top: -50px;">we are sorry, but the page you requested was not found.</span>
  </body>
</html>
"#;
