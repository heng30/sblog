pub const TEMPLATE: &str = r#"
<!doctype html>
<html lang="zh-CN">
  <head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8" />
    <meta
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=0"
      name="viewport"
    />
    <meta name="description" content="All about coding and writing" />
    <meta
      name="keyword"
      content="Programming, Programming Languages, Algorithms, Tools"
    />
    <meta property="og:site_name" content="$${{site-name}}" />
    <meta property="og:type" content="website" />

    <link rel="shortcut icon" href="$${{site-logo-tab}}" />
    <link
      href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.8.0/styles/github-dark.min.css"
      rel="stylesheet"
    />

    <title>$${{post-title}} | $${{site-name}}</title>

    <style>
      body {
        background-color: #16161a;
        color: #aeaeae;
        width: 100%;
        max-width: 860px;
        margin: 0 auto;
        scrollbar-width: thin;
        scrollbar-color: #aeaeae #161616;
      }

      a {
        text-decoration: none;
        color: #aeaeae;
        cursor: pointer;
      }

      a:hover {
        border-bottom: 2px solid #eeeeee;
        color: #eeeeee;
      }

      code {
        background: #161616;
        padding: 2px 6px;
        border-radius: 2px;
      }

      ::-webkit-scrollbar {
        width: 8px;
        height: 8px;
      }

      ::-webkit-scrollbar-track {
        background: #161616;
        border-radius: 2px;
      }

      ::-webkit-scrollbar-thumb {
        background: #aeaeae;
        border-radius: 2px;
      }

      ::-webkit-scrollbar-thumb:hover {
        background: #eeeeee;
      }

      ::-webkit-scrollbar-corner {
        background: #aeaeae;
      }

      .content-container {
        margin: 30px 0px;
        padding: 20px;
        background-color: #202020;
        border-radius: 8px;
        display: flex;
        flex-direction: column;
        box-shadow: 0 0 20px 10px rgba(0, 0, 0, 0.5);
      }
    </style>
  </head>

  <body>
    <div class="content-container">$${{header}} $${{body}}</div>
  </body>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.8.0/highlight.min.js"></script>
  <script>
    hljs.highlightAll();
  </script>
</html>
"#;
