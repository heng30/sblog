pub const TEMPLATE: &str = r#"
<!doctype html>
<html lang="zh-CN">
  <head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8" />
    <meta
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=0"
      name="viewport"
    />
    <meta name="description" content="这个博客关于编程，写作和英语学习" />
    <meta name="keyword" content="编程，写作，英语学习" />
    <meta property="og:site_name" content="$${{site-name}}" />
    <meta property="og:type" content="website" />

    <link rel="shortcut icon" href="$${{site-logo-tab}}" />
    <title>$${{post-title}} | $${{site-name}}</title>

    <style>
      body {
        width: 100%;
        height: 100%;
        max-width: 860px;
        margin: 0 auto;
        color: #aeaeae;
        background-color: #202020;
        background-image: radial-gradient(
          rgba(255, 255, 255, 0.171) 2px,
          transparent 0
        );
        background-size: 30px 30px;
        background-position: -5px -5px;

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
        padding: 1px 6px;
        border-radius: 2px;
      }

      img {
        display: block;
        width: 90%;
        margin: 1em auto;
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
        margin: 30px 10px;
        padding: 20px;
        background-color: #202020;
        border-radius: 8px;
        display: flex;
        flex-direction: column;
        box-shadow: 0 0 20px 10px rgba(0, 0, 0, 0.3);
        transition: all ease-in-out 0.3s;
      }

      @media screen and (max-width: 480px) {
        .content-container {
          margin: 0;
        }
      }

      .content-container:hover {
        box-shadow: 0 0 20px 10px rgba(0, 0, 0, 0.5);
      }
    </style>
  </head>

  <body>
    <div class="content-container">$${{header}} $${{body}}</div>
  </body>
</html>
"#;
