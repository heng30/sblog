pub const TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="cn">
  <style>
    @font-face {
      font-family: 'Merriweather';
      font-style: normal;
      font-weight: 400;
      src: local('Merriweather'), local('Merriweather-Regular');
    }
    @font-face {
      font-family: 'Lato';
      font-style: normal;
      font-weight: 400;
      src: local('Lato Regular'), local('Lato-Regular');
    }
    body {
      font-family: 'Merriweather', serif;
    }
    h1,
    h2,
    h3,
    h4,
    h5,
    h6,
    .nav,
    .article-duration,
    .archive-item-link,
    .footer {
      font-family: 'Lato', sans-serif;
    }
    ::selection {
      background: #209460;
      color: #fff;
    }
    html {
      font-size: 62.5%;
    }
    body {
      font-family: 'Merriweather', serif;
      font-size: 1.7em;
      position: relative;
      line-height: 1.6;
      font-weight: 400;
      color: #222;
      text-align: justify;
      word-break: break-word;
      hyphens: auto;
      margin: 0 16px;
    }
    @media screen and (max-width: 480px) {
      body {
        font-size: 15px;
      }
    }
    @media screen and (max-width: 1240px) {
      body {
        font-size: 15px;
      }
    }
    p {
      margin: 0.5;
    }
    ol,
    ul,
    form {
      margin: 0;
    }
    a {
      text-decoration: none;
      color: #209460;
      cursor: pointer;
    }
    p {
      word-spacing: 0.05em;
      color: #000;
    }
    pre {
      overflow-x: auto;
    }
  </style>

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
    <link rel="shortcut icon" href="$${{site-logo-tab}}">
    <title>$${{post-title}} | $${{site-name}}</title>
  </head>
  <body>
    $${{header}}
    $${{body}}
  </body>

  <script>
    var prevScrollpos = window.pageYOffset;
    window.onscroll = function () {
      var currentScrollPos = window.pageYOffset;
      if (prevScrollpos > currentScrollPos) {
          document.getElementById('fixed-header').style.position = 'fixed';
      } else {
          document.getElementById('fixed-header').style.position = 'absolute';
      }
      prevScrollpos = currentScrollPos;
    };
  </script>
</html>
"#;
